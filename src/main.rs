use std::fmt;

#[derive(Copy, Clone)]
enum Piece {
    WhitePawn=0,
    WhiteRook=1,
    WhiteKnight=2,
    WhiteBishop=3,
    WhiteQueen=4,
    WhiteKing=5,
    BlackPawn=6,
    BlackRook=7,
    BlackKnight=8,
    BlackBishop=9,
    BlackQueen=10,
    BlackKing=11,
    Marker=12,
    NoPiece,
}

enum Scope {
    All=0,
    White=1,
    Black=2,
}

enum Direction {
    N=0,
    NE=1,
    E=2,
    SE=3,
    S=4,
    SW=5,
    W=6,
    NW=7,
}



impl Scope {
    fn to_range(self: &Scope) -> (u8, u8) {
        match self {
            Scope::All => (0, 12),
            Scope::White => (0, 6),
            Scope::Black => (6, 12),
        }
    }
}

fn from_int(i: u8) -> Piece {
    match i {
        0 => Piece::WhitePawn,
        1 => Piece::WhiteRook,
        2 => Piece::WhiteKnight,
        3 => Piece::WhiteBishop,
        4 => Piece::WhiteQueen,
        5 => Piece::WhiteKing,
        6 => Piece::BlackPawn,
        7 => Piece::BlackRook,
        8 => Piece::BlackKnight,
        9 => Piece::BlackBishop,
        10 => Piece::BlackQueen,
        11 => Piece::BlackKing,
        12 => Piece::Marker,
        _ => Piece::NoPiece
    }
}

impl fmt::Debug for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let piece = match self {
            Piece::WhitePawn => "♙",
            Piece::BlackPawn => "♟︎",
            Piece::WhiteRook => "♖",
            Piece::BlackRook => "♜",
            Piece::WhiteKnight => "♘",
            Piece::BlackKnight => "♞",
            Piece::WhiteBishop => "♗",
            Piece::BlackBishop => "♝",
            Piece::WhiteQueen => "♕",
            Piece::BlackQueen => "♛",
            Piece::WhiteKing => "♔",
            Piece::BlackKing => "♚",
            Piece::Marker => "*",
            _ => " ",
        };
        f.write_str(piece)
    }
}



#[derive(Default)]
struct Board {
    pieces: [u64; 13],
}


fn flood_south(gen: u64, pro: u64) -> u64 {
    let mut gen = gen;
    let mut flood = 0;

    while gen > 0 {
      flood |= gen;
      gen = (gen >> 8) & pro;
    }
    flood

}

fn flood_north(gen: u64, pro: u64) -> u64 {
    let mut gen = gen;
    let mut flood = 0;

    while gen > 0 {
      flood |= gen;
      gen = (gen << 8) & pro;
   }
   flood
}

fn flood_east(gen: u64, pro: u64) -> u64 {
    let mut gen = gen;
    let mut flood = 0;

    while gen > 0 {
      flood |= gen;
      gen = (gen >> 1) & pro;
   }
   flood
}

fn flood_west(gen: u64, pro: u64) -> u64 {
    let mut gen = gen;
    let mut flood = 0;

    while gen > 0 {
      flood |= gen;
      gen = (gen << 1) & pro;
   }
   flood
}

impl Board {
    fn start(self: &mut Board) {
        /*
        for i in 0..8 {
            self.set(&Piece::WhitePawn, (1,i));
        }
        for i in 0..8 {
            self.set(&Piece::BlackPawn, (6,i));
        }

        self.set(&Piece::WhiteRook, (0,0));
        self.set(&Piece::WhiteRook, (0,7));
        self.set(&Piece::BlackRook, (7,0));
        self.set(&Piece::BlackRook, (7,7));
        self.set(&Piece::WhiteKnight, (0,1));
        self.set(&Piece::WhiteKnight, (0,6));
        self.set(&Piece::BlackKnight, (7,1));
        self.set(&Piece::BlackKnight, (7,6));
        self.set(&Piece::WhiteBishop, (0,2));
        self.set(&Piece::WhiteBishop, (0,5));
        self.set(&Piece::BlackBishop, (7,2));
        self.set(&Piece::BlackBishop, (7,5));
        self.set(&Piece::WhiteQueen, (0,4));
        self.set(&Piece::BlackQueen, (7,4));
        self.set(&Piece::WhiteKing, (0,3));
        self.set(&Piece::BlackKing, (7,3));
        */
        
        self.set(&Piece::WhiteRook, (3,3));
        self.set(&Piece::BlackRook, (3,0));
        self.set(&Piece::BlackRook, (3,7));
        self.set(&Piece::BlackRook, (7,3));
        self.set(&Piece::BlackRook, (0,3));

        /*
        for mov in self.generate_valid_moves(&Piece::BlackPawn, (6,1)) {
            self.set(&Piece::Marker, mov);
        }
        */

        for mov in self.generate_moves(&Scope::White) {
            let (x,y, piece) = mov;
            self.set(&Piece::Marker, (x,y));
        }


    }
    
    fn set(self: &mut Board, piece: &Piece, coords: (u8, u8)) {
        let (x, y) = coords;
        let index = 8*x + y;
        let piece_index = *piece as u8;
        self.pieces[piece_index as usize] |= 1 << index;
    }

    fn clear(self: &mut Board, coords: (u8, u8)) {
        let (x, y) = coords;
        let index = 8*x + y;

        for i in 0..12 {
            self.pieces[i] &= !(1 << index);
        }
    }

    fn occupied(self: &Board, scope: &Scope) -> u64 {
        let mut occupancy: u64 = 0;

        let (start, end) = scope.to_range();

        for i in start..end {
            occupancy |= self.pieces[i as usize];
        }
        occupancy
    }


    fn flood(self: &Board, piece: (u8,u8), dir: &Direction) -> u64 {
        let free = !self.occupied(&Scope::All);
        let (x, y) = piece;
        let index = 8*x + y;

        let mut gen : u64 = 1 << index;
        let start = gen;

        println!("{:#x}", self.occupied(&Scope::All));
        println!("{:#x}", free);
        let gen = match dir {
            Direction::N => flood_north(gen, free),
            Direction::E => flood_east(gen, free),
            Direction::S => flood_south(gen, free),
            Direction::W => flood_west(gen, free),
            _ => panic!(),
        };

        println!("gen {:#x}", gen);
        gen ^ start
    }

    fn check_occupancy(self: &Board, point: (u8, u8), scope: &Scope) -> bool {
        let occupancy = self.occupied(scope);
        let (x,y) = point;
        let index = 8*x + y;
        
        return ((occupancy >> index) & 1) == 1;
    }
    
    fn generate_white_rook_moves(self: &Board, src: (u8, u8)) -> Vec<(u8, u8)> {
        let (x,y) = src;
        let mut moves: Vec<(u8, u8)> = Vec::new();

        let mut f = self.flood(src, &Direction::N);
        f |= self.flood(src, &Direction::E);
        f |= self.flood(src, &Direction::S);
        f |= self.flood(src, &Direction::W);

        for i in 0..64 {
            if ((f >> i) & 1) == 1 {
                moves.push((i / 8, i%8));
            }
        }

        moves

        
    } 

    fn generate_white_pawn_moves(self: &Board, src: (u8, u8)) -> Vec<(u8, u8)> {
        let (x, y) = src; 
        let mut moves : Vec<(u8, u8)> = Vec::new();

        if x == 1 {
            if ! self.check_occupancy((3,y), &Scope::White) {
                moves.push((3,y));
            }
        }
        
        if x < 7 {
            if ! self.check_occupancy((3,y), &Scope::White) {
                moves.push((x+1, y));
            }
        }

        return moves;
    }

    fn generate_black_pawn_moves(self: &Board, src: (u8, u8)) -> Vec<(u8, u8)> {
        let (x, y) = src; 
        let mut moves : Vec<(u8, u8)> = Vec::new();

        if x == 6 {
            if ! self.check_occupancy((3,y), &Scope::Black) {
                moves.push((4,y));
            }
        }
        
        if x > 0 {
            if ! self.check_occupancy((3,y), &Scope::Black) {
                moves.push((x-1, y));
            }
        }

        return moves;
    }

    fn generate_valid_moves(self: &mut Board, piece: &Piece, src: (u8, u8)) -> Vec<(u8, u8)> {
        match piece {
            Piece::WhitePawn => self.generate_white_pawn_moves(src),
            Piece::BlackPawn => self.generate_black_pawn_moves(src),
            Piece::WhiteRook => self.generate_white_rook_moves(src),
            _ => Vec::new(),
        }
    }
    
    fn get_pieces(self: &mut Board, scope: &Scope) -> Vec<(u8,u8, Piece)> {
        let mut pieces: Vec<(u8,u8,Piece)> = Vec::new(); 
        let (start, end) = scope.to_range();

        for i in start .. end {
            for x in 0..8 {
                for y in 0..8 {
                    let index = 8*x + y;
                    if (self.pieces[i as usize] >> index) & 1 == 1 {
                        pieces.push((x,y,from_int(i)));
                    }
                }
            }
        }

        pieces
    }

    fn generate_moves(self: &mut Board, scope: &Scope) -> Vec<(u8,u8,Piece)> {
        let mut moves : Vec<(u8,u8,Piece)> = Vec::new();
        let pieces = self.get_pieces(scope);
        for (x,y,piece) in pieces {
            let movs = &mut self.generate_valid_moves(&piece, (x,y));
            for (x,y) in movs {
                moves.push((*x,*y,piece)) ;
            }
        }

        moves
    }

    fn move_piece(self: &mut Board, piece: &Piece, src: (u8, u8), dst: (u8, u8)) {
        self.clear(src);
        self.set(piece, dst);
    }

    fn piece_at(self: &Board, x:u8, y:u8) -> Piece { 
        let index = 8*x + y; 
        
        for piece_index in 0..13 {
            let bit = (self.pieces[piece_index] >> index) & 1;
            if bit == 1 {
                return from_int(piece_index as u8);
            }
        }
        return Piece::NoPiece;
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "┌───┬───┬───┬───┬───┬───┬───┬───┐\n");
        for i in 0..8 {
            for j in 0..8 {
                let piece = self.piece_at(i, j);
                write!(f, "│ {:?} ", piece);
            }
            
            if i != 7 {
                write!(f, "│\n├───┼───┼───┼───┼───┼───┼───┼───┤\n");
            }
        }
        write!(f, "│\n└───┴───┴───┴───┴───┴───┴───┴───┘\n");
        f.write_str("")
    }
}

fn main() {
    let mut board = Board {
        ..Default::default() 
    };

    board.start();
    println!("{:?}", board);
}
