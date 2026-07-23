#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Pose {
    pub x: i32,
    pub y: i32,
    pub heading: char,
}

impl Pose {
    pub fn new(x: i32, y: i32, heading: char) -> Self {
        Pose { x, y, heading }
    }
}

impl Default for Pose {
    fn default() -> Self {
        Pose {
            x: 0,
            y: 0,
            heading: 'N',
        }
    }
}

pub struct Executor {
    pose: Pose,
}

impl Executor {
    pub fn with_pose(pose: Pose) -> Self {
        Executor { pose }
    }

    pub fn execute(&mut self, cmds: &str) {
        let mut x = self.pose.x;
        let mut y = self.pose.y;
        let mut direction = match self.pose.heading {
            'N' => 0usize,
            'E' => 1,
            'S' => 2,
            'W' => 3,
            _ => return,
        };

        for cmd in cmds.bytes() {
            match cmd {
                b'M' => {
                    const STEPS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
                    let (dx, dy) = STEPS[direction];
                    x += dx;
                    y += dy;
                }
                b'R' => direction = (direction + 1) & 3,
                b'L' => direction = (direction + 3) & 3,
                _ => (),
            }
        }

        const HEADINGS: [char; 4] = ['N', 'E', 'S', 'W'];
        self.pose = Pose {
            x,
            y,
            heading: HEADINGS[direction],
        };
    }

    pub fn query(&self) -> Pose {
        self.pose
    }
}
