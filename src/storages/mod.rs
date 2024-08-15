mod spatial_hash;

use std::hash::Hash;
use num::Integer;
pub use crate::body::Body;
pub use spatial_hash::*;

pub trait Storage {
	fn get_surrounding_aabbs(&self, rect: Body) -> impl Iterator<Item=&Body>;
}
