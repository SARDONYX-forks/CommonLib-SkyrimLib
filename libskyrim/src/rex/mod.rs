//! Shared REX ("RE Extensions") support types used by translated RE layouts.

pub mod d3d;
pub mod enum_set;
pub mod openvr;
pub mod ps4;
pub mod w32;

pub use d3d::*;
pub use enum_set::*;
pub use openvr::*;
pub use ps4::*;
/// COM interfaces live in the standalone `rex_com` crate.
pub use rex_com as com;
pub use rex_com::*;
pub use w32::*;
