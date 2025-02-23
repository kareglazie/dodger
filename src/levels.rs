#[derive(Clone)]
/// Represents a game level with an image template and falling speed.
pub struct Level {
    pub image_template: &'static str,
    pub fall_speed: f32,
}

/// Returns a vector of predefined game levels.
///
/// Each level is characterized by an image template and a falling speed.
pub fn get_levels() -> Vec<Level> {
    vec![
        Level {
            image_template: "/Level1",
            fall_speed: 2.5,
        },
        Level {
            image_template: "/Level2",
            fall_speed: 3.0,
        },
        Level {
            image_template: "/Level3",
            fall_speed: 3.5,
        },
        Level {
            image_template: "/Level4",
            fall_speed: 4.0,
        },
    ]
}
