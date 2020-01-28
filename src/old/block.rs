use crate::assets::Texture;
use crate::collider::ColliderEdge;

pub struct Block {
    pub texture:  Texture,
    pub collider: ColliderEdge,
}