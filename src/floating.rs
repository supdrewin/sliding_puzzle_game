use bevy::prelude::*;

pub struct Floating;

impl Floating {
    fn setup() {

        // add settings

        // add about
    }

    fn update() {
        // pop-up box
    }
}

impl Plugin for Floating {
    fn build(&self, app: &mut App) {
        app.add_system(Self::setup).add_system(Self::update);
    }
}
