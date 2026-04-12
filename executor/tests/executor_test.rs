#[test]
fn test_move_north() {
    use executor::{Executor, Pose};
    let mut exec = Executor::with_pose(Pose::default());
    exec.execute("M");
    let res = exec.query();
    assert_eq!(res.y, 1);
    assert!(res.heading == 'N');
}

#[test]
fn test_move_east() {
    use executor::{Executor, Pose};
    let mut exec = Executor::with_pose(Pose::new(0, 0, 'E'));
    exec.execute("M");
    let res = exec.query();
    assert_eq!(res.x, 1);
    assert!(res.heading == 'E');
}

#[test]
fn test_move_south() {
    use executor::{Executor, Pose};
    let mut exec = Executor::with_pose(Pose::new(0, 0, 'S'));
    exec.execute("M");
    let res = exec.query();
    assert_eq!(res.y, -1);
    assert!(res.heading == 'S');
}

#[test]
fn test_move_west() {
    use executor::{Executor, Pose};
    let mut exec = Executor::with_pose(Pose::new(0, 0, 'W'));
    exec.execute("M");
    let res = exec.query();
    assert_eq!(res.x, -1);
    assert!(res.heading == 'W');
}

#[test]
fn test_turn_right() {
    use executor::{Executor, Pose};
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

