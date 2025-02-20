#[derive(Clone)]
pub struct Level {
    pub image_template: &'static str,
    pub fall_speed: f32,
}

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
