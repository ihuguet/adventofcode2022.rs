use std::{ops, fmt};

/// coordenates of a 2D grid
/// note that if thinking in rows and columns, Y are the rows and X are the
/// columns. So, Point::from((y, x)) means Point::from((row, col))
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point<T: Copy> {
	pub y: T,
	pub x: T,
}

impl<T: Copy> Point<T> {
	pub fn row(&self) -> T {
		self.y
	}

	pub fn col(&self) -> T {
		self.x
	}
}

/// create new Point with Point::from((0, 0))
/// also, allow to convert tuple to Point with (0, 0).into()
impl<T: Copy> From<(T, T)> for Point<T> {
	fn from(t: (T, T)) -> Self {
		Point { y: t.0, x: t.1 }
	}
}

impl<T> ops::Add<Point<T>> for Point<T>
where
	T: Copy + ops::Add<T, Output = T>
{
	type Output = Point<T>;
	fn add(self, rhs: Point<T>) -> Self::Output {
		Self {
			x: self.x + rhs.x,
			y: self.y + rhs.y
		}
	}
}

impl<T> ops::AddAssign<Point<T>> for Point<T>
where
	T: Copy + ops::AddAssign<T>
{
	fn add_assign(&mut self, rhs: Point<T>) {
		self.x += rhs.x;
		self.y += rhs.y;
	}
}

impl<T> ops::Sub<Point<T>> for Point<T>
where
	T: Copy + ops::Sub<T, Output = T>
{
	type Output = Point<T>;
	fn sub(self, rhs: Point<T>) -> Self::Output {
		Self {
			x: self.x - rhs.x,
			y: self.y - rhs.y
		}
	}
}

impl<T> ops::SubAssign<Point<T>> for Point<T>
where
	T: Copy + ops::SubAssign<T>
{
	fn sub_assign(&mut self, rhs: Point<T>) {
		self.x -= rhs.x;
		self.y -= rhs.y;
	}
}

impl<T> ops::Mul<T> for Point<T>
where
	T: Copy + ops::Mul<T, Output = T>
{
	type Output = Point<T>;
	fn mul(self, rhs: T) -> Self::Output {
		Self {
			x: self.x * rhs,
			y: self.y * rhs
		}
	}
}

impl<T> ops::MulAssign<T> for Point<T>
where
	T: Copy + ops::MulAssign<T>
{
	fn mul_assign(&mut self, rhs: T) {
		self.x *= rhs;
		self.y *= rhs;
	}
}

impl<T> ops::Div<T> for Point<T>
where
	T: Copy + ops::Div<T, Output = T>
{
	type Output = Point<T>;
	fn div(self, rhs: T) -> Self::Output {
		Self {
			x: self.x / rhs,
			y: self.y / rhs
		}
	}
}

impl<T> ops::DivAssign<T> for Point<T>
where
	T: Copy + ops::DivAssign<T>
{
	fn div_assign(&mut self, rhs: T) {
		self.x /= rhs;
		self.y /= rhs;
	}
}

impl<T> fmt::Display for Point<T>
where
	T: Copy + fmt::Display
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{{x: {}, y: {}}}", self.x, self.y)
	}
}
