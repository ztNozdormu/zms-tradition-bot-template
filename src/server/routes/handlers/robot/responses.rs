use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RobotResponse {
    robot: Robot,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Robot {
    pub username: String,
    pub email: String,
    pub token: String,
}

impl From<()> for RobotResponse {
    fn from(x: ()) -> Self {
        let () = x;
        Self {
            robot: Robot {
                username: "robot".to_string(),
                email: "email".to_string(),
                token: "token".to_string(),
            },
        }
    }
}
