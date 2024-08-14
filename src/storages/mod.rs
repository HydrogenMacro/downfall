mod spatial_hash;

use num::Integer;
use crate::physical_aabb::PhysicalAABB;

pub trait Storage<IntType: Integer + 'static> {
	fn get_surrounding_aabbs(&self, rect: PhysicalAABB<IntType>) -> impl Iterator<Item=&PhysicalAABB<IntType>>;
}