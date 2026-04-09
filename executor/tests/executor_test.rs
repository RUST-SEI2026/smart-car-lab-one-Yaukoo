#[test]
fn test_move_north() {
    use executor::{Executor, Pose};
    let mut exec = Executor::with_pose(Pose::default());
    exec.execute("M");
    let res = exec.query();
    assert_eq!(res.y, 1);
    assert!(res.heading == 'N');
}