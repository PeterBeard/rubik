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
