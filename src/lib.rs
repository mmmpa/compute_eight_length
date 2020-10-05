#![no_std]

pub fn compute_eight_length(src: usize) -> usize {
    match src >> 3 {
        m if m == 0 => 1,
        m if src % 8 == 0 => m,
        m => m + 1,
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
