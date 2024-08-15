use std::hash::Hash;
use num::Integer;
use crate::storages::{SpatialHash, Storage};

pub struct World {
	storage: SpatialHash
}
impl World {
	pub fn new() -> Self {
		World {
			storage: SpatialHash::new(),
		}
	}
}