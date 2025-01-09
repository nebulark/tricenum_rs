use std::{fmt::Debug, marker::PhantomData, ops::{self, Index}};

use crate::TrivialEnum;

type BitStorage = u32;

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct EnumBitset<E: TrivialEnum> {
    flags: BitStorage,
    phantom: PhantomData<E>,
}

impl<E: TrivialEnum> EnumBitset<E> {
    pub const fn const_validate_size() {
        assert!(
            E::ENUM_SIZE <= BitStorage::BITS as usize,
            "N must <= u32::BITS"
        );
    }
}

impl<E: TrivialEnum> EnumBitset<E> {
    pub fn new() -> Self {
        Self::default()
    }

    fn new_raw_internal(flags: BitStorage) -> Self {
        const {
            Self::const_validate_size();
        }
        Self {
            flags,
            phantom: Default::default(),
        }
    }

    pub fn is_set(&self, value: E) -> bool {
        self.is_any_set(value.into())
    }

    pub fn set(&mut self, value: E) {
        *self |= Self::from(value);
    }

    pub fn unset(&mut self, value: E) {
        *self &= !Self::from(value);
    }

    pub fn set_to(&mut self, value: E, should_be_set : bool) {
        if should_be_set {
            self.set(value);
        } else {
            self.unset(value);
        }
    }

    pub fn is_any_set(&self, other: Self) -> bool {
        (self.flags & other.flags) != 0
    }

    pub fn is_none(&self) -> bool {
        self.flags == 0
    }

    pub fn iter(&self) -> EnumBitsetIter<E> {
        EnumBitsetIter::new(self)
    }

    fn enum_as_flag(value : E) -> BitStorage {
        const { Self::const_validate_size();}
        1 << value.index()
    }
}

impl<E: TrivialEnum> Default for EnumBitset<E> {
    fn default() -> Self {
        Self::new_raw_internal(0)
    }
}

impl<E: TrivialEnum> Index<E> for EnumBitset<E> {
    type Output = bool;

    fn index(&self, index: E) -> &Self::Output {
        if self.is_set(index) {
            &true
        } else {
            &false
        }
    }
}

impl<E: TrivialEnum + Debug> std::fmt::Debug for EnumBitset<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_set().entries(self.iter()).finish()
    }
}

impl<E: TrivialEnum> ops::BitOr for EnumBitset<E> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self::new_raw_internal(self.flags | rhs.flags)
    }
}

impl<E: TrivialEnum> ops::BitOrAssign for EnumBitset<E> {
    fn bitor_assign(&mut self, rhs: Self) {
        self.flags |= rhs.flags
    }
}

impl<E: TrivialEnum> ops::BitAnd for EnumBitset<E> {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self::new_raw_internal(self.flags & rhs.flags)
    }
}

impl<E: TrivialEnum> ops::BitAndAssign for EnumBitset<E> {
    fn bitand_assign(&mut self, rhs: Self) {
        self.flags &= rhs.flags;
    }
}

impl<E: TrivialEnum> ops::BitXor for EnumBitset<E> {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self::new_raw_internal(self.flags ^ rhs.flags)
    }
}

impl<E: TrivialEnum> ops::BitXorAssign for EnumBitset<E> {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.flags ^= rhs.flags;
    }
}

impl<E: TrivialEnum> ops::Not for EnumBitset<E> {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self::new_raw_internal(!self.flags)
    }
}

impl<E: TrivialEnum> From<E> for EnumBitset<E> {
    fn from(value: E) -> Self {
        Self::new_raw_internal( Self::enum_as_flag(value))
    }
}

impl<E: TrivialEnum, const N: usize> From<[E; N]> for EnumBitset<E> {
    fn from(arr: [E; N]) -> Self {
        arr.into_iter().collect()
    }
}

impl<E: TrivialEnum> FromIterator<E> for EnumBitset<E> {
    fn from_iter<T: IntoIterator<Item = E>>(iter: T) -> Self {
        assert!(E::ENUM_SIZE <= BitStorage::BITS as usize);

        let mut flags = 0;

        for v in iter {
            flags |= Self::enum_as_flag(v)
        }

        Self::new_raw_internal(flags)
    }
}

impl<E: TrivialEnum> std::iter::IntoIterator for EnumBitset<E> {
    type Item = E;

    type IntoIter = EnumBitsetIter<E>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

// ------ Iterator

pub struct EnumBitsetIter<E> {
    flags: BitStorage,
    phantom: PhantomData<E>,
}

impl<E: TrivialEnum> EnumBitsetIter<E> {
    pub fn new(bitset: &EnumBitset<E>) -> Self {
        Self {
            flags: bitset.flags,
            phantom: bitset.phantom,
        }
    }
}

impl<E: TrivialEnum> Iterator for EnumBitsetIter<E> {
    type Item = E;

    fn next(&mut self) -> Option<Self::Item> {
        if self.flags == 0 {
            return None;
        }

        let idx = self.flags.trailing_zeros() as usize;
        let mask = (!0) << (idx + 1);

        self.flags = self.flags & mask;

        // SAFETY: We initialize iterator so that no bit is set that will yield a index < E::ENUM_SIZE
        debug_assert!(idx < E::ENUM_SIZE);
        let res = unsafe { E::from_index_unchecked(idx) };

        Some(res)
    }
}

impl<E: TrivialEnum> DoubleEndedIterator for EnumBitsetIter<E> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.flags == 0 {
            return None;
        }

        let leading_zeroes = self.flags.leading_zeros() as usize;
        let idx = (BitStorage::BITS as usize - 1 - leading_zeroes) as usize;
        let mask = (!0) >> (leading_zeroes + 1);
        self.flags = self.flags & mask;

        // SAFETY: We initialize iterator so that no bit is set that will yield a index < E::ENUM_SIZE
        debug_assert!((idx as usize) < E::ENUM_SIZE);
        let res = unsafe { E::from_index_unchecked(idx as usize) };

        Some(res)
    }
}

impl<E: TrivialEnum> ExactSizeIterator for EnumBitsetIter<E> {
    fn len(&self) -> usize {
        self.flags.count_ones() as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(dead_code)]
    #[repr(u8)]
    #[derive(TrivialEnum, PartialEq, Eq, Debug)]
    enum TestEnum {
        First,
        Second,
        Third,
        Fourth,
        Fith,
        Last,
    }

    #[test]
    fn test_from_iter() {
        let bitset: EnumBitset<TestEnum> = [TestEnum::First, TestEnum::Last].into_iter().collect();
        assert!(bitset.is_set(TestEnum::First));
        assert!(bitset.is_set(TestEnum::Last));
        assert!(!bitset.is_set(TestEnum::Second));
    }

    #[test]
    fn test_from() {
        let bitset: EnumBitset<TestEnum> =
            [TestEnum::First, TestEnum::Last, TestEnum::Third].into();
        assert!(bitset.is_set(TestEnum::First));
        assert!(bitset.is_set(TestEnum::Last));
        assert!(!bitset.is_set(TestEnum::Second));
    }

    #[test]
    fn test_iterator() {
        let bitset: EnumBitset<TestEnum> = [
            TestEnum::Fourth,
            TestEnum::First,
            TestEnum::Last,
            TestEnum::Third,
        ]
        .into();
        let mut it = bitset.iter();

        assert_eq!(Some(TestEnum::First), it.next());
        assert_eq!(Some(TestEnum::Third), it.next());
        assert_eq!(Some(TestEnum::Fourth), it.next());
        assert_eq!(Some(TestEnum::Last), it.next());
        assert_eq!(None, it.next());
        assert_eq!(None, it.next());
    }

    #[test]
    fn test_double_ended_iterator() {
        let bitset: EnumBitset<TestEnum> = [
            TestEnum::First,
            TestEnum::Last,
            TestEnum::Fourth,
            TestEnum::Third,
        ]
        .into();
        let mut it = bitset.iter();

        assert_eq!(Some(TestEnum::Last), it.next_back());
        assert_eq!(Some(TestEnum::Fourth), it.next_back());

        assert_eq!(Some(TestEnum::First), it.next());
        assert_eq!(Some(TestEnum::Third), it.next());
        assert_eq!(None, it.next());
        assert_eq!(None, it.next());
    }
}
