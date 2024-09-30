use bevy::color::Color;

pub trait CustomColors {
    const RED: Color;
    const ORANGE: Color;
    const YELLOW: Color;
    const GREEN: Color;
    const BLUE: Color;
    const PURPLE: Color;

    const FG_NORMAL: Color;
    const FG_HOVER: Color;
    const BG: Color;

    const HUD: Color;
    const HUD_HOVER: Color;

    const TRANSPARENT: Color;
}

impl CustomColors for Color {
    const RED: Color = Color::srgb(0.8, 0.1, 0.1);
    const ORANGE: Color = Color::srgb(0.8, 0.4, 0.1);
    const YELLOW: Color = Color::srgb(0.8, 0.8, 0.1);
    const GREEN: Color = Color::srgb(0.1, 0.8, 0.1);
    const BLUE: Color = Color::srgb(0.1, 0.1, 0.8);
    const PURPLE: Color = Color::srgb(0.8, 0.1, 0.8);

    const FG_NORMAL: Color = Color::srgb(0.1, 0.1, 0.1);
    const FG_HOVER: Color = Color::srgb(0.15, 0.15, 0.15);
    const BG: Color = Color::srgb(0.2, 0.2, 0.2);

    const HUD: Color = Color::srgba(0.0, 0.0, 0.0, 0.65);
    const HUD_HOVER: Color = Color::srgba(0.0, 0.0, 0.0, 0.5);

    const TRANSPARENT: Color = Color::srgba(0.0, 0.0, 0.0, 0.0);
}
