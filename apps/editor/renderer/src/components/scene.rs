

use bevy::prelude::*;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub enum BBObject {
    // Scene Object type for a vector element
    #[default]
    Vector,
}
