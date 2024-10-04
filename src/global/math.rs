use bevy::math::Vec2;

pub fn pt_rect_collision(pt: Vec2, rect_mid: Vec2, rect_dims: Vec2) -> bool {
    pt.x >= rect_mid.x - rect_dims.x / 2.0
        && pt.x <= rect_mid.x + rect_dims.x / 2.0
        && pt.y >= rect_mid.y - rect_dims.y / 2.0
        && pt.y <= rect_mid.y + rect_dims.y / 2.0
}
