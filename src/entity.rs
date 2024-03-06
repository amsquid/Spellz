pub mod vector;

pub struct Entity {
    position: vector::Vector2,
    velocity: vector::Vector2,
    hp: i32,
}