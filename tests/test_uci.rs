use std::io::{BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::result::Result;
use std::sync::mpsc::{self, RecvTimeoutError};
use std::time::{Duration, Instant};

use googletest::prelude::*;

struct UciClient {
    stdin: ChildStdin,
    lines: mpsc::Receiver<String>,
    deadline: Instant,
    child: Child,
}

impl UciClient {
    pub fn spawn() -> UciClient {
        let mut child = Command::new(env!("CARGO_BIN_EXE_uci"))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to spawn uci binary");

        let stdout = child.stdout.take().unwrap();
        let (tx, rx) = mpsc::channel();

        std::thread::spawn(move || {
            for line in BufReader::new(stdout).lines() {
                match line {
                    Ok(l) => {
                        if tx.send(l).is_err() {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
        });

        UciClient {
            stdin: child.stdin.take().unwrap(),
            lines: rx,
            deadline: Instant::now() + Duration::from_secs(5),
            child,
        }
    }

    #[allow(dead_code)]
    fn with_deadline(mut self, t: Duration) -> Self {
        self.deadline = Instant::now() + t;
        self
    }

    pub fn send(&mut self, cmd: &str) {
        writeln!(self.stdin, "{}", cmd).unwrap();
    }

    pub fn recv(&mut self) -> Result<String, String> {
        let remaining = self.deadline.saturating_duration_since(Instant::now());
        if remaining.is_zero() {
            let _ = self.child.kill();
            return Err("timed out".to_string());
        }

        match self.lines.recv_timeout(remaining) {
            Ok(line) => Ok(line),
            Err(RecvTimeoutError::Timeout) => {
                let _ = self.child.kill();
                Err("engine closed stdout".to_string())
            }
            Err(RecvTimeoutError::Disconnected) => Err("channel disconnected".to_string()),
        }
    }

    pub fn recvn(&mut self, n: usize) -> Result<String, String> {
        let mut buffer: String = String::new();
        for _ in 0..n {
            buffer.push_str(&self.recv()?);
        }

        Ok(buffer)
    }
}

impl Drop for UciClient {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

#[gtest]
fn test_uci_handshake() {
    let mut uci = UciClient::spawn();
    uci.send("uci");
    expect_that!(uci.recv(), ok(eq("id name FChess")));
    expect_that!(uci.recv(), ok(eq("id author João Freitas")));
    expect_that!(
        uci.recv(),
        ok(eq("option name OpeningBookFile type string default "))
    );
    expect_that!(uci.recv(), ok(eq("uciok")));
}

#[gtest]
fn test_uci_unknown_command() {
    let mut uci = UciClient::spawn();
    uci.send("some command");
    expect_that!(uci.recv(), ok(eq("Unknown command: 'some command'.")));
}

#[gtest]
fn test_uci_initial_position_fen() {
    let mut uci = UciClient::spawn();
    uci.send("uci");
    uci.recvn(4).unwrap();

    uci.send("position startpos");
    uci.send("isready");
    expect_that!(uci.recv(), ok(eq("readyok")));

    uci.send("fen");
    expect_that!(
        uci.recv(),
        ok(eq(
            "info string rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
        ))
    );
}

#[gtest]
fn test_uci_position_fen() {
    let mut uci = UciClient::spawn();
    uci.send("uci");
    uci.recvn(4).unwrap();

    uci.send("position fen rnbqkbn1/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    uci.send("isready");
    expect_that!(uci.recv(), ok(eq("readyok")));

    uci.send("fen");
    expect_that!(
        uci.recv(),
        ok(eq(
            "info string rnbqkbn1/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
        ))
    );
}

#[gtest]
fn test_uci_go() {
    let mut uci = UciClient::spawn();
    uci.send("uci");
    uci.recvn(4).unwrap();

    uci.send("position startpos");
    uci.send("isready");
    expect_that!(uci.recv(), ok(eq("readyok")));

    uci.send("go");
    expect_that!(uci.recv(), ok(eq("bestmove e2e4")));
}

#[gtest]
fn test_uci_non_standard_go_perft() {
    let mut uci = UciClient::spawn();
    uci.send("uci");
    uci.recvn(4).unwrap();

    uci.send("position startpos");
    uci.send("isready");
    expect_that!(uci.recv(), ok(eq("readyok")));

    uci.send("go perft 1");
    //expect_that!(uci.recv(), ok(eq("bestmove g1h3")));
    uci.recvn(20).unwrap();
    expect_that!(uci.recv(), ok(eq("Nodes searched: 20")));
}

#[gtest]
fn test_uci_non_standard_display() {
    let mut uci = UciClient::spawn();
    uci.send("uci");
    uci.recvn(4).unwrap();
    uci.send("position startpos");
    uci.send("display");
    uci.recv().unwrap();
    expect_that!(uci.recv(), ok(eq("8 │ r │ k │ b │ q │ k │ b │ k │ r │")));
    uci.recv().unwrap();
    expect_that!(uci.recv(), ok(eq("7 │ p │ p │ p │ p │ p │ p │ p │ p │")));
    uci.recvn(11).unwrap();
    expect_that!(uci.recv(), ok(eq("1 │ R │ K │ B │ Q │ K │ B │ K │ R │")));
    uci.recvn(3).unwrap();
}
