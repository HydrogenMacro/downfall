use crate::Int;

pub struct Body {
	pub x: Int,
	pub y: Int,
	pub width: Int,
	pub height: Int,
	persistent_accelerations: Vec<(Int, Int)>
}