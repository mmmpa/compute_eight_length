#![no_std]

pub fn compute_eight_length(src: usize) -> usize {
    match src >> 3 {
        m if m == 0 => 1,
        m if src % 8 == 0 => m,
        m => m + 1,
    }
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct ComputedEightLength(usize);

impl ComputedEightLength {
    pub fn from_computed(n: usize) -> Self {
        Self(n)
    }
}

impl From<usize> for ComputedEightLength {
    fn from(n: usize) -> Self {
        Self(compute_eight_length(n))
    }
}

impl Into<usize> for ComputedEightLength {
    fn into(self) -> usize {
        self.0
    }
}

impl AsRef<usize> for ComputedEightLength {
    fn as_ref(&self) -> &usize {
        &self.0
    }
}

mod test {
    #[allow(unused_imports)]
    use crate::compute_eight_length;

    #[test]
    fn test_eight_width() {
        assert_eq!(1, compute_eight_length(7));
        assert_eq!(1, compute_eight_length(8));
        assert_eq!(2, compute_eight_length(9));
        assert_eq!(2, compute_eight_length(16));
        assert_eq!(3, compute_eight_length(17));
    }
}
