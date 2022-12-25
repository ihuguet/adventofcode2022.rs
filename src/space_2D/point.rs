use std::ops::*;
use std::fmt;

/// trait used to limit T in Point<T> to isize and usize only
pub trait Coord: Copy + Default {}
impl Coord for isize {}
impl Coord for usize {}

/// coordenates of a 2D grid
/// note that if thinking in rows and columns, Y are the rows and X are the
/// columns. So, Point::from((y, x)) means Point::from((row, col))
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Point<T: Coord = usize> {
	pub y: T,
	pub x: T,
}

impl<T: Coord> Point<T> {
	pub fn row(&self) -> T {
		self.y
	}

	pub fn col(&self) -> T {
		self.x
	}

	pub fn to_tuple(&self) -> (T, T) {
		(self.y, self.x)
	}
}

impl Point<isize> {
	pub fn signum(&self) -> (isize, isize) {
		(if self.y == 0 {0} else {self.y / self.y.abs()},
		 if self.x == 0 {0} else {self.x / self.x.abs()})
	}
}

impl Point<usize> {
	/// panics on underflow
	pub fn add_signed(&self, other: Point<isize>) -> Point<usize> {
		Self::from(other + self.clone().into())
	}
}

/// create new Point with Point::from((0, 0))
/// also, allow to convert tuple to Point with (0, 0).into()
impl<T: Coord> From<(T, T)> for Point<T> {
	fn from(t: (T, T)) -> Self {
		Point { y: t.0, x: t.1 }
	}
}

impl<T: Coord> From<Point<T>> for (T, T) {
	fn from(p: Point<T>) -> Self {
		p.to_tuple()
	}
}

impl From<Point<usize>> for Point<isize> {
	fn from(p: Point<usize>) -> Self {
		Point::from((p.y as isize, p.x as isize))
	}
}

impl From<Point<isize>> for Point<usize> {
	fn from(p: Point<isize>) -> Self {
		Point::from((p.y as usize, p.x as usize))
	}
}

impl<T> Add for Point<T>
where
	T: Coord + Add<T, Output = T>
{
	type Output = Point<T>;
	fn add(self, rhs: Point<T>) -> Self::Output {
		Self {
			x: self.x + rhs.x,
			y: self.y + rhs.y
		}
	}
}

impl<T> AddAssign for Point<T>
where
	T: Coord + AddAssign<T>
{
	fn add_assign(&mut self, rhs: Point<T>) {
		self.x += rhs.x;
		self.y += rhs.y;
	}
}

impl Sub for Point {
	type Output = Point<isize>;
	/// return Point<isize> to avoid panics or underflows
	fn sub(self, rhs: Self) -> Self::Output {
		Point::from((self.y as isize - rhs.y as isize, self.x as isize - rhs.x as isize))
	}
}

impl Sub for Point<isize> {
	type Output = Point<isize>;
	fn sub(self, rhs: Self) -> Self::Output {
		Point::from((self.y - rhs.y, self.x - rhs.x))
	}
}

impl<T> SubAssign for Point<T>
where
	T: Coord + SubAssign<T>
{
	/// might panic with Point<usize>
	fn sub_assign(&mut self, rhs: Point<T>) {
		self.x -= rhs.x;
		self.y -= rhs.y;
	}
}

impl<T> Mul<T> for Point<T>
where
	T: Coord + Mul<T, Output = T>
{
	type Output = Point<T>;
	fn mul(self, rhs: T) -> Self::Output {
		Self {
			x: self.x * rhs,
			y: self.y * rhs
		}
	}
}

impl<T> MulAssign<T> for Point<T>
where
	T: Coord + MulAssign<T>
{
	fn mul_assign(&mut self, rhs: T) {
		self.x *= rhs;
		self.y *= rhs;
	}
}

impl<T> Div<T> for Point<T>
where
	T: Coord + Div<T, Output = T>
{
	type Output = Point<T>;
	fn div(self, rhs: T) -> Self::Output {
		Self {
			x: self.x / rhs,
			y: self.y / rhs
		}
	}
}

impl<T> DivAssign<T> for Point<T>
where
	T: Coord + DivAssign<T>
{
	fn div_assign(&mut self, rhs: T) {
		self.x /= rhs;
		self.y /= rhs;
	}
}

impl<T> fmt::Display for Point<T>
where
	T: Coord + fmt::Display
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{{x: {}, y: {}}}", self.x, self.y)
	}
}
