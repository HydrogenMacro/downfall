pub mod world;
pub mod storages;
pub mod body;
pub mod prelude {
	pub use crate::world::*;
	pub use crate::storages::*;
	pub use crate::body::*;
}

pub type Int = i64;