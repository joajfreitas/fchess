use std::fs::File;
use std::io;
use std::io::prelude::*;

use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use fchess::moves::{algebraic, Board, Scope};

fn main() -> io::Result<()> {
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();

    let t = thread::spawn(move || {
        let mut file = File::create("log.txt").unwrap();

        loop {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).unwrap();
            let buffer = buffer.replace('\n', "");
            file.write_all(format!("{}\n", buffer).as_bytes()).unwrap();

            match &buffer[..] {
                "uci" => {
                    println!("id name FChess");
                    println!("id author joajfreitas");
                    println!("option");
                    println!("uciok");
                }
                "isready" => {
                    println!("readyok");
                }
                "quit" => {
                    std::process::exit(0);
                }
                "stop" => {}
                _ => {
                    if buffer.starts_with("position") {
                        let sp = &buffer.split(' ').collect::<Vec<&str>>()[1..];
                        println!("{:?}", sp);
                        let mut moves = false;
                        for s in sp.iter() {
                            if moves {
                                tx.send(format!("move:{}", s)).unwrap();
                            }
                            if *s == "startpos" {
                                tx.send("startpos".to_string()).unwrap();
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
                        tx.send("go".to_string()).unwrap();
                    }
                }
            }
        }
    });

    let engine_thread = thread::spawn(move || {
        let mut board = Board::read_fen(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq e3 0 1".to_string(),
        );
        loop {
            let cmd = rx.recv().unwrap();
            if cmd == "startpos" {
                board = Board::read_fen(
                    "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq e3 0 1".to_string(),
                );
            } else if cmd.starts_with("move") {
                println!("cmd: {}", cmd);
                let sp = cmd.split(':');
                let mov = sp.collect::<Vec<&str>>()[1];
                board = board.apply_algebraic_notation(mov.to_string()).unwrap();
                println!("board {:?}", board);
            } else if cmd.starts_with("go") {
                let info = "info_currmove 1";
                println!("{}", info);
                println!("start: {:?}", board);
                println!(
                    "bestmove {}",
                    algebraic(board.best_move(Scope::White).unwrap())
                );
            }
        }
    });

    t.join().unwrap();
    engine_thread.join().unwrap();
    Ok(())
}
