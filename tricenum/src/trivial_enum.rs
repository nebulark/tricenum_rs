pub unsafe trait TrivialEnum: Sized {
    // SAFETY: ENUM_SIZE must be the number of enum variants
    // SAFETY: The enum must be a trivial enum, which no specifically set values
    const ENUM_SIZE: usize;

    // SAFETY: most return a different value for each variant an be between 0..ENUM_SIZE
    // typically just an as cast
    fn index(self) -> usize;

    // PRECONDITION: val < ENUM_SIZE
    unsafe fn from_index_unchecked(val: usize) -> Self;

    fn from_index(val: usize) -> Option<Self> {
        if val < Self::ENUM_SIZE {
            unsafe { Some(Self::from_index_unchecked(val)) }
        } else {
            None
        }
    }
}
