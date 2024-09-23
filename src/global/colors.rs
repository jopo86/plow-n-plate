use bevy::color::Color;

pub trait CustomColors {
    const RED: Color;
    const ORANGE: Color;
    const YELLOW: Color;
    const GREEN: Color;
    const BLUE: Color;
    const PURPLE: Color;

    const GRAY_0: Color;
    const GRAY_1: Color;
    const GRAY_2: Color;

    const TRANSPARENT: Color;
}

impl CustomColors for Color {
    const RED: Color = Color::srgb(0.8, 0.1, 0.1);
    const ORANGE: Color = Color::srgb(0.8, 0.4, 0.1);
    const YELLOW: Color = Color::srgb(0.8, 0.8, 0.1);
    const GREEN: Color = Color::srgb(0.1, 0.8, 0.1);
    const BLUE: Color = Color::srgb(0.1, 0.1, 0.8);
    const PURPLE: Color = Color::srgb(0.8, 0.1, 0.8);

    const GRAY_0: Color = Color::srgb(0.1, 0.1, 0.1);
    const GRAY_1: Color = Color::srgb(0.15, 0.15, 0.15);
    const GRAY_2: Color = Color::srgb(0.2, 0.2, 0.2);

    const TRANSPARENT: Color = Color::srgba(0.0, 0.0, 0.0, 0.0);
}
