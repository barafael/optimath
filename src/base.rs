use crate::types::Vector;
use core::ops::*;

// reference operations
// need to have the Output = T on the Add for &T, otherwise you get infinite recursion

impl<'a, const N: usize> Add for &'a Vector<f32, N> {
	fn add(self, other: Self) -> Vector<f32, N> {
		//todo: do some simd-stuff for specific types
		self.inner
			.iter()
			.zip(other.inner.iter())
			.map(|(s, o)| Add::add(s, o))
			.collect()
	}
}

impl<'a, T, const N: usize> Add for &'a Vector<T, N>
where
	&'a T: Add<Output = T>,
{
	type Output = Vector<T, N>;
	default fn add(self, other: Self) -> Vector<T, N> {
		self.inner
			.iter()
			.zip(other.inner.iter())
			.map(|(s, o)| Add::add(s, o))
			.collect()
	}
}

impl<'a, T, const N: usize> Sub for &'a Vector<T, N>
where
	&'a T: Sub<Output = T>,
{
	type Output = Vector<T, N>;
	fn sub(self, other: Self) -> Vector<T, N> {
		self.inner
			.iter()
			.zip(other.inner.iter())
			.map(|(s, o)| Sub::sub(s, o))
			.collect()
	}
}

impl<'a, T, const N: usize> Mul for &'a Vector<T, N>
where
	&'a T: Mul<Output = T>,
{
	type Output = Vector<T, N>;
	fn mul(self, other: Self) -> Vector<T, N> {
		self.inner
			.iter()
			.zip(other.inner.iter())
			.map(|(s, o)| Mul::mul(s, o))
			.collect()
	}
}

impl<'a, T, const N: usize> Div for &'a Vector<T, N>
where
	&'a T: Div<Output = T>,
{
	type Output = Vector<T, N>;
	fn div(self, other: Self) -> Vector<T, N> {
		self.inner
			.iter()
			.zip(other.inner.iter())
			.map(|(s, o)| Div::div(s, o))
			.collect()
	}
}

// assigning operations

impl<'a, T, const N: usize> AddAssign<&'a Vector<T, N>> for Vector<T, N>
where
	T: AddAssign<&'a T>,
{
	fn add_assign(&mut self, other: &'a Vector<T, N>) {
		let iter = self.inner.iter_mut().zip(other.inner.iter());
		for (s, o) in iter {
			*s += o
		}
	}
}

impl<'a, T, const N: usize> SubAssign<&'a Vector<T, N>> for Vector<T, N>
where
	T: SubAssign<&'a T>,
{
	fn sub_assign(&mut self, other: &'a Vector<T, N>) {
		let iter = self.inner.iter_mut().zip(other.inner.iter());
		for (s, o) in iter {
			*s -= o
		}
	}
}

impl<'a, T, const N: usize> MulAssign<&'a Vector<T, N>> for Vector<T, N>
where
	T: MulAssign<&'a T>,
{
	fn mul_assign(&mut self, other: &'a Vector<T, N>) {
		let iter = self.inner.iter_mut().zip(other.inner.iter());
		for (s, o) in iter {
			*s *= o
		}
	}
}

impl<'a, T, const N: usize> DivAssign<&'a Vector<T, N>> for Vector<T, N>
where
	T: DivAssign<&'a T>,
{
	fn div_assign(&mut self, other: &'a Vector<T, N>) {
		let iter = self.inner.iter_mut().zip(other.inner.iter());
		for (s, o) in iter {
			*s /= o
		}
	}
}

#[cfg(test)]
pub(crate) const TESTLEN: usize = 777usize;

#[test]
fn default_is_default() {
	let m = Vector::<f32, TESTLEN>::default();
	for i in 0..TESTLEN {
		assert_eq!(m.inner[i], f32::default());
	}
}

#[test]
fn operations() {
	let a: Vector<f32, TESTLEN> = (0..TESTLEN).map(|x| x as f32).collect();
	let b: Vector<f32, TESTLEN> = (1..{ TESTLEN + 1 }).map(|x| x as f32).collect();

	let add = &a + &b;
	let sub = &a - &b;
	let mul = &a * &b;
	let div = &a / &b;

	for i in 0..TESTLEN {
		assert_eq!(a.inner[i] + b.inner[i], add.inner[i]);
		assert_eq!(a.inner[i] - b.inner[i], sub.inner[i]);
		assert_eq!(a.inner[i] * b.inner[i], mul.inner[i]);
		assert_eq!(a.inner[i] / b.inner[i], div.inner[i]);
	}
}

#[test]
fn assignment_operations() {
	let a: Vector<f32, TESTLEN> = (0..TESTLEN).map(|x| x as f32).collect();
	let b: Vector<f32, TESTLEN> = (1..{ TESTLEN + 1 }).map(|x| x as f32).collect();

	let mut add = a.clone();
	add += &b;

	let mut sub = a.clone();
	sub -= &b;

	let mut mul = a.clone();
	mul *= &b;

	let mut div = a.clone();
	div /= &b;

	for i in 0..TESTLEN {
		assert_eq!(a.inner[i] + b.inner[i], add.inner[i]);
		assert_eq!(a.inner[i] - b.inner[i], sub.inner[i]);
		assert_eq!(a.inner[i] * b.inner[i], mul.inner[i]);
		assert_eq!(a.inner[i] / b.inner[i], div.inner[i]);
	}
}
