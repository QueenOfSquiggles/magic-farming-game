use avian3d::prelude::PhysicsLayer;

#[derive(PhysicsLayer)]
pub enum GameLayers {
    Default,
    Player,
    Interactable,
}
