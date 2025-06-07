use crate::Board;
use anyhow::Result;
use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Epd {
    board: Board,
    properties: HashMap<String, String>,
}

impl Epd {
    pub fn new(board: &Board, properties: HashMap<String, String>) -> Epd {
        Epd {
            board: board.clone(),
            properties,
        }
    }

    pub fn get_board(&self) -> Board {
        self.board.clone()
    }

    pub fn get_properties(&self) -> HashMap<String, String> {
        self.properties.clone()
    }

    pub fn from_string(s: &str) -> Result<Epd> {
        let mut properties: HashMap<String, String> = HashMap::default();
        let mut iter = s.split(" ").into_iter().skip(6);
        loop {
            let key = iter.next();
            let value = iter.next();

            if key.is_none() || value.is_none() {
                break;
            }

            properties.insert(key.unwrap().to_string(), value.unwrap().to_string());
        }

        Ok(Epd {
            board: Board::from_fen(s)?,
            properties,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_epd() {
        let epd =
            Epd::from_string("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 bm g6g4")
                .unwrap();

        assert_eq!(
            Epd::new(
                &Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
                    .unwrap(),
                HashMap::from([("bm".to_string(), "g6g4".to_string())])
            ),
            epd
        );
    }
}
