use super::Point;

pub trait Grid<T: Copy> {
	type Item;

	/// check if point is valid index and the Grid has an entry for it
	fn contains_point(&self, point: Point<T>) -> bool;

	/// get a immutable ref to the element in the point
	fn get_point(&self, point: Point<T>) -> Option<&Self::Item>;

	/// get a mutable ref to the element in the point
	fn get_point_mut(&mut self, point: Point<T>) -> Option<&mut Self::Item>;

	/// get the 4 adjacent points (left, right, top, bottom) if they exist
	fn adjacents_4(&self, point: Point<T>) -> Vec<Point<T>>;

	/// iterate over the grid in an enumerate fashion
	fn iter_grid(&self) -> Box<dyn Iterator<Item = (Point<T>, &Self::Item)> + '_>;

	/// iterate over the grid in an enumerate fashion with mutable refs
	fn iter_grid_mut(&mut self) -> Box<dyn Iterator<Item = (Point<T>, &mut Self::Item)> + '_>;
}
