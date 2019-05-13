use std::mem::size_of;

use num_traits::{FromPrimitive, PrimInt};

/// Trait to provide an integer square root of a number.
pub trait IntegerSquareRoot {
    /// Compute the integer square root.
    ///
    /// The integer square root is defined such that
    /// n.isqrt()^2 <= n, and (n.isqrt() + 1)^2 > n.
    ///
    /// # Example
    ///
    /// ```
    /// use genwunsym::utils::IntegerSquareRoot;
    /// let n = 15i32.isqrt();
    /// assert_eq!(3, n);
    /// ```
    fn isqrt(self) -> Self;
}

/// Blanket integer square root implementation for primitive types.
///
/// This implementation uses the nearest larger floating point type, uses that to compute
/// the square root, and then converts it back to an integer.
impl<T> IntegerSquareRoot for T
where
    T: PrimInt + FromPrimitive,
{
    #[inline]
    fn isqrt(self) -> Self {
        if size_of::<T>() <= size_of::<u16>() {
            let f = self.to_f32().unwrap();
            FromPrimitive::from_f32(f.sqrt()).unwrap()
        } else {
            let f = self.to_f64().unwrap();
            FromPrimitive::from_f64(f.sqrt()).unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_isqrt() {
        // Ensure that it works across the entire number u16 range.
        for n in 0u32..(1 << 16) {
            let root = n.isqrt();
            assert!(root * root <= n);
            assert!((root + 1) * (root + 1) > n);
        }

        // Ensure that the optimized code path also works
        assert_eq!(15, 255u8.isqrt())
    }
}
