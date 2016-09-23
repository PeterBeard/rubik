// This file is part of Rubik.
// Copyright Peter Beard, licensed under the GPLv3. See LICENSE for details.
//
//! Algorithms for solving Rubik's cubes
use super::cube::{Cube, Move};

/// Trait for things that can solve Rubik's cubes
pub trait Solver {
    /// Calculate a sequence of moves that puts the cube in the solved state
    fn find_solution(&mut self, cube: &Cube) -> Vec<Move>;
}

/// Solver that doesn't do anything
///
/// # Example
/// ```
/// use rubik::cube::Cube;
/// use rubik::solver::{Solver, NullSolver};
///
/// let mut c = Cube::new();
/// let mut ns = NullSolver::new();
///
/// assert_eq!(c.solve(&mut ns), vec![]);
/// ```
pub struct NullSolver;

impl NullSolver {
    pub fn new() -> NullSolver {
        NullSolver
    }
}

impl Solver for NullSolver {
    fn find_solution(&mut self, _: &Cube) -> Vec<Move> {
        vec![]
    }
}

/// Solver that uses a simple iterative deepening algorithm
///
/// This algorithm is very slow and probably won't halt in a reasonable time for 
/// most cubes
///
/// # Example
/// ```
/// use rubik::cube::Cube;
/// use rubik::solver::IDSolver;
///
/// let mut c = Cube::new();
/// let mut ids = IDSolver::new();
///
/// c.apply_moves("F'U'D'");
/// println!("{:?}", c.solve(&mut ids));
///
/// assert!(c.is_solved());
/// ```
pub struct IDSolver {
    max_depth: u8,
}

impl IDSolver {
    /// Create a new solver with the default maximum depth of 26
    /// (all cubes are solveable in at most 26 moves)
    pub fn new() -> IDSolver {
        IDSolver {
            max_depth: 26u8,
        }
    }

    /// Create a solver with the given maximum depth (max number of moves)
    pub fn with_max_depth(d: u8) -> IDSolver {
        IDSolver {
            max_depth: d,
        }
    }
}

impl Solver for IDSolver {
    fn find_solution(&mut self, cube: &Cube) -> Vec<Move> {
        let mut current_solution: Option<Vec<Move>> = None;
        let mut current_depth = 1;

        // A solved cube requires zero moves to solve
        if !cube.is_solved() {
            // Look until we find a solution or run out of moves
            while current_depth <= self.max_depth && current_solution.is_none() {
                current_solution = dbsearch(cube, current_depth);
                current_depth += 1;
            }
        }
        // Return no moves if there's no solution within the max depth
        if let Some(s) = current_solution {
            s
        } else {
            vec![]
        }
    }

}

/// Depth-bounded search for a solution
fn dbsearch(start: &Cube, maxdepth: u8) -> Option<Vec<Move>> {
    // Zero means we're at the max depth
    if maxdepth == 0 {
        return None;
    }

    let possible_moves = [
        Move::F,
        Move::R,
        Move::U,
        Move::B,
        Move::L,
        Move::D,
        Move::FPrime,
        Move::RPrime,
        Move::UPrime,
        Move::BPrime,
        Move::LPrime,
        Move::DPrime,
    ];
    let mut moves = Vec::new();
    // Try every possible move and see where we get
    for &m in &possible_moves {
        let mut s = start.clone();
        s.apply_move(m);
        moves.push(m);

        if s.is_solved() {
            break;
        }

        if let Some(ms) = dbsearch(&s, maxdepth - 1) {
            moves.append(&mut ms.clone());
            break;
        } else {
            moves.pop();
        }
    }
    if moves.len() > 0 {
        Some(moves)
    } else {
        None
    }
}
