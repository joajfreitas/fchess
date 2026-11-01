use std::fs::File;
use std::io;
use std::io::prelude::*;

use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use anyhow::Result;
use fchess::{Board, Move, Solver};

fn to_client(file: &mut File, output: &str) {
    file.write_all(format!("engine - {}\n", output).as_bytes())
        .unwrap();
    println!("{}", output);
}

fn main() -> Result<()> {
    let (gui_tx, engine_rx): (Sender<String>, Receiver<String>) = mpsc::channel();
    let (engine_tx, _): (Sender<String>, Receiver<String>) = mpsc::channel();

    let t = thread::spawn(move || {
        let mut file = File::create("/home/joaj/sources/fchess/log1.txt").unwrap();
        loop {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).unwrap();
            let buffer = buffer.replace('\n', "");
            file.write_all(format!("client - {buffer}\n").as_bytes())
                .unwrap();

            match &buffer[..] {
                "uci" => {
                    to_client(&mut file, "id name FChess");
                    to_client(&mut file, "id author joaj");
                    to_client(&mut file, "uciok");
                    to_client(&mut file, "option");
                }
                "ucinewgame" => {}
                "isready" => {
                    to_client(&mut file, "readyok");
                }
                "quit" => {
                    std::process::exit(0);
                }
                "stop" => {}
                _ => {
                    if buffer.starts_with("position") {
                        let sp = &buffer.split(' ').collect::<Vec<&str>>()[1..];
                        let mut moves = false;
                        for s in sp.iter() {
                            if moves {
                                gui_tx.send(format!("move:{s}")).unwrap();
                            }
                            if *s == "startpos" {
                                gui_tx.send("startpos".to_string()).unwrap();
                            }
                            if *s == "moves" {
                                moves = true;
                            }
                        }
                        //let j = sp.collect::<Vec<&str>>()[1..3].join(" ");
                        //file.write_all(j.as_bytes());

                        //file.write_all(b"starting pos\n");
                    }
                    if buffer.starts_with("go") {
                        gui_tx.send("go".to_string()).unwrap();
                    }
                    if buffer.starts_with("setoption") {
                        continue;
                    }
                }
            }
        }
    });

    let _engine_thread = thread::spawn(move || {
        let mut board =
            Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq e3 0 1").unwrap();
        let mut solver = Solver::new();
        let mut file = File::create("/home/joaj/sources/fchess/log2.txt").unwrap();
        loop {
            let cmd = engine_rx.recv().unwrap();
            if cmd == "startpos" {
                board = Board::from_basic_board();
            } else if cmd.starts_with("move") {
                //to_client(&mut file, "cmd: {cmd}");
                let sp = cmd.split(':');
                let mov = sp.collect::<Vec<&str>>()[1];
                board = board
                    .apply(&Move::from_full_algebraic(mov).unwrap())
                    .unwrap();
                to_client(&mut file, &format!("board {board:?}"));
            } else if cmd.starts_with("go") {
                let info = "info_currmove 1";
                to_client(&mut file, info);
                file.write_all(format!("start: {board:?}\n").as_bytes())
                    .unwrap();
                to_client(
                    &mut file,
                    &format!(
                        "bestmove {}",
                        solver.best_move(&board).unwrap().to_algebraic()
                    ),
                );
            } else if cmd == "fen" {
                engine_tx.send(board.to_fen().unwrap()).unwrap();
            }
        }
    });

    t.join().unwrap();
    _engine_thread.join().unwrap();
    Ok(())
}
