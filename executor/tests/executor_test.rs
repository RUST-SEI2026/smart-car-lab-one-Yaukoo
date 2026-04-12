use executor::{Executor, Pose};

#[test]
fn test_move_north() {
    let mut exec = Executor::with_pose(Pose::default());
    exec.execute("M");
    let res = exec.query();
    assert_eq!(res.y, 1);
    assert!(res.heading == 'N');
}

#[test]
fn test_move_east() {
    let mut exec = Executor::with_pose(Pose::new(0, 0, 'E'));
    exec.execute("M");
    let res = exec.query();
    assert_eq!(res.x, 1);
    assert!(res.heading == 'E');
}

#[test]
fn test_move_south() {
    let mut exec = Executor::with_pose(Pose::new(0, 0, 'S'));
    exec.execute("M");
    let res = exec.query();
    assert_eq!(res.y, -1);
    assert!(res.heading == 'S');
}

#[test]
fn test_move_west() {
    let mut exec = Executor::with_pose(Pose::new(0, 0, 'W'));
    exec.execute("M");
    let res = exec.query();
    assert_eq!(res.x, -1);
    assert!(res.heading == 'W');
}

#[test]
fn test_turn_right() {
    let mut exec = Executor::with_pose(Pose::default());
    exec.execute("R");
    let res = exec.query();
    assert_eq!(res.heading, 'E');
    exec.execute("R");
    let res = exec.query();
    assert_eq!(res.heading, 'S');
    exec.execute("R");
    let res = exec.query();
    assert_eq!(res.heading, 'W');
    exec.execute("R");
    let res = exec.query();
    assert_eq!(res.heading, 'N');
}

#[test]
fn test_turn_left() {
    let mut exec = Executor::with_pose(Pose::default());
    exec.execute("L");
    let res = exec.query();
    assert_eq!(res.heading, 'W');
    exec.execute("L");
    let res = exec.query();
    assert_eq!(res.heading, 'S');
    exec.execute("L");
    let res = exec.query();
    assert_eq!(res.heading, 'E');
    exec.execute("L");
    let res = exec.query();
    assert_eq!(res.heading, 'N');
}

#[test]
fn test_zigzag_path() {
    let mut exec = Executor::with_pose(Pose::default());
    exec.execute("MRMLM");
    let res = exec.query();
    assert_eq!(res.x, 1);
    assert_eq!(res.y, 2);
    assert_eq!(res.heading, 'N');
}

#[test]
fn test_square_loop() {
    let mut exec = Executor::with_pose(Pose::default());
    exec.execute("MMR MMR MMR MMR");
    let res = exec.query();
    assert_eq!(res.x, 0);
    assert_eq!(res.y, 0);
    assert_eq!(res.heading, 'N');
}

#[test]
fn test_complex_path() {
    let mut exec = Executor::with_pose(Pose::default());
    exec.execute("MMRMLMRRMLMMMRMLM");
    let res = exec.query();
    assert_eq!(res.x, 5);
    assert_eq!(res.y, 1);
    assert_eq!(res.heading, 'E');
}

#[test]
fn test_invalid_commands() {
    let mut exec = Executor::with_pose(Pose::default());
    exec.execute("M 1M MX@");
    let res = exec.query();
    assert_eq!(res.x, 0);
    assert_eq!(res.y, 3);
    assert_eq!(res.heading, 'N');
}

#[test]
fn test_empty_command() {
    let mut exec = Executor::with_pose(Pose::default());
    exec.execute("");
    let res = exec.query();
    assert_eq!(res.x, 0);
    assert_eq!(res.y, 0);
    assert_eq!(res.heading, 'N');
}

#[test]
fn test_negative_coordinates() {
    let mut exec = Executor::with_pose(Pose::new(-10, -10, 'S'));
    exec.execute("MMRML");
    assert_eq!(exec.query().y, -12);
    assert_eq!(exec.query().x, -11);
    assert_eq!(exec.query().heading, 'S');
}

#[test]
fn test_lowercase_commands() {
    let mut exec = Executor::with_pose(Pose::default());
    exec.execute("mrm");
    let res = exec.query();
    assert_eq!(res.x, 0);
    assert_eq!(res.y, 0);
    assert_eq!(res.heading, 'N');
}
