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

/// Get the corner cubie face corresponding to a certain orientation index
///
/// # Panics
/// This function panics if it encounters an invalid orientation
fn get_corner_face(corner: Corner, orient: u8) -> Face {
    use self::Corner::*;
    match corner {
        UFL => {
            match orient {
                0 => Face::U,
                1 => Face::F,
                2 => Face::L,
				_ => panic!("Invalid corner orientation: {}", orient)
            }
        },
        URF => {
            match orient {
                0 => Face::U,
                1 => Face::R,
                2 => Face::F,
				_ => panic!("Invalid corner orientation: {}", orient)
            }
        },
        UBR => {
            match orient {
                0 => Face::U,
                1 => Face::B,
                2 => Face::R,
				_ => panic!("Invalid corner orientation: {}", orient)
            }
        }
        ULB => {
            match orient {
                0 => Face::U,
                1 => Face::L,
                2 => Face::B,
				_ => panic!("Invalid corner orientation: {}", orient)
            }
        },
        DBL => {
            match orient {
                0 => Face::D,
                1 => Face::B,
                2 => Face::L,
				_ => panic!("Invalid corner orientation: {}", orient)
            }
        },
        DLF => {
            match orient {
                0 => Face::D,
                1 => Face::L,
                2 => Face::F,
				_ => panic!("Invalid corner orientation: {}", orient)
            }
        },
        DFR => {
            match orient {
                0 => Face::D,
                1 => Face::F,
                2 => Face::R,
				_ => panic!("Invalid corner orientation: {}", orient)
            }
        },
        DRB => {
            match orient {
                0 => Face::D,
                1 => Face::R,
                2 => Face::B,
				_ => panic!("Invalid corner orientation: {}", orient)
            }
        },
    }
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

/// Get the edge cubie face corresponding to a certain orientation index
///
/// # Panics
/// This function panics if it encounters an invalid orientation
fn get_edge_face(edge: Edge, orient: u8) -> Face {
    use self::Edge::*;
    match edge {
        UB => {
            match orient {
                0 => Face::U,
                1 => Face::B,
                _ => panic!("Invalid edge orientation: {}", orient)
            }
        },
        UR => {
            match orient {
                0 => Face::U,
                1 => Face::R,
                _ => panic!("Invalid edge orientation: {}", orient)
            }
        },
        UF => {
            match orient {
                0 => Face::U,
                1 => Face::F,
                _ => panic!("Invalid edge orientation: {}", orient)
            }
        },
        UL => {
            match orient {
                0 => Face::U,
                1 => Face::L,
                _ => panic!("Invalid edge orientation: {}", orient)
            }
        },
        LB => {
            match orient {
                0 => Face::L,
                1 => Face::B,
                _ => panic!("Invalid edge orientation: {}", orient)
            }
        },
        RB => {
            match orient {
                0 => Face::R,
                1 => Face::B,
                _ => panic!("Invalid edge orientation: {}", orient)
            }
        },
        RF => {
            match orient {
                0 => Face::R,
                1 => Face::F,
                _ => panic!("Invalid edge orientation: {}", orient)
            }
        },
        LF => {
            match orient {
                0 => Face::L,
                1 => Face::F,
                _ => panic!("Invalid edge orientation: {}", orient)
            }
        },
        DB => {
            match orient {
                0 => Face::D,
                1 => Face::B,
                _ => panic!("Invalid edge orientation: {}", orient)
            }
        },
        DR => {
            match orient {
                0 => Face::D,
                1 => Face::R,
                _ => panic!("Invalid edge orientation: {}", orient)
            }
        },
        DF => {
            match orient {
                0 => Face::D,
                1 => Face::F,
                _ => panic!("Invalid edge orientation: {}", orient)
            }
        },
        DL => {
            match orient {
                0 => Face::D,
                1 => Face::L,
                _ => panic!("Invalid edge orientation: {}", orient)
            }
        },
    }
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
///
/// # Panics
/// This function will panic if the input char isn't a valid move, i.e. not one of FRUBLD.
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

/// A face of a cubie (corresponds to a single color sticker on a real cube)
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Face {
    F,
    R,
    U,
    B,
    L,
    D,
}

/// Create a Face from a char. See
/// [http://rubiks.wikia.com/wiki/Notation](http://rubiks.wikia.com/wiki/Notation) 
/// for notation.
///
/// # Panics
/// This function will panic if the input char isn't a valid face, i.e. not one of FRUBLD.
impl From<char> for Face {
    fn from(ch: char) -> Face {
        match ch {
            'F' => Face::F,
            'R' => Face::R,
            'U' => Face::U,
            'B' => Face::B,
            'L' => Face::L,
            'D' => Face::D,
            _ => panic!("Invalid face name: {}", ch),
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

    /// Get a the cubie located in a particular cubicle
    pub fn get(&self, cubicle: Corner) -> Corner {
        *self.map.get(&cubicle).unwrap()
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

    /// Get a the cubie located in a particular cubicle
    pub fn get(&self, cubicle: Edge) -> Edge {
        *self.map.get(&cubicle).unwrap()
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

/// Swap values in an X vector
fn swap_x(values: X, indices: &[u8; 8]) -> X {
    let mut swapped = [0u8; 8];
    let mut curr = 0;
    for &i in indices.iter() {
        swapped[curr] = match i {
            0 => values.0,
            1 => values.1,
            2 => values.2,
            3 => values.3,
            4 => values.4,
            5 => values.5,
            6 => values.6,
            7 => values.7,
            _ => panic!("Invalid index for X tuple"),
        };
        curr += 1;
    }

    X(
        swapped[0],
        swapped[1],
        swapped[2],
        swapped[3],
        swapped[4],
        swapped[5],
        swapped[6],
        swapped[7],
    )
}

/// Add values to an X vector
fn add_x(values: X, addends: &[u8; 8]) -> X {
    X(
        (values.0 + addends[0]) % 3,
        (values.1 + addends[1]) % 3,
        (values.2 + addends[2]) % 3,
        (values.3 + addends[3]) % 3,
        (values.4 + addends[4]) % 3,
        (values.5 + addends[5]) % 3,
        (values.6 + addends[6]) % 3,
        (values.7 + addends[7]) % 3,
    )
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
/// Edge orientation state
struct Y(u8,u8,u8,u8,u8,u8,u8,u8,u8,u8,u8,u8);

/// Swap values in a Y vector
fn swap_y(values: Y, indices: &[u8; 12]) -> Y {
    let mut swapped = [0u8; 12];
    let mut curr = 0;
    for &i in indices.iter() {
        swapped[curr] = match i {
            0 => values.0,
            1 => values.1,
            2 => values.2,
            3 => values.3,
            4 => values.4,
            5 => values.5,
            6 => values.6,
            7 => values.7,
            8 => values.8,
            9 => values.9,
            10 => values.10,
            11 => values.11,
            _ => panic!("Invalid index for Y tuple"),
        };
        curr += 1;
    }

    Y(
        swapped[0],
        swapped[1],
        swapped[2],
        swapped[3],
        swapped[4],
        swapped[5],
        swapped[6],
        swapped[7],
        swapped[8],
        swapped[9],
        swapped[10],
        swapped[11],
    )
}

/// Add values to a Y vector
fn add_y(values: Y, addends: &[u8; 12]) -> Y {
    Y(
        (values.0 + addends[0]) % 2,
        (values.1 + addends[1]) % 2,
        (values.2 + addends[2]) % 2,
        (values.3 + addends[3]) % 2,
        (values.4 + addends[4]) % 2,
        (values.5 + addends[5]) % 2,
        (values.6 + addends[6]) % 2,
        (values.7 + addends[7]) % 2,
        (values.8 + addends[8]) % 2,
        (values.9 + addends[9]) % 2,
        (values.10 + addends[10]) % 2,
        (values.11 + addends[11]) % 2,
    )
}

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

    /// Apply a string of moves to a cube. Notation here: [http://rubiks.wikia.com/wiki/Notation](http://rubiks.wikia.com/wiki/Notation) 
    ///
    /// # Arguments
    /// moves: A move or moves to apply to the cube, e.g. FRUU'R'F'
    ///
    /// # Panics
    /// This function will panic if it encounters an invalid character. Allowed characters are FRUBLD, ', `, \u{2032} (prime), and 2.
    ///
    /// # Example
    /// ```
    /// use rubik::cube::*;
    ///
    /// let mut cube = Cube::new();
    /// // Scramble the cube a little
    /// cube.apply_moves("FUR");
    /// assert!(!cube.is_solved());
    ///
    /// // Undo the moves
    /// cube.apply_moves("R'U'F'");
    /// assert!(cube.is_solved());
    /// ```
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

    /// Apply a single move to the cube. See lemma 11.4 in the Chen paper for details about how x and y are calculated.
    ///
    /// # Example
    /// ```
    /// use rubik::cube::*;
    ///
    /// let mut cube = Cube::new();
    /// cube.apply_move(Move::F);
    /// assert!(!cube.is_solved());
    ///
    /// cube.apply_move(Move::F);
    /// cube.apply_move(Move::F);
    /// cube.apply_move(Move::F);
    /// assert!(cube.is_solved());
    /// ```
    pub fn apply_move(&mut self, m: Move) {
        use self::Move;
        println!("Moving {:?}", m);
        // Compute sigma and tau
        self.sigma.permute(m);
        self.tau.permute(m);

        // Compute X and Y
        let (swap_indices, addends) = match m {
            Move::F => ([5,0,2,3,4,6,1,7], [1,2,0,0,0,2,1,0]),
            Move::R => ([0,6,1,3,4,5,7,2], [0,1,2,0,0,0,2,1]),
            Move::U => ([1,2,3,0,4,5,6,7], [0u8; 8]),
            Move::B => ([0,1,7,2,3,5,6,4], [0,0,1,2,1,0,0,2]),
            Move::L => ([3,1,2,4,5,0,6,7], [2,0,0,1,2,1,0,0]),
            Move::D => ([0,1,2,3,7,4,5,6], [0u8; 8]),
        };
        self.x = swap_x(self.x, &swap_indices);
        self.x = add_x(self.x, &addends);

        let (swap_indices, addends) = match m {
            Move::F => ([0,1,7,3,4,5,2,10,8,9,6,11], [0,0,1,0,0,0,1,1,0,0,1,0]),
            Move::R => ([0,6,2,3,4,1,9,7,8,5,10,11], [0u8; 12]),
            Move::U => ([3,0,1,2,4,5,6,7,8,9,10,11], [0u8; 12]),
            Move::B => ([5,1,2,3,0,8,6,7,4,9,10,11], [1,0,0,0,1,1,0,0,1,0,0,0]),
            Move::L => ([0,1,2,4,11,5,6,3,8,9,10,7], [0u8; 12]),
            Move::D => ([0,1,2,3,4,5,6,7,9,10,11,8], [0u8; 12]),
        };
        self.y = swap_y(self.y, &swap_indices);
        self.y = add_y(self.y, &addends);
    }

    /// Determine whether the cube is in the solved state
    pub fn is_solved(&self) -> bool {
        self.sigma == CornerPermutation::default() &&
        self.tau == EdgePermutation::default() &&
        self.x == X::default() &&
        self.y == Y::default()
    }

    /// Get the cubie faces visible on one face of the cube. Faces are stored out
    /// in the array such that the top row of the face is in the first three
    /// elements, the next row is stored in the next three elements, and the
    /// bottom row is stored in the last three elements.
    ///
    /// # Example
    /// ```
    /// use rubik::cube::*;
    ///
    /// let mut cube = Cube::new();
    /// // On a solved cube, every cubie face on the up face should also be an up face
    /// assert_eq!(cube.get_face(Face::U), [Face::U; 9]);
    /// ```
    pub fn get_face(&self, face: Face) -> [Face; 9] {
        // Get the cubie in each cubicle of this face
        // The center is always the same
        let center = face;

        // Find the corners located in this face clockwise from top left
        use self::Corner::*;
        let corners = match face {
            Face::F => [UFL, URF, DFR, DLF],
            Face::R => [URF, UBR, DRB, DBL],
            Face::U => [ULB, UBR, URF, UFL],
            Face::B => [UBR, ULB, DBL, DRB],
            Face::L => [ULB, UFL, DLF, DBL],
            Face::D => [DBL, DRB, DLF, DFR],
        };

        // Find the edges in the face clockwise from the top
        use self::Edge::*;
        let edges = match face {
            Face::F => [UF, RF, DF, LF],
            Face::R => [UR, RB, DR, RF],
            Face::U => [UB, UR, UF, UL],
            Face::B => [UB, LB, DB, RB],
            Face::L => [UL, LF, DL, LB],
            Face::D => [DB, DL, DF, DR],
        };

        // Get the corner and edge cubies in each cubicle of interest
        let corner_indices = [
            self.sigma.get(corners[0]),
            self.sigma.get(corners[1]),
            self.sigma.get(corners[2]),
            self.sigma.get(corners[3])
        ];
        let edge_indices = [
            self.tau.get(edges[0]),
            self.tau.get(edges[1]),
            self.tau.get(edges[2]),
            self.tau.get(edges[3])
        ];

        // Now get the orientations for the cubies we care about
        let corner_orientations = [
            self.get_corner_orientation(corners[0]),
            self.get_corner_orientation(corners[1]),
            self.get_corner_orientation(corners[2]),
            self.get_corner_orientation(corners[3])
        ];
        let edge_orientations = [
            self.get_edge_orientation(edges[0]),
            self.get_edge_orientation(edges[1]),
            self.get_edge_orientation(edges[2]),
            self.get_edge_orientation(edges[3])
        ];

        let mut corner_faces = [Face::U; 4];
        let mut edge_faces = [Face::U; 4];

        for i in 0..4 {
            corner_faces[i] = get_corner_face(corner_indices[i],corner_orientations[i]);
            edge_faces[i] = get_edge_face(edge_indices[i],edge_orientations[i]);
        }

        println!("{:?}", corner_indices);
        println!("{:?}", corner_orientations);
        println!("{:?}", edge_indices);
        println!("{:?}", edge_orientations);

        [corner_faces[0], edge_faces[0], corner_faces[1],
        edge_faces[3], center, edge_faces[1],
        corner_faces[3], edge_faces[2], corner_faces[2]]

    }

    /// Get the orientation of a corner cubicle
    fn get_corner_orientation(&self, c: Corner) -> u8 {
        use self::Corner::*;
        match c {
            UFL => self.x.0,
            URF => self.x.1,
            UBR => self.x.2,
            ULB => self.x.3,
            DBL => self.x.4,
            DLF => self.x.5,
            DFR => self.x.6,
            DRB => self.x.7,
        }
    }

    /// Get the orientation of an edge cubicle
    fn get_edge_orientation(&self, e: Edge) -> u8 {
        use self::Edge::*;
        match e {
            UB => self.y.0,
            UR => self.y.1,
            UF => self.y.2,
            UL => self.y.3,
            LB => self.y.4,
            RB => self.y.5,
            RF => self.y.6,
            LF => self.y.7,
            DB => self.y.8,
            DR => self.y.9,
            DF => self.y.10,
            DL => self.y.11,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_default_solved() {
        assert!(Cube::new().is_solved());
    }

    #[test]
    fn test_permute_bottom_corners() {
        let mut cube = Cube::new();
        cube.apply_moves("R'UR'D2RU'R'D2R2");
        cube.apply_moves("R2D2RUR'D2RU'R");

        assert!(cube.is_solved());
    }

    #[test]
    fn test_f_fprime() {
        let mut cube = Cube::new();
        cube.apply_moves("FF'");
        assert!(cube.is_solved());
    }

    #[test]
    fn test_r_rprime() {
        let mut cube = Cube::new();
        cube.apply_moves("RR'");
        assert!(cube.is_solved());
    }

    #[test]
    fn test_u_uprime() {
        let mut cube = Cube::new();
        cube.apply_moves("UU'");
        assert!(cube.is_solved());
    }

    #[test]
    fn test_b_bprime() {
        let mut cube = Cube::new();
        cube.apply_moves("BB'");
        assert!(cube.is_solved());
    }

    #[test]
    fn test_l_lprime() {
        let mut cube = Cube::new();
        cube.apply_moves("LL'");
        assert!(cube.is_solved());
    }

    #[test]
    fn test_d_dprime() {
        let mut cube = Cube::new();
        cube.apply_moves("DD'");
        assert!(cube.is_solved());
    }

    #[test]
    fn test_move_and_unmove() {
        let mut cube = Cube::new();
        cube.apply_moves("FRUBLD");
        cube.apply_moves("D'L'B'U'R'F'");

        assert!(cube.is_solved());
    }

    #[test]
    fn test_solved_face_u() {
        assert_eq!(Cube::new().get_face(Face::U), [Face::U; 9]);
    }

    #[test]
    fn test_solved_face_r() {
        assert_eq!(Cube::new().get_face(Face::R), [Face::R; 9]);
    }

    #[test]
    fn test_solved_face_f() {
        assert_eq!(Cube::new().get_face(Face::F), [Face::F; 9]);
    }

    #[test]
    fn test_solved_face_d() {
        assert_eq!(Cube::new().get_face(Face::D), [Face::D; 9]);
    }

    #[test]
    fn test_solved_face_l() {
        assert_eq!(Cube::new().get_face(Face::L), [Face::L; 9]);
    }

    #[test]
    fn test_solved_face_b() {
        assert_eq!(Cube::new().get_face(Face::B), [Face::B; 9]);
    }
}
