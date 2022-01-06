mod line;
mod triangle;

pub type Vec2i = (u64, u64);
pub use line::{line, line1, line2};
pub use triangle::{triangle, triangle1};
