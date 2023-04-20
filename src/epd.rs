
use std::collections::HashMap;
use std::collections::VecDeque;
use crate::Board;

#[derive(Debug, Eq, PartialEq)]
struct Epd {
    board: Board,
    properties: HashMap<String, String>,
}

impl Epd {
    fn new(board: &Board, properties: HashMap<String, String>) -> Epd {
        Epd {
            board: board.clone(),
            properties
        }
    }
    fn from_string(s: &str) -> Epd {
        let sp = s.split(" ").collect::<Vec<&str>>();
        let fen = sp[..6].iter().map(|x| x.to_string()).intersperse(" ".to_string()).collect::<String>();

        let mut properties = sp[6..].iter().map(|x| x.to_string()).collect::<VecDeque<String>>();
        
        let mut property_map: HashMap<String, String> = HashMap::default();

        let mut key = "".to_string();
        let mut value = "".to_string();
        let mut state = 0;

        while !properties.is_empty() {
            let v = properties.pop_front().unwrap();
            if state == 0 {
                key = v;
                state = 1;
            }
            else if state == 1 {
                value = v;
                state = 0;
                property_map.insert(key.clone(), value.to_string());
            }
        }

        Epd {
            board: Board::from_fen(&fen),
            properties: property_map,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn  test_simple_epd() {
        let epd = Epd::from_string("5N1r/5n1n/ppp3R1/5K2/7k/6p1/6PN/8 w - - 0 1 bm g6g4");
        
        let properties: HashMap<String, String> = [("bm".to_string(), "g6g4".to_string())].iter().cloned().collect();
        assert_eq!(Epd::new(&Board::from_fen("5N1r/5n1n/ppp3R1/5K2/7k/6p1/6PN/8 w - - 0 1 bm g6g4"), properties), epd);
    }
}
