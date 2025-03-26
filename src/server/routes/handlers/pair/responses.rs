use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TradePairResponse {
    robot: TradePair,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TradePair {
    pub username: String,
    pub email: String,
    pub token: String,
}

impl From<()> for TradePairResponse {
    fn from(x: ()) -> Self {
        let () = x;
        Self {
            robot: TradePair {
                username: "robot".to_string(),
                email: "email".to_string(),
                token: "token".to_string(),
            },
        }
    }
}
