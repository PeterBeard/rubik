// This file is part of Rubik.
// Copyright Peter Beard, licensed under the GPLv3. See LICENSE for details.
//
//! Objects and functions for maintaining/manipulating Rubik's cube state.
use std::collections::HashMap;
use std::hash::Hash;
use std::fmt;

/// A Corner of a Rubik's cube (there are 8)
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
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

/// An edge of a Rubik's cube (there are 12)
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
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

/// There are six possible Rubik's cube moves: Front, Right, Up, Back, Left, and Down.
/// A move consists of a single clockwise rotation of the corresponding face.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Move {
    F,
    R,
    U,
    B,
    L,
    D,
}

/// Create a Move from a char. See
/// [http://rubiks.wikia.com/wiki/Notation](http://rubiks.wikia.com/wiki/Notation) 
/// for notation.
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

/// Takes a symmetric group and returns a vector representing its disjoint
/// cycles including cycles with length 1.
fn disjoint_cycle_decompose<T: Copy + Eq + Hash>(map: &HashMap<T, T>) -> Vec<Vec<T>> {
    let mut cycles: Vec<Vec<T>> = Vec::new();
    let mut current_cycle: Vec<T> = Vec::new();
    let mut used_values: Vec<T> = Vec::new();
    for &v in map.values() {
        if used_values.contains(&v) {
            continue;
        }

        current_cycle.push(v);
        let mut current_value = v;
        loop {
            if (current_cycle.len() > 1 && current_cycle[0] == current_value)
            || (used_values.contains(&current_value)) {
                current_cycle.pop();
                cycles.push(current_cycle.clone());
                current_cycle = Vec::new();
                break;
            }
            used_values.push(current_value);
            current_value = *(map.get(&current_value).unwrap());
            current_cycle.push(current_value);
        }
    }

    // Sort the cycles by length
    cycles.sort_by_key(|k| k.len());
    cycles.reverse();

    cycles
}

#[derive(Clone, Eq, PartialEq)]
/// Map corner cubicles to cubies
struct CornerPermutation {
    map: HashMap<Corner, Corner>,
}

impl Default for CornerPermutation {
    /// The default mapping has each cubie in its corresponding cubicle
    fn default() -> CornerPermutation {
        let mut m: HashMap<Corner, Corner> = HashMap::new();
        // Default mapping
        use self::Corner;
        m.insert(Corner::UFL, Corner::UFL);
        m.insert(Corner::URF, Corner::URF);
        m.insert(Corner::UBR, Corner::UBR);
        m.insert(Corner::ULB, Corner::ULB);
        m.insert(Corner::DBL, Corner::DBL);
        m.insert(Corner::DLF, Corner::DLF);
        m.insert(Corner::DFR, Corner::DFR);
        m.insert(Corner::DRB, Corner::DRB);

        CornerPermutation {
            map: m,
        }
    }
}

impl fmt::Debug for CornerPermutation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let cycles = disjoint_cycle_decompose(&self.map);
        write!(f, "{:?}", cycles)
    }
}

impl CornerPermutation {
    /// Create a solved permutation
    pub fn new() -> CornerPermutation {
        CornerPermutation::default()
    }

    /// Apply a move and determine which cubies end up where
    pub fn permute(&mut self, m: Move) {
        use self::Move;
        use self::Corner;
        let cycle = match m {
            Move::F => {
                (Corner::URF, Corner::DFR, Corner::DLF, Corner::UFL)
            },
            Move::R => {
                (Corner::UBR, Corner::DRB, Corner::DFR, Corner::URF)
            },
            Move::U => {
                (Corner::URF, Corner::UFL, Corner::ULB, Corner::UBR)
            },
            Move::B => {
                (Corner::ULB, Corner::DBL, Corner::DRB, Corner::UBR)
            },
            Move::L => {
                (Corner::UFL, Corner::DLF, Corner::DBL, Corner::ULB)
            },
            Move::D => {
                (Corner::DRB, Corner::DBL, Corner::DLF, Corner::DFR)
            },
        };

        let mut new_map = self.map.clone();
        new_map.insert(cycle.1, *self.map.get(&cycle.0).unwrap());
        new_map.insert(cycle.2, *self.map.get(&cycle.1).unwrap());
        new_map.insert(cycle.3, *self.map.get(&cycle.2).unwrap());
        new_map.insert(cycle.0, *self.map.get(&cycle.3).unwrap());
        self.map = new_map;
    }
}

#[derive(Clone, Eq, PartialEq)]
/// Map edge cubicles to cubies
struct EdgePermutation {
    map: HashMap<Edge, Edge>,
}

impl Default for EdgePermutation {
    /// The default mapping has each cubie in its corresponding cubicle
    fn default() -> EdgePermutation {
        let mut m: HashMap<Edge, Edge> = HashMap::new();
        use self::Edge;
        m.insert(Edge::UB, Edge::UB);
        m.insert(Edge::UR, Edge::UR);
        m.insert(Edge::UF, Edge::UF);
        m.insert(Edge::UL, Edge::UL);
        m.insert(Edge::LB, Edge::LB);
        m.insert(Edge::RB, Edge::RB);
        m.insert(Edge::RF, Edge::RF);
        m.insert(Edge::LF, Edge::LF);
        m.insert(Edge::DB, Edge::DB);
        m.insert(Edge::DR, Edge::DR);
        m.insert(Edge::DF, Edge::DF);
        m.insert(Edge::DL, Edge::DL);

        EdgePermutation {
            map: m,
        }
    }
}

impl fmt::Debug for EdgePermutation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let cycles = disjoint_cycle_decompose(&self.map);
        write!(f, "{:?}", cycles)
    }
}

impl EdgePermutation {
    /// Create a new solved permutation
    pub fn new() -> EdgePermutation {
        EdgePermutation::default()
    }

    /// Apply a move and determine which cubies end up where
    pub fn permute(&mut self, m: Move) {
        use self::Move;
        let cycle = match m {
            Move::F => {
                (Edge::UF, Edge::RF, Edge::DF, Edge::LF)
            },
            Move::R => {
                (Edge::UR, Edge::RB, Edge::DR, Edge::RF)
            },
            Move::U => {
                (Edge::UB, Edge::UR, Edge::UF, Edge::UL)
            },
            Move::B => {
                (Edge::UB, Edge::LB, Edge::DB, Edge::RB)
            },
            Move::L => {
                (Edge::UL, Edge::LF, Edge::DL, Edge::LB)
            },
            Move::D => {
                (Edge::DF, Edge::DR, Edge::DB, Edge::DL)
            },
        };

        let mut new_map = self.map.clone();
        new_map.insert(cycle.1, *self.map.get(&cycle.0).unwrap());
        new_map.insert(cycle.2, *self.map.get(&cycle.1).unwrap());
        new_map.insert(cycle.3, *self.map.get(&cycle.2).unwrap());
        new_map.insert(cycle.0, *self.map.get(&cycle.3).unwrap());
        self.map = new_map;
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
/// Corner orientation state
struct X(u8,u8,u8,u8,u8,u8,u8,u8);

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
/// Edge orientation state
struct Y(u8,u8,u8,u8,u8,u8,u8,u8,u8,u8,u8,u8);

#[derive(Clone, Eq, PartialEq)]
/// Maintain the state information for a Rubik's cube.
pub struct Cube {
    sigma: CornerPermutation,
    tau: EdgePermutation,
    x: X,
    y: Y,
}

impl fmt::Debug for Cube {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\u{03c3} = {:?}\n\u{03c4} = {:?}\nx = {:?}\ny = {:?}",
               self.sigma, self.tau, self.x, self.y)
    }
}

impl Cube {
    /// Create a new cube in the solved state.
    pub fn new() -> Cube {
        Cube {
            sigma: CornerPermutation::new(),
            tau: EdgePermutation::new(),
            x: X::default(),
            y: Y::default(),
        }
    }

    /// Apply a string of moves to a cube
    ///
    /// # Arguments
    /// moves: A move or moves to apply to the cube, e.g. FRUU'R'F'
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

    /// Apply a single move to the cube.
    pub fn apply_move(&mut self, m: Move) {
        use self::Move;
        println!("Moving {:?}", m);
        // Compute sigma and tau
        self.sigma.permute(m);
        self.tau.permute(m);

        // Compute X and Y
    }

    /// Determine whether the cube is in the solved state
    pub fn is_solved(&self) -> bool {
        self.sigma == CornerPermutation::default() &&
        self.tau == EdgePermutation::default() &&
        self.x == X::default() &&
        self.y == Y::default()
    }
}
