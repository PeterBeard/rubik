// This file is part of Rubik.
// Copyright Peter Beard, licensed under the GPLv3. See LICENSE for details.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Corner {
    UFL,
    URF,
    UBR,
    ULB,
    DBL,
    DLF,
    DFR,
    DRB,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Edge {
    UB,
    UR,
    UF,
    UL,
    LB,
    RB,
    RF,
    LF,
    DB,
    DR,
    DF,
    DL,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Move {
    F,
    R,
    U,
    B,
    L,
    D,
}

impl From<char> for Move {
    fn from(ch: char) -> Move {
        match ch {
            'F' => Move::F,
            'R' => Move::R,
            'U' => Move::U,
            'B' => Move::B,
            'L' => Move::L,
            'D' => Move::D,
            _ => panic!("Invalid move: {}", ch),
        }
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
struct X(u8,u8,u8,u8,u8,u8,u8,u8);

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
struct Y(u8,u8,u8,u8,u8,u8,u8,u8,u8,u8,u8,u8);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Cube {
    sigma: Vec<Corner>,
    tau: Vec<Edge>,
    x: X,
    y: Y,
}

impl Cube {
    pub fn new() -> Cube {
        Cube {
            sigma: Vec::with_capacity(8),
            tau: Vec::with_capacity(12),
            x: X::default(),
            y: Y::default(),
        }
    }

    pub fn apply_moves(&mut self, moves: &str) {
        use self::Move;
        let mut movelist: Vec<Move> = Vec::new();
        let mut prevch = 'X';
        for ch in moves.chars() {
            match ch {
                'F' | 'R' | 'U' | 'B' | 'L' | 'D' => {
                    movelist.push(Move::from(ch));
                },
                '2' => {
                    movelist.push(Move::from(prevch));
                },
                '\'' | '`' | '\u{2032}' => {
                    movelist.push(Move::from(prevch));
                    movelist.push(Move::from(prevch));
                },
                _ => {
                    panic!("Unrecognized move: {}", ch);
                }
            }
            prevch = ch;
        }
        
        for m in movelist {
            self.apply_move(m);
        }
    }

    pub fn apply_move(&mut self, m: Move) {
        use self::Move;
        println!("Moving {:?}", m);
        match m {
            Move::F => {

            },
            Move::R => {

            },
            Move::U => {

            },
            Move::B => {

            },
            Move::L => {

            },
            Move::D => {

            },
        }
    }

    pub fn is_solved(&self) -> bool {
        self.sigma.len() == 0 &&
        self.tau.len() == 0 &&
        self.x == X::default() &&
        self.y == Y::default()
    }
}
