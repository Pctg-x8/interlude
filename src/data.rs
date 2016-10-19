// Interlude: Data Structures

/// x, y, z, w
#[repr(C)] #[derive(Clone)] pub struct Position(pub f32, pub f32, pub f32, pub f32);
/// x, y, u, v
#[repr(C)] #[derive(Clone)] pub struct PosUV(pub f32, pub f32, pub f32, pub f32);

// C(GLSL)-complatible Vector Types
pub type CVector4 = [f32; 4];
pub type CVector2 = [f32; 2];
pub type CMatrix4 = [CVector4; 4];
