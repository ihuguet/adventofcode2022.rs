use super::{Point, Grid};
use std::ops;

/// 2D Grid implemented with Vecs. Can be indexed with Point<usize>
pub type VecGrid<T> = Vec<Vec<T>>;

impl<T> Grid<usize> for VecGrid<T> {
	type Item = T;

	fn contains_point(&self, point: Point<usize>) -> bool {
		(0..self.len()).contains(&point.y) && (0..self[0].len()).contains(&point.x)
	}
	
	fn get_point(&self, point: Point<usize>) -> Option<&Self::Item> {
		match self.contains_point(point) {
			true => Some(&self[point.y][point.x]),
			false => None
		}
	}

	fn get_point_mut(&mut self, point: Point<usize>) -> Option<&mut Self::Item> {
		match self.contains_point(point) {
			true => Some(&mut self[point.y][point.x]),
			false => None
		}
	}

	fn adjacents_4(&self, point: Point<usize>) -> Vec<Point<usize>> {
		let (y, x) = (point.y as isize, point.x as isize);
		let (y_max, x_max) = (self.len() as isize - 1, self[0].len() as isize - 1);

		[(-1, 0), (0, -1), (0, 1), (1, 0)].iter().map(|p| {
			Point::from((
				(y + p.0).clamp(0, y_max) as usize,
				(x + p.1).clamp(0, x_max) as usize
			))
		}).filter(|p| *p != point).collect()
	}

	fn iter_grid(&self) -> Box<dyn Iterator<Item = (Point<usize>, &Self::Item)> + '_> {
		Box::new(self.iter().enumerate().flat_map(|(y, row)| {
			row.iter().enumerate().map(move |(x, v)| (Point::from((y, x)), v))
		}))
	}

	fn iter_grid_mut(&mut self) -> Box<dyn Iterator<Item = (Point<usize>, &mut Self::Item)> + '_> {
		Box::new(self.iter_mut().enumerate().flat_map(|(y, row)| {
			row.iter_mut().enumerate().map(move |(x, v)| (Point::from((y, x)), v))
		}))
	}
}

impl<T> ops::Index<Point<usize>> for VecGrid<T> {
	type Output = T;
	fn index(&self, index: Point<usize>) -> &Self::Output {
		&self[index.y][index.x]
	}
}

impl<T> ops::IndexMut<Point<usize>> for VecGrid<T> {
	fn index_mut(&mut self, index: Point<usize>) -> &mut Self::Output {
		&mut self[index.y][index.x]
	}
}

/// trait implementation to use Point<isize> as index of a VecGrid
/// Warn: it will panic if indexed with negative values of isize
/// It can still be useful to allow doing additions of Point<isize> before
/// indexing, but when indexing the caller must be sure that coordinates are
/// not negative, or use get_point that will return None
impl<T> Grid<isize> for VecGrid<T> {
	type Item = T;

	fn contains_point(&self, point: Point<isize>) -> bool {
		(0..self.len() as isize).contains(&point.y) && (0..self[0].len() as isize).contains(&point.x)
	}

	fn get_point(&self, point: Point<isize>) -> Option<&Self::Item> {
		match self.contains_point(point) {
			true => Some(&self[point.y as usize][point.x as usize]),
			false => None
		}
	}

	fn get_point_mut(&mut self, point: Point<isize>) -> Option<&mut Self::Item> {
		match self.contains_point(point) {
			true => Some(&mut self[point.y as usize][point.x as usize]),
			false => None
		}
	}

	fn adjacents_4(&self, point: Point<isize>) -> Vec<Point<isize>> {
		let (y_max, x_max) = (self.len() as isize - 1, self[0].len() as isize - 1);

		[(-1, 0), (0, -1), (0, 1), (1, 0)].iter().map(|p| {
			Point::from((
				(point.y + p.0).clamp(0, y_max),
				(point.x + p.1).clamp(0, x_max)
			))
		}).filter(|p| *p != point).collect()
	}

	fn iter_grid(&self) -> Box<dyn Iterator<Item = (Point<isize>, &Self::Item)> + '_> {
		Box::new(self.iter().enumerate().flat_map(|(y, row)| {
			row.iter().enumerate().map(move |(x, v)| (Point::from((y as isize, x as isize)), v))
		}))
	}

	fn iter_grid_mut(&mut self) -> Box<dyn Iterator<Item = (Point<isize>, &mut Self::Item)> + '_> {
		Box::new(self.iter_mut().enumerate().flat_map(|(y, row)| {
			row.iter_mut().enumerate().map(move |(x, v)| (Point::from((y as isize, x as isize)), v))
		}))
	}
}

impl<T> ops::Index<Point<isize>> for VecGrid<T> {
	type Output = T;
	fn index(&self, index: Point<isize>) -> &Self::Output {
		&self[index.y as usize][index.x as usize]
	}
}

impl<T> ops::IndexMut<Point<isize>> for VecGrid<T> {
	fn index_mut(&mut self, index: Point<isize>) -> &mut Self::Output {
		&mut self[index.y as usize][index.x as usize]
	}
}
