use std::fs::File;
use std::io;
use std::io::prelude::*;

use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use anyhow::Result;
use fchess::{
    Board, InfoSubCmdBuilder, Move, OptionTypes, PositionSubCmd, Solver, UciEngineCmd, UciGuiCmd,
    write_engine_command,
};

fn to_client(file: &mut File, output: &str) {
    file.write_all(format!("engine - {}\n", output).as_bytes())
        .unwrap();
    println!("{}", output);
}

enum GuiCmd {
    Fen,
    Position {
        pos: PositionSubCmd,
        moves: Vec<Move>,
    },
    Go,
}

enum EngineCmd {
    Fen(String),
    BestMove(Move),
}

fn main() -> Result<()> {
    let (gui_tx, engine_rx): (Sender<GuiCmd>, Receiver<GuiCmd>) = mpsc::channel();
    let (engine_tx, gui_rx): (Sender<EngineCmd>, Receiver<EngineCmd>) = mpsc::channel();

    let t = thread::spawn(move || {
        let mut file = File::create("/home/joaj/sources/fchess/log1.txt").unwrap();
        loop {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).unwrap();
            let buffer = buffer.replace('\n', "");

            let commands = match dbg!(UciGuiCmd::parse(dbg!(&buffer))).unwrap() {
                UciGuiCmd::Uci => vec![
                    UciEngineCmd::IdName("FChess".to_string()),
                    UciEngineCmd::IdAuthor("João Freitas".to_string()),
                    UciEngineCmd::Option(OptionTypes::String {
                        name: "Opening book".to_string(),
                        default: None,
                    }),
                    UciEngineCmd::UciOk,
                ],
                UciGuiCmd::UciNewGame => vec![],
                UciGuiCmd::IsReady => vec![UciEngineCmd::ReadyOk],
                UciGuiCmd::Quit => std::process::exit(0),
                UciGuiCmd::Stop => vec![],
                UciGuiCmd::Position { pos, moves } => {
                    gui_tx.send(GuiCmd::Position { pos, moves }).unwrap();
                    vec![]
                }
                UciGuiCmd::Go(go_sub_cmd) => {
                    gui_tx.send(GuiCmd::Go).unwrap();
                    if let EngineCmd::BestMove(mov) = gui_rx.recv().unwrap() {
                        vec![UciEngineCmd::BestMove {
                            mov: mov,
                            ponder: None,
                        }]
                    } else {
                        panic!()
                    }
                }
                UciGuiCmd::SetOption { name, value } => vec![],
                UciGuiCmd::Fen => {
                    gui_tx.send(GuiCmd::Fen).unwrap();
                    let cmd = gui_rx.recv().unwrap();
                    if let EngineCmd::Fen(fen) = cmd {
                        vec![UciEngineCmd::Info(
                            InfoSubCmdBuilder::new().with_string(&fen).build(),
                        )]
                    } else {
                        panic!()
                    }
                }
                _ => panic!(),
            };

            for command in commands {
                to_client(&mut file, &write_engine_command(&command));
            }

            file.write_all(format!("client - {buffer}\n").as_bytes())
                .unwrap();
        }
    });

    let _engine_thread = thread::spawn(move || {
        let mut board =
            Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq e3 0 1").unwrap();
        let mut solver = Solver::new();
        loop {
            let cmd = engine_rx.recv().unwrap();
            match cmd {
                GuiCmd::Fen => engine_tx
                    .send(EngineCmd::Fen(board.to_fen().unwrap()))
                    .unwrap(),
                GuiCmd::Position { pos, moves } => {
                    match pos {
                        PositionSubCmd::StartPos => {
                            board = Board::from_basic_board();
                        }
                        PositionSubCmd::Fen(fen) => {
                            board = Board::from_fen(&fen).unwrap();
                        }
                    };

                    for mov in moves {
                        board = board.apply(&mov).unwrap();
                    }
                }
                GuiCmd::Go => {
                    engine_tx
                        .send(EngineCmd::BestMove(solver.best_move(&board).unwrap()))
                        .unwrap();
                }
            };
        }
    });

    t.join().unwrap();
    _engine_thread.join().unwrap();
    Ok(())
}
