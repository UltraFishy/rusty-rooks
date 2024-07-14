
pub mod team {
    use serde_derive::{Deserialize, Serialize}; 

    #[derive(Debug, Serialize, Deserialize)]
    pub enum Team {
        WHITE,
        BLACK,
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Teams {
        white_ : u8,
        black_ : u8,
        team_cap : u8
    }

    impl Teams {
        pub fn from(white_ : u8, black_ : u8, team_cap: u8) -> Self {
            Teams { white_, black_, team_cap }
        }
        pub fn new(cap: u8) -> Self {
            Teams {
                white_ : 0,
                black_ : 0,
                team_cap : cap
            }
        }
        pub fn add_team(&mut self, team : Team) -> () {
            match team {
                Team::WHITE => self.add_white(),
                Team::BLACK => self.add_black()
            }
        }
        fn add_white(&mut self) -> () {
            if self.white_ != self.team_cap {
                self.white_ += 1;
            }
        }
        fn add_black(&mut self) -> () {
            if self.black_ != self.team_cap {
                self.black_ += 1;
            }
        }

        // fn ready(&self) -> bool {
        //     if self.black_ == self.team_cap && self.white_== self.team_cap {
        //         return true;
        //     }
        //     return false;
        // } 
    }
}

