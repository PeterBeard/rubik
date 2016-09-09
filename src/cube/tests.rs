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

#[test]
fn test_move_f() {
    use super::Face::*;

    let mut cube = Cube::new();
    cube.apply_moves("F");
    assert_eq!(cube.get_face(F), [F; 9]);
    assert_eq!(cube.get_face(R), [U,R,R,U,R,R,U,R,R]);
    assert_eq!(cube.get_face(U), [U,U,U,U,U,U,L,L,L]);
    assert_eq!(cube.get_face(B), [B; 9]);
    assert_eq!(cube.get_face(L), [L,L,D,L,L,D,L,L,D]);
    assert_eq!(cube.get_face(D), [R,R,R,D,D,D,D,D,D]);
}

#[test]
fn test_move_r() {
    use super::Face::*;

    let mut cube = Cube::new();
    cube.apply_moves("R");
    assert_eq!(cube.get_face(F), [F,F,D,F,F,D,F,F,D]);
    assert_eq!(cube.get_face(R), [R; 9]);
    assert_eq!(cube.get_face(U), [U,U,F,U,U,F,U,U,F]);
    assert_eq!(cube.get_face(B), [U,B,B,U,B,B,U,B,B]);
    assert_eq!(cube.get_face(L), [L; 9]);
    assert_eq!(cube.get_face(D), [D,D,B,D,D,B,D,D,B]);
}

#[test]
fn test_move_u() {
    use super::Face::*;

    let mut cube = Cube::new();
    cube.apply_moves("U");
    assert_eq!(cube.get_face(F), [R,R,R,F,F,F,F,F,F]);
    assert_eq!(cube.get_face(R), [B,B,B,R,R,R,R,R,R]);
    assert_eq!(cube.get_face(U), [U; 9]);
    assert_eq!(cube.get_face(B), [L,L,L,B,B,B,B,B,B]);
    assert_eq!(cube.get_face(L), [F,F,F,L,L,L,L,L,L]);
    assert_eq!(cube.get_face(D), [D; 9]);
}

#[test]
fn test_move_b() {
    use super::Face::*;

    let mut cube = Cube::new();
    cube.apply_moves("B");
    assert_eq!(cube.get_face(F), [F; 9]);
    assert_eq!(cube.get_face(R), [R,R,D,R,R,D,R,R,D]);
    assert_eq!(cube.get_face(U), [R,R,R,U,U,U,U,U,U]);
    assert_eq!(cube.get_face(B), [B; 9]);
    assert_eq!(cube.get_face(L), [U,L,L,U,L,L,U,L,L]);
    assert_eq!(cube.get_face(D), [D,D,D,D,D,D,L,L,L]);
}

#[test]
fn test_move_l() {
    use super::Face::*;

    let mut cube = Cube::new();
    cube.apply_moves("L");
    assert_eq!(cube.get_face(F), [U,F,F,U,F,F,U,F,F]);
    assert_eq!(cube.get_face(R), [R; 9]);
    assert_eq!(cube.get_face(U), [B,U,U,B,U,U,B,U,U]);
    assert_eq!(cube.get_face(B), [B,B,D,B,B,D,B,B,D]);
    assert_eq!(cube.get_face(L), [L; 9]);
    assert_eq!(cube.get_face(D), [F,D,D,F,D,D,F,D,D]);
}

#[test]
fn test_move_d() {
    use super::Face::*;

    let mut cube = Cube::new();
    cube.apply_moves("D");
    assert_eq!(cube.get_face(F), [F,F,F,F,F,F,L,L,L]);
    assert_eq!(cube.get_face(R), [R,R,R,R,R,R,F,F,F]);
    assert_eq!(cube.get_face(U), [U; 9]);
    assert_eq!(cube.get_face(B), [B,B,B,B,B,B,R,R,R]);
    assert_eq!(cube.get_face(L), [L,L,L,L,L,L,B,B,B]);
    assert_eq!(cube.get_face(D), [D; 9]);
}

#[test]
fn test_moved_faces() {
    use super::Face::*;

    let mut cube = Cube::new();
    cube.apply_moves("R2U'FLB2");

    let faces = [
        cube.get_face(U),
        cube.get_face(R),
        cube.get_face(F),
        cube.get_face(D),
        cube.get_face(L),
        cube.get_face(B),
    ];

    assert_eq!(faces[0], [U,D,B,B,U,U,R,L,B]);
    assert_eq!(faces[1], [U,F,U,U,R,L,U,R,L]);
    assert_eq!(faces[2], [D,F,L,U,F,L,L,B,L]);
    assert_eq!(faces[3], [F,R,F,F,D,U,D,D,B]);
    assert_eq!(faces[4], [R,L,F,R,L,B,B,D,D]);
    assert_eq!(faces[5], [R,B,F,D,B,F,D,R,R]);
}
