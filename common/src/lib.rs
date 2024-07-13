use serde_derive::{Deserialize, Serialize};

pub mod team {
    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    pub enum Team {
        WHITE,
        BLACK,
    }
}