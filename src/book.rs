use std::fmt;
use std::fs;

use crate::board::{Board, Side};
use crate::moves::Move;
use crate::square::Square;

#[derive(Clone, Copy)]
pub struct Entry {
    key: u64,
    mov: u16,
    weight: u16,
}

impl Entry {
    fn new(key: u64, mov: u16, weight: u16) -> Entry {
        Entry { key, mov, weight }
    }
    fn get_from(&self) -> Square {
        let file: u8 = (self.mov >> 6 & 0x7) as u8;
        let rank: u8 = (self.mov >> 9 & 0x7) as u8;

        Square::from_rank_file(rank, file)
    }

    fn get_to(&self) -> Square {
        let file: u8 = (self.mov & 0x7) as u8;
        let rank: u8 = (self.mov >> 3 & 0x7) as u8;

        Square::from_rank_file(rank, file)
    }

    fn get_key(&self) -> u64 {
        self.key
    }

    fn get_weight(&self) -> u16 {
        self.weight
    }
}

impl fmt::Debug for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:x} ", self.key)?;
        write!(f, "{:?} ", self.get_from())?;
        write!(f, "{:?} ", self.get_to())?;
        write!(f, "{}", self.get_weight())
    }
}

pub struct Book(Vec<Entry>);

impl Book {
    pub fn from_filename(filename: &str) -> Book {
        let bytes = fs::read(filename).unwrap();
        let struct_size = 8 + 2 + 2 + 4;
        let mut book: Book = Book(vec![]);
        for i in 0..(bytes.len() / struct_size) {
            let key: u64 = ((bytes[struct_size * i] as u64) << (7 * 8))
                + ((bytes[struct_size * i + 1] as u64) << (6 * 8))
                + ((bytes[struct_size * i + 2] as u64) << (5 * 8))
                + ((bytes[struct_size * i + 3] as u64) << (4 * 8))
                + ((bytes[struct_size * i + 4] as u64) << (3 * 8))
                + ((bytes[struct_size * i + 5] as u64) << (2 * 8))
                + ((bytes[struct_size * i + 6] as u64) << 8)
                + (bytes[struct_size * i + 7] as u64);

            let mov: u16 =
                ((bytes[struct_size * i + 8] as u16) << 8) + (bytes[struct_size * i + 9] as u16);
            let weight: u16 =
                ((bytes[struct_size * i + 10] as u16) << 8) + bytes[struct_size * i + 11] as u16;

            book.0.push(Entry::new(key, mov, weight));
        }
        book
    }

    fn binary_search(&self, key: u64) -> usize {
        let mut lo: usize = 0;
        let mut hi: usize = self.0.len();
        while lo < hi {
            let mid = (lo + hi) / 2;
            let entry = self.0[mid];
            if entry.get_key() < key {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }

        lo
    }

    fn find_all(&self, board: &Board, turn: &Side) -> Vec<Entry> {
        let hash = board.zobryst_hash(turn);
        let index = self.binary_search(hash);

        let mut entries: Vec<Entry> = vec![];
        for entry in self.0[index..].iter() {
            if entry.get_key() != hash {
                break;
            }

            entries.push(*entry);
        }

        entries
    }

    pub fn get_best_move(&self, board: &Board, turn: &Side) -> Option<Move> {
        let all_entries = self.find_all(board, turn);
        let best_entry = all_entries.iter().max_by_key(|entry| entry.get_weight())?;

        Some(Move::new(best_entry.get_from(), best_entry.get_to()))
    }
}
