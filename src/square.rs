#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Square {
    square: u8,
}

impl Square {
    pub fn from_rank_file(rank: u8, file: u8) -> Square {
        Square {
            square: 8 * rank + file,
        }
    }

    pub fn from_index(square: u8) -> Square {
        Square { square }
    }

    pub fn from_algebraic(mov: &str) -> Square {
        let mov: Vec<char> = mov.chars().collect();
        if mov.len() == 2 {
            let src_rank = (mov[1] as u8) - b'1';
            let src_file = (mov[0] as u8) - b'a';

            Square::from_rank_file(src_rank, src_file)
        } else {
            panic!();
        }
    }

    pub fn to_algebraic(&self) -> String {
        "a1".to_string()
    }

    pub fn get_rank(&self) -> u8 {
        self.square / 8
    }
    pub fn set_rank(&mut self, rank: u8) {
        self.square &= 0b111000;
        self.square |= rank << 3;
    }

    pub fn get_file(&self) -> u8 {
        self.square % 8
    }

    pub fn set_file(&mut self, file: u8) {
        self.square &= 0x7;
        self.square |= file;
    }

    pub fn get_index(&self) -> u8 {
        self.square
    }
}
