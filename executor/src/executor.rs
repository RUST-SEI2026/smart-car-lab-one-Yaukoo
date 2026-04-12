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
        for cmd in cmds.chars(){
            match cmd{
                'M' => match self.pose.heading{
                    'N' => self.pose.y += 1,
                    'E' => self.pose.x += 1, 
                    'S' => self.pose.y -= 1, 
                    'W' => self.pose.x -= 1,              
                    _ => (),
                },
                'R' => match self.pose.heading{
                    'N' => self.pose.heading = 'E',
                    'E' => self.pose.heading = 'S', 
                    'S' => self.pose.heading = 'W', 
                    'W' => self.pose.heading = 'N',              
                    _ => (),
                },
                'L' => match self.pose.heading{
                    'N' => self.pose.heading = 'W',
                    'E' => self.pose.heading = 'N', 
                    'S' => self.pose.heading = 'E', 
                    'W' => self.pose.heading = 'S',              
                    _ => (),
                },
                _ => (),
            }
        }
    }

    pub fn query(&self) -> Pose {
        self.pose
    }
}
