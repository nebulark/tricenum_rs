use std::{marker::PhantomData, ops};

use crate::{vector_math::VectorMath, TrivialEnum};

#[repr(transparent)]
pub struct EnumArray<E: TrivialEnum, T, const N: usize> {
    values: [T; N],
    key: PhantomData<E>,
}

impl<E: TrivialEnum, T, const N: usize> EnumArray<E, T, N> {
    pub const fn const_validate_size() {
        assert!(E::ENUM_SIZE == N, "N must be T::ENUM_SIZE");
    }
}

impl<E: TrivialEnum, T, const N: usize> EnumArray<E, T, N>
where
    T: Clone,
{
    pub fn fill(value: &T) -> Self {
        Self {
            values: [(); N].map(|_| value.clone()),
            key: Default::default(),
        }
    }
}

impl<E: TrivialEnum, T, const N: usize> EnumArray<E, T, N>
where
    Self: Default,
{
    pub fn from_single(ty: E, val: T) -> Self {
        const {
            Self::const_validate_size();
        }

        let mut res = Self::default();
        res[ty] = val;
        res
    }

    pub fn new() -> Self {
        const {
            assert!(E::ENUM_SIZE == N);
        }
        Self::default()
    }
}

impl<E: TrivialEnum, T, const N: usize> VectorMath for EnumArray<E, T, N> {
    type ElementType = T;

    fn into_zip_with(
        self,
        other: Self,
        mut f: impl FnMut(Self::ElementType, Self::ElementType) -> Self::ElementType,
    ) -> Self {
        let mut self_iter = self.values.into_iter();
        let mut other_iter = other.values.into_iter();

        let values = [(); N].map(|_| f(self_iter.next().unwrap(), other_iter.next().unwrap()));

        Self {
            values,
            key: Default::default(),
        }
    }

    fn assign_zip_with(
        &mut self,
        other: Self,
        mut f: impl FnMut(&mut Self::ElementType, Self::ElementType),
    ) {
        for (s, v) in self.values.iter_mut().zip(other.values.into_iter()) {
            f(s, v);
        }
    }
}

// ---- Traits

impl<E: TrivialEnum, T, const N: usize> ops::Index<E> for EnumArray<E, T, N> {
    type Output = T;

    fn index(&self, index: E) -> &Self::Output {
        const {
            Self::const_validate_size();
        }
        &self.values[index.index()]
    }
}

impl<E: TrivialEnum, T, const N: usize> ops::IndexMut<E> for EnumArray<E, T, N> {
    fn index_mut(&mut self, index: E) -> &mut Self::Output {
        const {
            Self::const_validate_size();
        }
        &mut self.values[index.index()]
    }
}

impl<E: TrivialEnum, T, const N: usize> FromIterator<(E, T)> for EnumArray<E, T, N>
where
    Self: Default,
{
    fn from_iter<I: IntoIterator<Item = (E, T)>>(iter: I) -> Self {
        let mut res: Self = Default::default();

        for i in iter {
            res[i.0] = i.1;
        }

        res
    }
}

impl<E: TrivialEnum, T, const N: usize> From<(E, T)> for EnumArray<E, T, N>
where
    T: Default,
{
    fn from(val: (E, T)) -> Self {
        Self::from_single(val.0, val.1)
    }
}

impl<E: TrivialEnum, T, const N: usize> Default for EnumArray<E, T, N>
where
    T: Default,
{
    fn default() -> Self {
        const {
            Self::const_validate_size();
        }

        Self {
            values: [(); N].map(|_| T::default()),
            key: Default::default(),
        }
    }
}
