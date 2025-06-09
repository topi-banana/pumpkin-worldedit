use std::{
    cmp::Ordering,
    ops::{Bound, RangeBounds, RangeInclusive},
};

use num_traits::Euclid;

/// An iterator that splits a given integer range into 16-value "chunks".
///
/// Each item yielded by the iterator is a tuple `(chunk_index, range_within_chunk)`,
/// where `chunk_index` is the base chunk number (`i32`) and `range_within_chunk` is a
/// `RangeInclusive<i32>` within the chunk. Each chunk corresponds to a 16-element block
/// of integers, aligned to multiples of 16.
///
/// # Example
///
/// ```
/// let range = -10..20;
/// let chunks: Vec<_> = ChunkedRange::new(range).collect();
///
/// for (chunk, inner_range) in chunks {
///     println!("Chunk {}: {:?}", chunk, inner_range.collect::<Vec<_>>());
/// }
/// ```
///
/// This will output something like:
///
/// ```text
/// Chunk -1: [6..=15]
/// Chunk 0: [0..=15]
/// Chunk 1: [0..=3]
/// ```
pub struct ChunkedRange {
    current: i32,
    end: i32,
}

impl ChunkedRange {
    /// Creates a new `ChunkedRange` from any `RangeBounds<i32>`.
    ///
    /// This converts the possibly unbounded and inclusive/exclusive bounds
    /// into an inclusive range with a definite start and end.
    ///
    /// # Arguments
    ///
    /// * `range` - A range of integers to iterate over in 16-value chunks.
    ///
    /// # Example
    ///
    /// ```
    /// let mut chunked = ChunkedRange::new(0..32);
    /// assert_eq!(chunked.next(), Some((0, 0..=15)));
    /// assert_eq!(chunked.next(), Some((1, 0..=15)));
    /// assert_eq!(chunked.next(), None);
    /// ```
    #[must_use]
    pub fn new<R: RangeBounds<i32>>(range: R) -> Self {
        let start = match range.start_bound() {
            Bound::Included(&p) => p,
            Bound::Excluded(&p) => p + 1,
            Bound::Unbounded => i32::MIN,
        };
        let end = match range.end_bound() {
            Bound::Included(&p) => p,
            Bound::Excluded(&p) => p - 1,
            Bound::Unbounded => i32::MAX,
        };
        Self {
            current: start,
            end,
        }
    }
}

impl Iterator for ChunkedRange {
    type Item = (i32, RangeInclusive<i32>);

    fn next(&mut self) -> Option<Self::Item> {
        let (div_current, rem_current) = self.current.div_rem_euclid(&16);
        let (div_end, rem_end) = self.end.div_rem_euclid(&16);

        let range_end = match div_current.cmp(&div_end) {
            Ordering::Less => 15,
            Ordering::Equal => {
                if rem_current < rem_end {
                    rem_end
                } else {
                    return None;
                }
            }
            Ordering::Greater => return None,
        };

        let res = Some((div_current, rem_current..=range_end));
        self.current = (div_current + 1) << 4;

        res
    }
}

#[cfg(test)]
mod tests {
    use super::ChunkedRange;

    #[test]
    fn test_chunked_range_restore() {
        let mut original = -12345..12345;

        for (chunk, range) in ChunkedRange::new(original.clone()) {
            for i in range {
                assert_eq!((chunk << 4) | i, original.next().unwrap());
            }
        }
    }

    #[test]
    fn test_chunked_range_example1() {
        let range = -10..20;
        let chunks: Vec<_> = ChunkedRange::new(range).collect();
        assert_eq!(chunks, vec![(-1, 6..=15), (0, 0..=15), (1, 0..=3),]);
    }

    #[test]
    fn test_chunked_range_example2() {
        let mut chunked = ChunkedRange::new(0..32);
        assert_eq!(chunked.next(), Some((0, 0..=15)));
        assert_eq!(chunked.next(), Some((1, 0..=15)));
        assert_eq!(chunked.next(), None);
    }
}
