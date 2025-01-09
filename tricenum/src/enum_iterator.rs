use std::marker::PhantomData;

use crate::trivial_enum::TrivialEnum;

pub struct EnumIterator<E: TrivialEnum> {
    begin: usize,
    end: usize,
    key: PhantomData<E>,
}

impl<E: TrivialEnum> EnumIterator<E> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<E: TrivialEnum> Default for EnumIterator<E> {
    fn default() -> Self {
        Self {
            begin: 0,
            end: E::ENUM_SIZE,
            key: Default::default(),
        }
    }
}

impl<E: TrivialEnum> Iterator for EnumIterator<E> {
    type Item = E;

    fn next(&mut self) -> Option<Self::Item> {
        if self.begin >= self.end {
            return None;
        }

        // SAFETY: we initialize begin and end so that begin is always < ENUM_SIZE
        let res = unsafe { E::from_index_unchecked(self.begin) };
        self.begin += 1;
        Some(res)
    }
}

impl<E: TrivialEnum> ExactSizeIterator for EnumIterator<E> {
    fn len(&self) -> usize {
        self.end - self.begin
    }
}

impl<E: TrivialEnum> DoubleEndedIterator for EnumIterator<E> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.begin >= self.end {
            return None;
        }

        self.end -= 1;

        // SAFETY: we initialize begin and end so that (end - 1) is always < ENUM_SIZE
        Some(unsafe { E::from_index_unchecked(self.end) })
    }
}
