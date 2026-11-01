use std::io::{BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, Command, Stdio};
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
    expect_that!(uci.recv(), ok(contains_substring("id name FChess")));
    expect_that!(uci.recv(), ok(contains_substring("id author joaj")));
    expect_that!(uci.recv(), ok(eq("uciok")));
}

#[gtest]
fn test_uci_initial_position_fen() {
    let mut uci = UciClient::spawn();
    uci.send("uci");
    uci.recvn(3).unwrap();

    uci.send("position startpos");
    uci.send("isready");
    expect_that!(uci.recv(), ok(eq("readyok")));

    uci.send("fen");
    expect_that!(
        uci.recv(),
        ok(eq(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
        ))
    );
}
