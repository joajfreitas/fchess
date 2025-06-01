use crate::Board;
use anyhow::Result;
use std::collections::HashMap;
use std::collections::VecDeque;

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
        let sp = s.split(" ").collect::<Vec<&str>>();
        let fen = sp[..6]
            .iter()
            .map(|x| x.to_string())
            .intersperse(" ".to_string())
            .collect::<String>();

        let mut properties = sp[6..]
            .iter()
            .map(|x| x.to_string())
            .collect::<VecDeque<String>>();

        let mut property_map: HashMap<String, String> = HashMap::default();

        let mut key = "".to_string();
        let mut value;
        let mut state = 0;

        while !properties.is_empty() {
            let v = properties.pop_front().unwrap();
            if state == 0 {
                key = v;
                state = 1;
            } else if state == 1 {
                value = v;
                state = 0;
                property_map.insert(key.clone(), value.to_string());
            }
        }

        Ok(Epd {
            board: Board::from_fen(&fen)?,
            properties: property_map,
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
                &Board::from_fen(
                    "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 bm g6g4"
                )
                .unwrap(),
                HashMap::from([("bm".to_string(), "g6g4".to_string())])
            ),
            epd
        );
    }
}
