use std::collections::HashMap;
use std::io;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use anyhow::Result;
use fchess::{
    Board, InfoSubCmdBuilder, Move, OptionType, PositionSubCmd, Solver, UciEngineCmd, UciGuiCmd,
};
use log::error;

fn to_client(output: &str) {
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

#[allow(dead_code)]
#[derive(Clone)]
enum Value {
    Check(bool),
    Spin(i64),
    String(String),
    Button,
    Combo(String),
}

impl Value {
    fn check(&self) -> Option<bool> {
        if let Value::Check(value) = self {
            Some(*value)
        } else {
            None
        }
    }

    fn spin(&self) -> Option<i64> {
        if let Value::Spin(value) = self {
            Some(*value)
        } else {
            None
        }
    }

    fn string(&self) -> Option<String> {
        if let Value::String(value) = self {
            Some(value.clone())
        } else {
            None
        }
    }

    fn combo(&self) -> Option<String> {
        if let Value::Combo(value) = self {
            Some(value.clone())
        } else {
            None
        }
    }
}

#[allow(dead_code)]
#[derive(Clone)]
enum Kind {
    Check,
    Spin { min: Option<i64>, max: Option<i64> },
    Combo(Vec<String>),
    Button,
    String,
}

#[derive(Clone)]
struct Opt {
    pub name: &'static str,
    pub kind: Kind,
    pub value: Value,
    pub default: Value,
}

impl Opt {
    pub fn new(name: &'static str, kind: Kind, value: Value) -> Self {
        Self {
            name,
            kind,
            value: value.clone(),
            default: value,
        }
    }
}

struct Registry {
    opts: Vec<Opt>,
    extra: HashMap<String, Option<String>>,
}

impl Registry {
    pub fn new() -> Self {
        let mut opts: Vec<Opt> = Vec::new();
        let extra: HashMap<String, Option<String>> = HashMap::default();

        opts.push(Opt::new(
            "OpeningBookFile",
            Kind::String,
            Value::String("".to_string()),
        ));

        Self { opts, extra }
    }

    fn option_definitions(&self) -> Vec<OptionType> {
        self.opts
            .iter()
            .map(|opt| match opt.kind {
                Kind::Check => OptionType::Check {
                    name: opt.name.to_string(),
                    default: opt.default.check(),
                },
                Kind::Spin { min, max } => OptionType::Spin {
                    name: opt.name.to_string(),
                    default: opt.default.spin().unwrap(),
                    min,
                    max,
                },
                Kind::Combo(ref vars) => OptionType::Combo {
                    name: opt.name.to_string(),
                    default: opt.default.combo().unwrap(),
                    vars: vars.clone(),
                },
                Kind::Button => OptionType::Button(opt.name.to_string()),
                Kind::String => OptionType::String {
                    name: opt.name.to_string(),
                    default: Some(opt.default.string().unwrap_or("<empty>".to_string())),
                },
            })
            .collect()
    }

    pub fn set(&mut self, name: &String, value: &Option<String>) {
        for opt in self.opts.iter_mut() {
            if opt.name == name {
                match opt.kind {
                    Kind::Check => {
                        if let Some(value) = value {
                            opt.value = Value::Check(value.to_lowercase() == "true");
                        } else {
                            error!("No value provided for option of type Check {name}")
                        }
                    }
                    Kind::Spin { min, max } => {
                        if let Some(value) = value {
                            let v = value.parse::<i64>();
                            if let Ok(v) = v {
                                let min = min.unwrap_or(i64::MIN);
                                let max = max.unwrap_or(i64::MAX);
                                if v > min && v < max {
                                    opt.value = Value::Spin(v);
                                } else {
                                    error!(
                                        "Value of type Spin is out of bounds. {name} must be in [{min}, {max}]. Attempting to set it to {v}"
                                    )
                                }
                            } else {
                                error!(
                                    "Value of type Spin cannot be interpreted as an integer {name}"
                                );
                            }
                        } else {
                            error!("No value provided for option of type Check {name}")
                        }
                    }
                    Kind::String => {
                        if let Some(value) = value {
                            opt.value = Value::String(value.clone());
                        } else {
                            error!("No value provided for option of type String {name}")
                        }
                    }
                    Kind::Button => {
                        unimplemented!();
                    }
                    Kind::Combo(_) => {
                        if let Some(value) = value {
                            opt.value = Value::Combo(value.clone());
                        } else {
                            error!("No value provided for option of type String {name}")
                        }
                    }
                }
                return;
            }
        }

        self.extra.insert(name.to_string(), value.clone());
    }
}

fn main() -> Result<()> {
    env_logger::init();

    let mut registry = Registry::new();

    let (gui_tx, engine_rx): (Sender<GuiCmd>, Receiver<GuiCmd>) = mpsc::channel();
    let (engine_tx, gui_rx): (Sender<EngineCmd>, Receiver<EngineCmd>) = mpsc::channel();

    let t = thread::spawn(move || {
        loop {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).unwrap();
            let buffer = buffer.replace('\n', "");

            let commands = match dbg!(UciGuiCmd::parse(dbg!(&buffer))).unwrap() {
                UciGuiCmd::Uci => {
                    let mut cmds = vec![
                        UciEngineCmd::IdName("FChess".to_string()),
                        UciEngineCmd::IdAuthor("João Freitas".to_string()),
                    ];

                    for option in registry.option_definitions() {
                        cmds.push(UciEngineCmd::Option(option));
                    }

                    cmds.push(UciEngineCmd::UciOk);
                    cmds
                }
                UciGuiCmd::UciNewGame => vec![],
                UciGuiCmd::IsReady => vec![UciEngineCmd::ReadyOk],
                UciGuiCmd::Quit => std::process::exit(0),
                UciGuiCmd::Stop => vec![],
                UciGuiCmd::Position { pos, moves } => {
                    gui_tx.send(GuiCmd::Position { pos, moves }).unwrap();
                    vec![]
                }
                UciGuiCmd::Go(_) => {
                    gui_tx.send(GuiCmd::Go).unwrap();
                    if let EngineCmd::BestMove(mov) = gui_rx.recv().unwrap() {
                        vec![UciEngineCmd::BestMove { mov, ponder: None }]
                    } else {
                        panic!()
                    }
                }
                UciGuiCmd::SetOption { name, value } => {
                    registry.set(&name, &value);
                    vec![]
                }
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
                to_client(dbg!(&command.write()));
            }
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
