use std::ops;

pub trait VectorMath {
    type ElementType;
    fn into_zip_with(
        self,
        other: Self,
        f: impl FnMut(Self::ElementType, Self::ElementType) -> Self::ElementType,
    ) -> Self;
    fn assign_zip_with(
        &mut self,
        other: Self,
        f: impl FnMut(&mut Self::ElementType, Self::ElementType),
    );
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Vector<A>(pub A);

impl<A> From<A> for Vector<A> {
    fn from(value: A) -> Self {
        Self(value)
    }
}

impl<A, E, I, O> ops::Index<I> for Vector<A>
where
    A: VectorMath<ElementType = E> + ops::Index<I, Output = O>,
{
    type Output = O;

    fn index(&self, index: I) -> &Self::Output {
        &self.0[index]
    }
}

impl<A, E, I, O> ops::IndexMut<I> for Vector<A>
where
    A: VectorMath<ElementType = E> + ops::IndexMut<I, Output = O>,
{
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<A, E> ops::Add for Vector<A>
where
    E: ops::Add<E, Output = E>,
    A: VectorMath<ElementType = E>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0.into_zip_with(rhs.0, |a, b| a + b))
    }
}

impl<A, E> ops::AddAssign for Vector<A>
where
    E: ops::AddAssign<E>,
    A: VectorMath<ElementType = E>,
{
    fn add_assign(&mut self, rhs: Self) {
        self.0.assign_zip_with(rhs.0, |a, b| *a += b);
    }
}

impl<A, E> ops::Sub for Vector<A>
where
    E: ops::Sub<E, Output = E>,
    A: VectorMath<ElementType = E>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0.into_zip_with(rhs.0, |a, b| a - b))
    }
}

impl<A, E> ops::SubAssign for Vector<A>
where
    E: ops::SubAssign<E>,
    A: VectorMath<ElementType = E>,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.0.assign_zip_with(rhs.0, |a, b| *a -= b);
    }
}

impl<A, E> ops::Mul for Vector<A>
where
    E: ops::Mul<E, Output = E>,
    A: VectorMath<ElementType = E>,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0.into_zip_with(rhs.0, |a, b| a * b))
    }
}

impl<A, E> ops::MulAssign for Vector<A>
where
    E: ops::MulAssign<E>,
    A: VectorMath<ElementType = E>,
{
    fn mul_assign(&mut self, rhs: Self) {
        self.0.assign_zip_with(rhs.0, |a, b| *a *= b);
    }
}

impl<A, E> ops::Div for Vector<A>
where
    E: ops::Div<E, Output = E>,
    A: VectorMath<ElementType = E>,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0.into_zip_with(rhs.0, |a, b| a / b))
    }
}

impl<A, E> ops::DivAssign for Vector<A>
where
    E: ops::DivAssign<E>,
    A: VectorMath<ElementType = E>,
{
    fn div_assign(&mut self, rhs: Self) {
        self.0.assign_zip_with(rhs.0, |a, b| *a /= b);
    }
}


impl<A, E, I> FromIterator<I> for Vector<A>
where
    A: VectorMath<ElementType = E> + FromIterator<I>,
{
    fn from_iter<T: IntoIterator<Item = I>>(iter: T) -> Self {
        let a : A = iter.into_iter().collect();
        a.into()
    }
}
