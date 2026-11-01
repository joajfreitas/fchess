use crate::Move;
use lazy_regex::regex;

#[derive(Clone, PartialEq, Debug, Default)]
struct GoSubCmd {
    search_moves: Vec<Move>,
    ponder: bool,
    wtime: Option<u32>,
    btime: Option<u32>,
    winc: Option<u32>,
    binc: Option<u32>,
    movestogo: Option<u32>,
    depth: Option<u32>,
    nodes: Option<u32>,
    mate: Option<u32>,
    infinite: Option<bool>,
    perft: Option<u32>,
}

#[allow(dead_code)]
#[derive(Clone)]
struct GoSubCmdBuilder {
    go_sub_cmd: GoSubCmd,
}

#[allow(dead_code)]
impl GoSubCmdBuilder {
    pub fn new() -> Self {
        Self {
            go_sub_cmd: GoSubCmd::default(),
        }
    }

    pub fn with_search_moves(&self, search_moves: &[Move]) -> GoSubCmdBuilder {
        let mut this = self.clone();
        this.go_sub_cmd.search_moves = search_moves.to_vec();
        this
    }

    pub fn with_ponder(&self, ponder: bool) -> GoSubCmdBuilder {
        let mut this = self.clone();
        this.go_sub_cmd.ponder = ponder;
        this
    }

    pub fn with_wtime(&self, wtime: u32) -> GoSubCmdBuilder {
        let mut this = self.clone();
        this.go_sub_cmd.wtime = Some(wtime);
        this
    }

    pub fn with_btime(&self, btime: u32) -> GoSubCmdBuilder {
        let mut this = self.clone();
        this.go_sub_cmd.btime = Some(btime);
        this
    }

    pub fn with_winc(&self, winc: u32) -> GoSubCmdBuilder {
        let mut this = self.clone();
        this.go_sub_cmd.winc = Some(winc);
        this
    }

    pub fn with_binc(&self, binc: u32) -> GoSubCmdBuilder {
        let mut this = self.clone();
        this.go_sub_cmd.binc = Some(binc);
        this
    }

    pub fn with_movestogo(&self, movestogo: u32) -> GoSubCmdBuilder {
        let mut this = self.clone();
        this.go_sub_cmd.movestogo = Some(movestogo);
        this
    }

    pub fn with_depth(&self, depth: u32) -> GoSubCmdBuilder {
        let mut this = self.clone();
        this.go_sub_cmd.depth = Some(depth);
        this
    }

    pub fn with_nodes(&self, nodes: u32) -> GoSubCmdBuilder {
        let mut this = self.clone();
        this.go_sub_cmd.nodes = Some(nodes);
        this
    }

    pub fn with_mate(&self, mate: u32) -> GoSubCmdBuilder {
        let mut this = self.clone();
        this.go_sub_cmd.mate = Some(mate);
        this
    }

    pub fn with_infinite(&self, infinite: bool) -> GoSubCmdBuilder {
        let mut this = self.clone();
        this.go_sub_cmd.infinite = Some(infinite);
        this
    }

    pub fn with_perft(&self, perft: u32) -> GoSubCmdBuilder {
        let mut this = self.clone();
        this.go_sub_cmd.perft = Some(perft);
        this
    }

    fn build(&self) -> GoSubCmd {
        self.go_sub_cmd.clone()
    }
}

#[derive(PartialEq, Debug)]
enum PositionSubCmd {
    StartPos,
    Fen(String),
}

#[allow(dead_code)]
#[derive(PartialEq, Debug)]
enum UciGuiCmd {
    Uci,
    Debug(Option<bool>),
    IsReady,
    SetOption {
        name: String,
        value: Option<String>,
    },
    UciNewGame,
    Go(GoSubCmd),
    PonderHit,
    Position {
        pos: PositionSubCmd,
        moves: Vec<Move>,
    },
    Quit,
    Register {
        later: bool,
        name: String,
        code: String,
    },
    Stop,
}

#[allow(dead_code)]
#[derive(PartialEq, Debug)]
enum IdSubCmd {
    Name(String),
    Author(String),
}

#[derive(PartialEq, Debug, Clone, Default)]
struct ScoreSubCmd {
    cp: Option<u32>,
    mate: Option<u32>,
    upperbound_cp: Option<u32>,
    upperbound_mate: Option<u32>,
    lowerbound_cp: Option<u32>,
    lowerbound_mate: Option<u32>,
}

#[derive(PartialEq, Debug, Clone)]
struct ScoreSubCmdBuilder {
    score_sub_cmd: ScoreSubCmd,
}

#[allow(dead_code)]
impl ScoreSubCmdBuilder {
    pub fn new() -> Self {
        Self {
            score_sub_cmd: ScoreSubCmd::default(),
        }
    }

    pub fn with_cp(&self, cp: u32) -> Self {
        let mut this = self.clone();
        this.score_sub_cmd.cp = Some(cp);
        this
    }

    pub fn with_mate(&self, mate: u32) -> Self {
        let mut this = self.clone();
        this.score_sub_cmd.mate = Some(mate);
        this
    }

    pub fn with_upperbound_cp(&self, upperbound_cp: u32) -> Self {
        let mut this = self.clone();
        this.score_sub_cmd.upperbound_cp = Some(upperbound_cp);
        this
    }

    pub fn with_upperbound_mate(&self, upperbound_mate: u32) -> Self {
        let mut this = self.clone();
        this.score_sub_cmd.upperbound_mate = Some(upperbound_mate);
        this
    }

    pub fn with_lowerbound_cp(&self, lowerbound_cp: u32) -> Self {
        let mut this = self.clone();
        this.score_sub_cmd.lowerbound_cp = Some(lowerbound_cp);
        this
    }

    pub fn with_lowerbound_mate(&self, lowerbound_mate: u32) -> Self {
        let mut this = self.clone();
        this.score_sub_cmd.lowerbound_mate = Some(lowerbound_mate);
        this
    }

    pub fn build(&self) -> ScoreSubCmd {
        self.score_sub_cmd.clone()
    }
}

#[derive(PartialEq, Debug, Default, Clone)]
struct InfoSubCmd {
    depth: Option<u32>,
    seldepth: Option<u32>,
    time: Option<u32>,
    nodes: Option<u32>,
    pv: Option<Vec<Move>>,
    multipv: Option<u32>,
    score: Option<ScoreSubCmd>,
    currmove: Option<Move>,
    currmovenumber: Option<u32>,
    hashfull: Option<u32>,
    nps: Option<u32>,
    tbhits: Option<u32>,
    sbhits: Option<u32>,
    cpuload: Option<u32>,
    string: Option<String>,
    refutation: Option<Vec<Move>>,
    currline: Option<(Option<u32>, Vec<Move>)>,
}

#[derive(Clone)]
struct InfoSubCmdBuilder {
    info_sub_cmd: InfoSubCmd,
}

#[allow(dead_code)]
impl InfoSubCmdBuilder {
    pub fn new() -> Self {
        Self {
            info_sub_cmd: InfoSubCmd::default(),
        }
    }

    pub fn with_depth(&self, depth: u32) -> Self {
        let mut this = self.clone();
        this.info_sub_cmd.depth = Some(depth);
        this
    }

    pub fn with_seldepth(&self, seldepth: u32) -> Self {
        let mut this = self.clone();
        this.info_sub_cmd.seldepth = Some(seldepth);
        this
    }

    pub fn with_time(&self, time: u32) -> Self {
        let mut this = self.clone();
        this.info_sub_cmd.time = Some(time);
        this
    }

    pub fn with_nodes(&self, nodes: u32) -> Self {
        let mut this = self.clone();
        this.info_sub_cmd.nodes = Some(nodes);
        this
    }

    pub fn with_pv(&self, pv: &[Move]) -> Self {
        let mut this = self.clone();
        this.info_sub_cmd.pv = Some(pv.to_vec());
        this
    }

    pub fn with_multipv(&self, multipv: u32) -> Self {
        let mut this = self.clone();
        this.info_sub_cmd.multipv = Some(multipv);
        this
    }

    pub fn with_score(&self, score: &ScoreSubCmdBuilder) -> Self {
        let mut this = self.clone();
        this.info_sub_cmd.score = Some(score.build());
        this
    }

    pub fn with_currmove(&self, currmove: &Move) -> Self {
        let mut this = self.clone();
        this.info_sub_cmd.currmove = Some(currmove.clone());
        this
    }

    pub fn with_currmovenumber(&self, currmovenumber: u32) -> Self {
        let mut this = self.clone();
        this.info_sub_cmd.currmovenumber = Some(currmovenumber);
        this
    }

    pub fn with_hashfull(&self, hashfull: u32) -> Self {
        let mut this = self.clone();
        this.info_sub_cmd.hashfull = Some(hashfull);
        this
    }

    pub fn with_nps(&self, nps: u32) -> Self {
        let mut this = self.clone();
        this.info_sub_cmd.nps = Some(nps);
        this
    }

    pub fn with_tbhits(&self, tbhits: u32) -> Self {
        let mut this = self.clone();
        this.info_sub_cmd.tbhits = Some(tbhits);
        this
    }

    pub fn with_sbhits(&self, sbhits: u32) -> Self {
        let mut this = self.clone();
        this.info_sub_cmd.sbhits = Some(sbhits);
        this
    }

    pub fn with_cpuload(&self, cpuload: u32) -> Self {
        let mut this = self.clone();
        this.info_sub_cmd.cpuload = Some(cpuload);
        this
    }

    pub fn with_string(&self, string: &str) -> Self {
        let mut this = self.clone();
        this.info_sub_cmd.string = Some(string.to_string());
        this
    }

    pub fn with_refutation(&self, refutation: &[Move]) -> Self {
        let mut this = self.clone();
        this.info_sub_cmd.refutation = Some(refutation.to_vec());
        this
    }

    pub fn with_currline(&self, currline: (Option<u32>, Vec<Move>)) -> Self {
        let mut this = self.clone();
        this.info_sub_cmd.currline = Some(currline);
        this
    }

    pub fn build(&self) -> InfoSubCmd {
        self.info_sub_cmd.clone()
    }
}

#[allow(dead_code)]
#[derive(PartialEq, Debug)]
enum OptionTypes {
    Check {
        name: String,
        default: Option<bool>,
    },
    Spin {
        name: String,
        default: i32,
        min: Option<i32>,
        max: Option<i32>,
    },
    Combo {
        default: String,
        vars: Vec<String>,
    },
    Button(String),
    String {
        name: String,
        default: Option<String>,
    },
}

#[allow(dead_code)]
#[derive(PartialEq, Debug)]
enum UciEngineCmd {
    RegistrationChecking,
    CopyProtectionChecking,
    RegistrationOk,
    CopyProtectionError,
    BestMove { mov: Move, ponder: Option<Move> },
    Id(IdSubCmd),
    Info(InfoSubCmd),
    Option(OptionTypes),
    ReadyOk,
    UciOk,
}

#[allow(dead_code)]
fn parse_gui_command(command: &str) -> Result<UciGuiCmd, String> {
    let debug_re = regex!(r"^debug\s?((on)|(off))?$");
    let setoption_re = regex!(r"^setoption name (\w+)( value (\w+))?$");
    let go_re = regex!(
        r"^go(\ssearchmoves\s(?<moves>([a-h][1-8][a-h][1-8]\s?)+))?(?<ponder>\sponder)?(\swtime (?<wtime>\d+))?(\sbtime (?<btime>\d+))?(\swinc (?<winc>\d+))?(\sbinc (?<binc>\d+))?(\smovestogo (?<movestogo>\d+))?(\sdepth (?<depth>\d+))?(\snodes (?<nodes>\d+))?(\smate (?<mate>\d+))?$"
    );

    let position_re = regex!(
        r"^position\s((?<startpos>startpos)|(fen\s(?<fen>(?:(?:[pnbrqkPNBRQK]+|[1-8])\/){7}(?:[pnbrqkPNBRQK]+|[1-8])\s([wb])\s(-|[KQkq]{1,4})\s(-|[a-h][36])\s(\d+)\s(\d+))))(\smoves\s(?<moves>([a-h][1-8][a-h][1-8]\s)*([a-h][1-8][a-h][1-8])))?$"
    );

    if regex!("^uci$").is_match(command) {
        Ok(UciGuiCmd::Uci)
    } else if debug_re.is_match(command) {
        let caps = debug_re.captures(command).unwrap();
        match caps.get(1) {
            None => Ok(UciGuiCmd::Debug(None)),
            Some(arg) => match arg.as_str() {
                "on" => Ok(UciGuiCmd::Debug(Some(true))),
                "off" => Ok(UciGuiCmd::Debug(Some(false))),
                _ => unreachable!(),
            },
        }
    } else if regex!("^isready$").is_match(command) {
        Ok(UciGuiCmd::IsReady)
    } else if setoption_re.is_match(command) {
        let caps = setoption_re.captures(command).unwrap();
        let name = caps.get(1).map(|x| x.as_str().to_string()).unwrap();
        let value = caps.get(3).map(|x| x.as_str().to_string());
        Ok(UciGuiCmd::SetOption { name, value })
    } else if regex!("^ucinewgame$").is_match(command) {
        Ok(UciGuiCmd::UciNewGame)
    } else if go_re.is_match(command) {
        let caps = go_re.captures(command).unwrap();
        let search_moves = caps
            .name("moves")
            .and_then(|moves| {
                moves
                    .as_str()
                    .split(' ')
                    .map(Move::from_full_algebraic)
                    .collect::<Option<Vec<Move>>>()
            })
            .unwrap_or(Vec::new());

        let ponder = caps.name("ponder").is_some();

        let wtime = caps
            .name("wtime")
            .and_then(|wtime| wtime.as_str().to_string().parse::<u32>().ok());
        let btime = caps
            .name("btime")
            .and_then(|btime| btime.as_str().to_string().parse::<u32>().ok());
        let winc = caps
            .name("winc")
            .and_then(|winc| winc.as_str().to_string().parse::<u32>().ok());
        let binc = caps
            .name("winc")
            .and_then(|binc| binc.as_str().to_string().parse::<u32>().ok());
        let movestogo = caps
            .name("movestogo")
            .and_then(|movestogo| movestogo.as_str().to_string().parse::<u32>().ok());
        let depth = caps
            .name("depth")
            .and_then(|depth| depth.as_str().to_string().parse::<u32>().ok());
        let nodes = caps
            .name("nodes")
            .and_then(|nodes| nodes.as_str().to_string().parse::<u32>().ok());
        let mate = caps
            .name("mate")
            .and_then(|mate| mate.as_str().to_string().parse::<u32>().ok());

        Ok(UciGuiCmd::Go(GoSubCmd {
            search_moves,
            ponder,
            wtime,
            btime,
            winc,
            binc,
            movestogo,
            depth,
            nodes,
            mate,
            infinite: None,
            perft: None,
        }))
    } else if regex!("^ponderhit$").is_match(command) {
        Ok(UciGuiCmd::PonderHit)
    } else if position_re.is_match(command) {
        let caps = position_re.captures(command).unwrap();

        let position_sub_cmd = if caps.name("startpos").is_some() {
            PositionSubCmd::StartPos
        } else {
            PositionSubCmd::Fen(caps.name("fen").unwrap().as_str().to_string())
        };

        let moves = caps
            .name("moves")
            .and_then(|moves| {
                moves
                    .as_str()
                    .split(" ")
                    .map(Move::from_full_algebraic)
                    .collect::<Option<Vec<Move>>>()
            })
            .unwrap_or(Vec::new());

        Ok(UciGuiCmd::Position {
            pos: position_sub_cmd,
            moves,
        })
    } else if regex!("quit").is_match(command) {
        Ok(UciGuiCmd::Quit)
    } else if regex!("stop").is_match(command) {
        Ok(UciGuiCmd::Stop)
    } else {
        Err(String::from("Unimplemented"))
    }
}

#[allow(dead_code)]
fn write_engine_command(command: &UciEngineCmd) -> String {
    fn write_option<T: std::fmt::Display>(name: &str, value: Option<T>) -> String {
        if let Some(v) = value {
            format!(" {name} {v}")
        } else {
            "".to_string()
        }
    }
    match command {
        UciEngineCmd::BestMove { mov, ponder } => {
            format!("bestmove {}", mov.to_algebraic())
                + &(if let Some(ponder) = ponder {
                    format!(" ponder {}", ponder.to_algebraic())
                } else {
                    "".to_string()
                })
        }
        UciEngineCmd::Id(id) => match id {
            IdSubCmd::Name(name) => format!("id name {name}"),
            IdSubCmd::Author(author) => format!("id author {author}"),
        },
        UciEngineCmd::Info(info) => {
            "info".to_string() + &write_option("depth", info.depth) +
            &write_option("seldepth", info.seldepth) +
            &write_option("time", info.time) +
            &write_option("nodes", info.nodes) +
            &write_option("pv", info.pv.as_ref().map(|v| v.iter().map(|mov| mov.to_algebraic()).collect::<Vec<String>>().join(" "))) +
            &write_option("multipv", info.multipv) +
            &write_option("score cp", info.score.as_ref().and_then(|x| x.cp)) +
            &write_option("score mate", info.score.as_ref().and_then(|x| x.mate)) +
            &write_option("score upperbound cp", info.score.as_ref().and_then(|x| x.upperbound_cp)) +
            &write_option("score upperbound mate", info.score.as_ref().and_then(|x| x.upperbound_mate)) +
            &write_option("score lowerbound cp", info.score.as_ref().and_then(|x| x.lowerbound_cp)) +
            &write_option("score lowerbound mate", info.score.as_ref().and_then(|x| x.lowerbound_mate)) +
            &write_option("currmove", info.currmove.as_ref().map(|m| m.to_algebraic())) +
            &write_option("currmovenumber", info.currmovenumber) +
            &write_option("hashfull", info.hashfull) +
            &write_option("nps", info.nps) +
            &write_option("tbhits", info.tbhits) +
            &write_option("sbhits", info.sbhits) +
            &write_option("cpuload", info.cpuload) +
            &write_option("string", info.string.clone()) +
            //&write_option("refutation", info.refutation) +
            &write_option("currline", info.currline.as_ref().map(|currline| currline.0.map(|cpunumber| cpunumber.to_string() + " ").unwrap_or("".to_string()) + &currline.1.clone().into_iter().map(|v| v.to_algebraic()).collect::<Vec<String>>().join(" ")))
        }
        UciEngineCmd::ReadyOk => "readyok".to_string(),
        UciEngineCmd::UciOk => "uciok".to_string(),
        _ => unimplemented!(),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        GoSubCmdBuilder, IdSubCmd, InfoSubCmd, InfoSubCmdBuilder, Move, PositionSubCmd,
        ScoreSubCmdBuilder, UciEngineCmd, UciGuiCmd, parse_gui_command, write_engine_command,
    };

    #[test]
    fn test_erroneous_command_fails_to_parse() {
        assert_eq!(
            parse_gui_command("test"),
            Err(String::from("Unimplemented"))
        );
    }

    #[test]
    fn test_parse_gui_command_uci() {
        assert_eq!(parse_gui_command("uci"), Ok(UciGuiCmd::Uci));
    }

    #[test]
    fn test_parse_gui_command_debug() {
        assert_eq!(parse_gui_command("debug"), Ok(UciGuiCmd::Debug(None)));
        assert_eq!(
            parse_gui_command("debug on"),
            Ok(UciGuiCmd::Debug(Some(true)))
        );
        assert_eq!(
            parse_gui_command("debug off"),
            Ok(UciGuiCmd::Debug(Some(false)))
        );
    }

    #[test]
    fn test_parse_gui_command_isready() {
        assert_eq!(parse_gui_command("isready"), Ok(UciGuiCmd::IsReady));
    }

    #[test]
    fn test_parse_gui_command_set_option() {
        assert_eq!(
            parse_gui_command("setoption name Nullmove value true"),
            Ok(UciGuiCmd::SetOption {
                name: String::from("Nullmove"),
                value: Some(String::from("true"))
            })
        )
    }

    #[test]
    fn test_parse_gui_command_ucinewgame() {
        assert_eq!(parse_gui_command("ucinewgame"), Ok(UciGuiCmd::UciNewGame))
    }

    #[test]
    fn test_parse_gui_command_go_searchmoves() {
        assert_eq!(
            parse_gui_command("go searchmoves e2e4 d2d4"),
            Ok(UciGuiCmd::Go(
                GoSubCmdBuilder::new()
                    .with_search_moves(&vec![
                        Move::from_full_algebraic("e2e4").unwrap(),
                        Move::from_full_algebraic("d2d4").unwrap()
                    ])
                    .build()
            ))
        );
    }

    #[test]
    fn test_parse_gui_command_go_ponder() {
        assert_eq!(
            parse_gui_command("go"),
            Ok(UciGuiCmd::Go(
                GoSubCmdBuilder::new().with_ponder(false).build()
            ))
        );

        assert_eq!(
            parse_gui_command("go ponder"),
            Ok(UciGuiCmd::Go(
                GoSubCmdBuilder::new().with_ponder(true).build()
            ))
        );
    }

    #[test]
    fn test_parse_gui_command_go_time_commands() {
        assert_eq!(
            parse_gui_command("go wtime 600000 btime 600000 winc 30000 binc 30000 movestogo 40"),
            Ok(UciGuiCmd::Go(
                GoSubCmdBuilder::new()
                    .with_wtime(600000)
                    .with_btime(600000)
                    .with_winc(30000)
                    .with_binc(30000)
                    .with_movestogo(40)
                    .build()
            ))
        );
    }

    #[test]
    fn test_parse_gui_command_go_tree_commands() {
        assert_eq!(
            parse_gui_command("go depth 3 nodes 10000 mate 5"),
            Ok(UciGuiCmd::Go(
                GoSubCmdBuilder::new()
                    .with_depth(3)
                    .with_nodes(10000)
                    .with_mate(5)
                    .build()
            ))
        );
    }

    #[test]
    fn test_parse_gui_command_ponderhit() {
        assert_eq!(parse_gui_command("ponderhit"), Ok(UciGuiCmd::PonderHit));
    }

    #[test]
    fn test_parse_gui_command_position_startpos() {
        assert_eq!(
            parse_gui_command("position startpos"),
            Ok(UciGuiCmd::Position {
                pos: PositionSubCmd::StartPos,
                moves: Vec::new()
            })
        );
    }

    #[test]
    fn test_parse_gui_command_position_startpos_with_moves() {
        assert_eq!(
            parse_gui_command("position startpos moves e2e3 d2d3"),
            Ok(UciGuiCmd::Position {
                pos: PositionSubCmd::StartPos,
                moves: vec![
                    Move::from_full_algebraic("e2e3").unwrap(),
                    Move::from_full_algebraic("d2d3").unwrap()
                ]
            })
        );
    }

    #[test]
    fn test_parse_gui_command_position_fen() {
        assert_eq!(
            parse_gui_command(
                "position fen rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq e3 0 1"
            ),
            Ok(UciGuiCmd::Position {
                pos: PositionSubCmd::Fen(
                    "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq e3 0 1".to_string()
                ),
                moves: Vec::new()
            })
        );
    }

    #[test]
    fn test_parse_gui_command_position_fen_with_moves() {
        assert_eq!(
            parse_gui_command(
                "position fen rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq e3 0 1 moves e2e3 d2d3"
            ),
            Ok(UciGuiCmd::Position {
                pos: PositionSubCmd::Fen(
                    "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq e3 0 1".to_string()
                ),
                moves: vec![
                    Move::from_full_algebraic("e2e3").unwrap(),
                    Move::from_full_algebraic("d2d3").unwrap()
                ]
            })
        );
    }

    #[test]
    fn test_parse_gui_command_quit() {
        assert_eq!(parse_gui_command("quit"), Ok(UciGuiCmd::Quit));
    }

    #[test]
    fn test_parse_gui_command_stop() {
        assert_eq!(parse_gui_command("stop"), Ok(UciGuiCmd::Stop));
    }

    #[test]
    fn test_write_engine_command_bestmove() {
        assert_eq!(
            write_engine_command(&UciEngineCmd::BestMove {
                mov: Move::from_full_algebraic("e3e4").unwrap(),
                ponder: None
            }),
            String::from("bestmove e3e4")
        );
    }

    #[test]
    fn test_write_engine_command_bestmove_with_ponder() {
        assert_eq!(
            write_engine_command(&UciEngineCmd::BestMove {
                mov: Move::from_full_algebraic("e3e4").unwrap(),
                ponder: Move::from_full_algebraic("e4e5")
            }),
            String::from("bestmove e3e4 ponder e4e5")
        );
    }

    #[test]
    fn test_write_engine_command_id_name() {
        assert_eq!(
            write_engine_command(&UciEngineCmd::Id(IdSubCmd::Name("engine name".to_string()))),
            "id name engine name".to_string()
        );
    }

    #[test]
    fn test_write_engine_command_id_author() {
        assert_eq!(
            write_engine_command(&UciEngineCmd::Id(IdSubCmd::Author(
                "author name".to_string()
            ))),
            "id author author name".to_string()
        );
    }

    #[test]
    fn test_write_engine_command_info_depth() {
        assert_eq!(
            write_engine_command(&UciEngineCmd::Info(
                InfoSubCmdBuilder::new().with_depth(42).build()
            )),
            "info depth 42"
        );
    }

    #[test]
    fn test_write_engine_command_info_seldepth() {
        assert_eq!(
            write_engine_command(&UciEngineCmd::Info(
                InfoSubCmdBuilder::new().with_seldepth(42).build()
            )),
            "info seldepth 42"
        );
    }

    #[test]
    fn test_write_engine_command_info_time() {
        assert_eq!(
            write_engine_command(&UciEngineCmd::Info(
                InfoSubCmdBuilder::new().with_time(42).build()
            )),
            "info time 42"
        );
    }

    #[test]
    fn test_write_engine_command_info_nodes() {
        assert_eq!(
            write_engine_command(&UciEngineCmd::Info(
                InfoSubCmdBuilder::new().with_nodes(42).build()
            )),
            "info nodes 42"
        );
    }

    #[test]
    fn test_write_engine_command_info_pv() {
        assert_eq!(
            write_engine_command(&UciEngineCmd::Info(
                InfoSubCmdBuilder::new()
                    .with_pv(&vec![
                        Move::from_full_algebraic("e2e4").unwrap(),
                        Move::from_full_algebraic("c5c6").unwrap(),
                        Move::from_full_algebraic("d3e5").unwrap(),
                    ])
                    .build()
            )),
            "info pv e2e4 c5c6 d3e5"
        );
    }

    #[test]
    fn test_write_engine_command_info_multipv() {
        assert_eq!(
            write_engine_command(&UciEngineCmd::Info(
                InfoSubCmdBuilder::new().with_multipv(42).build()
            )),
            "info multipv 42"
        );
    }

    #[test]
    fn test_write_engine_command_info_score_cp() {
        assert_eq!(
            write_engine_command(&UciEngineCmd::Info(
                InfoSubCmdBuilder::new()
                    .with_score(&ScoreSubCmdBuilder::new().with_cp(42))
                    .build()
            )),
            "info score cp 42"
        );
    }

    #[test]
    fn test_write_engine_command_info_score_mate() {
        assert_eq!(
            write_engine_command(&UciEngineCmd::Info(
                InfoSubCmdBuilder::new()
                    .with_score(&ScoreSubCmdBuilder::new().with_mate(42))
                    .build()
            )),
            "info score mate 42"
        );
    }

    #[test]
    fn test_write_engine_command_info_score_upperbound_cp() {
        assert_eq!(
            write_engine_command(&UciEngineCmd::Info(
                InfoSubCmdBuilder::new()
                    .with_score(&ScoreSubCmdBuilder::new().with_upperbound_cp(42))
                    .build()
            )),
            "info score upperbound cp 42"
        );
    }

    #[test]
    fn test_write_engine_command_info_score_upperbound_mate() {
        assert_eq!(
            write_engine_command(&UciEngineCmd::Info(
                InfoSubCmdBuilder::new()
                    .with_score(&ScoreSubCmdBuilder::new().with_upperbound_mate(42))
                    .build()
            )),
            "info score upperbound mate 42"
        );
    }

    #[test]
    fn test_write_engine_command_info_score_lowerbound_cp() {
        assert_eq!(
            write_engine_command(&UciEngineCmd::Info(
                InfoSubCmdBuilder::new()
                    .with_score(&ScoreSubCmdBuilder::new().with_lowerbound_cp(42))
                    .build()
            )),
            "info score lowerbound cp 42"
        );
    }

    #[test]
    fn test_write_engine_command_info_score_lowerbound_mate() {
        assert_eq!(
            write_engine_command(&UciEngineCmd::Info(
                InfoSubCmdBuilder::new()
                    .with_score(&ScoreSubCmdBuilder::new().with_lowerbound_mate(42))
                    .build()
            )),
            "info score lowerbound mate 42"
        );
    }

    #[test]
    fn test_write_engine_command_info_currmove() {
        assert_eq!(
            write_engine_command(&UciEngineCmd::Info(
                InfoSubCmdBuilder::new()
                    .with_currmove(&Move::from_full_algebraic("e2e4").unwrap())
                    .build()
            )),
            "info currmove e2e4"
        );
    }

    #[test]
    fn test_write_engine_command_info_currmovenumber() {
        assert_eq!(
            write_engine_command(&UciEngineCmd::Info(
                InfoSubCmdBuilder::new().with_currmovenumber(42).build()
            )),
            "info currmovenumber 42"
        );
    }

    #[test]
    fn test_write_engine_command_info_hashfull() {
        assert_eq!(
            write_engine_command(&UciEngineCmd::Info(
                InfoSubCmdBuilder::new().with_hashfull(42).build()
            )),
            "info hashfull 42"
        );
    }

    #[test]
    fn test_write_engine_command_info_nps() {
        assert_eq!(
            write_engine_command(&UciEngineCmd::Info(
                InfoSubCmdBuilder::new().with_nps(42).build()
            )),
            "info nps 42"
        );
    }

    #[test]
    fn test_write_engine_command_info_tbhits() {
        assert_eq!(
            write_engine_command(&UciEngineCmd::Info(
                InfoSubCmdBuilder::new().with_tbhits(42).build()
            )),
            "info tbhits 42"
        );
    }

    #[test]
    fn test_write_engine_command_info_sbhits() {
        assert_eq!(
            write_engine_command(&UciEngineCmd::Info(
                InfoSubCmdBuilder::new().with_sbhits(42).build()
            )),
            "info sbhits 42"
        );
    }

    #[test]
    fn test_write_engine_command_info_cpuload() {
        assert_eq!(
            write_engine_command(&UciEngineCmd::Info(
                InfoSubCmdBuilder::new().with_cpuload(42).build()
            )),
            "info cpuload 42"
        );
    }

    #[test]
    fn test_write_engine_command_info_string() {
        assert_eq!(
            write_engine_command(&UciEngineCmd::Info(
                InfoSubCmdBuilder::new()
                    .with_string("the quick brown fox jumps over the lazy fox")
                    .build()
            )),
            "info string the quick brown fox jumps over the lazy fox"
        );
    }

    #[test]
    fn test_write_engine_command_info_currline_with_cpunumber() {
        assert_eq!(
            write_engine_command(&UciEngineCmd::Info(
                InfoSubCmdBuilder::new()
                    .with_currline((
                        Some(42),
                        vec![
                            Move::from_full_algebraic("e2e4").unwrap(),
                            Move::from_full_algebraic("c5c6").unwrap()
                        ]
                    ))
                    .build()
            )),
            "info currline 42 e2e4 c5c6"
        );
    }

    #[test]
    fn test_write_engine_command_info_currline_without_cpunumber() {
        assert_eq!(
            write_engine_command(&UciEngineCmd::Info(
                InfoSubCmdBuilder::new()
                    .with_currline((
                        None,
                        vec![
                            Move::from_full_algebraic("e2e4").unwrap(),
                            Move::from_full_algebraic("c5c6").unwrap()
                        ]
                    ))
                    .build()
            )),
            "info currline e2e4 c5c6"
        );
    }

    #[test]
    fn test_write_engine_command_readyok() {
        assert_eq!(write_engine_command(&UciEngineCmd::ReadyOk), "readyok")
    }

    #[test]
    fn test_write_engine_command_uciok() {
        assert_eq!(write_engine_command(&UciEngineCmd::UciOk), "uciok")
    }
}
