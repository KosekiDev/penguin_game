use bevy::prelude::*;

#[derive(Resource, Debug)]
pub struct PlayerScore(pub u32);

impl PlayerScore {
    pub fn add(&mut self, points: u32) {
        self.0 += points;
    }
}

#[derive(Resource, Debug, Default)]
pub struct PlayerStats {
    pub misses: u32,
    pub combos_count: u32,
}

#[derive(Component)]
pub struct PlayerCombosChanged;
