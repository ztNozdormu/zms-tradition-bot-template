use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TradeResponse {
    robot: Trade,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Trade {
    pub username: String,
    pub email: String,
    pub token: String,
}

impl From<()> for TradeResponse {
    fn from(x: ()) -> Self {
        let () = x;
        Self {
            robot: Trade {
                username: "robot".to_string(),
                email: "email".to_string(),
                token: "token".to_string(),
            },
        }
    }
}
