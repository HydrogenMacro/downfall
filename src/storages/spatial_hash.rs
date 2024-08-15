use std::collections::HashMap;
use std::hash::Hash;
use std::iter;
use crate::Int;
use crate::body::Body;
use crate::storages::Storage;

pub struct SpatialHash {
	chunk_size: Int,
	chunks: HashMap<(Int, Int), Vec<Body>>
}
impl SpatialHash {
	pub fn new(chunk_size: Int) -> Self {
		SpatialHash {
			chunk_size,
			chunks: HashMap::new()
		}
	}
	pub fn insert(&mut self, aabb: Body) {
		self.chunks.entry(self.get_chunk_of(&aabb)).or_insert_with(|| vec![]).push(aabb);
	}
	pub fn get_chunk_of(&self, aabb: &Body) -> (Int, Int) {
		(aabb.x / self.chunk_size, aabb.y / self.chunk_size)
	}
}
impl Storage for SpatialHash {
	fn get_surrounding_aabbs(&self, rect: Body) -> impl Iterator<Item=&Body> {
		let mut aabbs: Box<dyn Iterator<Item=&Body>> = Box::new(iter::empty());
		let (x, y) = self.get_chunk_of(&rect);
		for dx in -1..=1 {
			for dy in -1..=1 {
				if dx == 0 && dy == 0 {
					continue;
				}
				if let Some(chunk_contents) = self.chunks.get(&(x.clone() + dx, y.clone() + dy)) {
					aabbs = Box::new(aabbs.chain(chunk_contents.iter()));
				}
			}
		}
		aabbs
	}
}