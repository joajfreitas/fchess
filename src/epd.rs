use crate::Board;
use anyhow::Result;
use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Epd {
    pub board: Board,
    pub properties: HashMap<String, String>,
}

#[allow(dead_code)]
impl Epd {
    pub fn new(board: &Board, properties: HashMap<String, String>) -> Epd {
        Epd {
            board: board.clone(),
            properties,
        }
    }

    pub fn from_string(s: &str) -> Result<Epd> {
        let mut properties: HashMap<String, String> = HashMap::default();
        let mut iter = s.split(" ").skip(6);
        loop {
            let key = iter.next();
            let value = iter.next();

            if key.is_none() || value.is_none() {
                break;
            }

            properties.insert(
                key.unwrap().to_string().replace(";", ""),
                value.unwrap().to_string().replace(";", ""),
            );
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
    use googletest::prelude::*;

    #[gtest]
    fn test_simple_epd() {
        let epd =
            Epd::from_string("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 bm g6g4")
                .unwrap();

        assert_that!(
            Epd::new(
                &Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
                    .unwrap(),
                HashMap::from([("bm".to_string(), "g6g4".to_string())])
            ),
            eq(&epd)
        );
    }

    #[gtest]
    fn test_get_property() {
        let epd = Epd::from_string("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 ;D1 20 ;D2 400 ;D3 8902 ;D4 197281 ;D5 4865609 ;D6 119060324").unwrap();

        assert_that!(epd.properties.get("D1").unwrap(), eq("20"));
    }
}
