
pub mod team {
    use serde_derive::{Deserialize, Serialize}; 

    #[derive(Debug, Serialize, Deserialize)]
    pub enum Team {
        WHITE,
        BLACK,
    }
}