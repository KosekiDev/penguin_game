use bevy::prelude::*;

#[derive(Resource, Debug)]
pub struct PlayerScore(pub u32);

impl PlayerScore {
    pub fn add(&mut self, points: u32) {
        self.0 += points;
    }
}
