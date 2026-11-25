use anyhow::{Result, anyhow};
use clap::Parser;

use fchess::{Epd, Search, SearchConfig};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    epd: String,
    depth: u32,
}

fn main() -> Result<()> {
    let args = Args::parse();

    for epd in std::fs::read_to_string(args.epd)?.lines() {
        for depth in 2..(args.depth + 1) {
            let epd = Epd::from_string(epd)?;

            let mut search = Search::new();

            let result = search
                .best_move(&epd.board, &SearchConfig::new(Some(depth)).resolve())
                .ok_or(anyhow!("Failed to find best move"))?;

            println!(
                "{} ;depth {} ;node_count {} ;eval {} ;time {} ;nps {}",
                epd.board.to_fen()?,
                depth,
                result.node_count,
                result.eval,
                result.time.as_secs_f64() * 1000.0,
                result.nps,
            );
        }
    }

    Ok(())
}
