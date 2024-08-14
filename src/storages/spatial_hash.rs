use std::collections::HashMap;
use std::hash::Hash;
use std::iter;
use num::Integer;
use crate::physical_aabb::PhysicalAABB;
use crate::storages::Storage;

pub struct SpatialHash<IntType: Integer + Hash + Clone> {
	chunk_size: IntType,
	chunks: HashMap<(IntType, IntType), Vec<PhysicalAABB<IntType>>>
}
impl<IntType: Integer + Hash + Clone> SpatialHash<IntType> {
	pub fn new(chunk_size: IntType) -> Self {
		SpatialHash {
			chunk_size,
			chunks: HashMap::new()
		}
	}
	pub fn insert(&mut self, aabb: PhysicalAABB<IntType>) {
		self.chunks.entry(Self::get_chunk_of(&aabb, &self.chunk_size)).or_insert_with(|| vec![]).push(aabb);
	}
	pub fn get_chunk_of(&self, aabb: &PhysicalAABB<IntType>) -> (IntType, IntType) {
		(aabb.x.div_floor(&self.chunk_size), aabb.y.div_floor(&self.chunk_size))
	}
}
impl<IntType: Integer + Hash + Clone + 'static> Storage<IntType> for SpatialHash<IntType> {
	fn get_surrounding_aabbs(&self, rect: PhysicalAABB<IntType>) -> impl Iterator<Item=&PhysicalAABB<IntType>> {
		let mut aabbs: Box<dyn Iterator<Item=&PhysicalAABB<IntType>>> = Box::new(iter::empty());
		let (x, y) = self.get_chunk_of(&rect);
		for (dx, dy) in [
			(IntType::one(), IntType::one()),
			(IntType::zero(), IntType::one()),
			(IntType::zero() - IntType::one(), IntType::one()),
			(IntType::one(), IntType::zero()),
			(IntType::zero() - IntType::one(), IntType::zero()),
			(IntType::one(), IntType::zero() - IntType::one()),
			(IntType::zero(), IntType::zero() - IntType::one()),
			(IntType::zero() - IntType::one(), IntType::zero() - IntType::one()),
		] {
			aabbs = Box::new(aabbs.chain(self.chunks.get(&(x.clone() + dx, y.clone() + dy)).unwrap_or(&Vec::new())));
		}
		aabbs
	}
}