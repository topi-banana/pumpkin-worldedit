use std::{
    cmp::Ordering,
    ops::{Bound, RangeBounds, RangeInclusive},
};

use num_traits::Euclid;

pub struct ChunkSplitRange {
    current: i32,
    end: i32,
}

impl ChunkSplitRange {
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

impl Iterator for ChunkSplitRange {
    type Item = (i32, RangeInclusive<i32>);
    fn next(&mut self) -> Option<Self::Item> {
        let (div_current, rem_current) = self.current.div_rem_euclid(&16);
        let (div_end, rem_end) = self.end.div_rem_euclid(&16);
        let range_end = match div_current.cmp(&div_end) {
            Ordering::Less => 15,
            Ordering::Equal => rem_end,
            Ordering::Greater => return None,
        };
        let res = Some((div_current, rem_current..=range_end));
        self.current = (div_current + 1) << 4;
        res
    }
}

#[cfg(test)]
mod test {
    use super::ChunkSplitRange;

    #[test]
    fn check_restore() {
        let mut original = -12345..12345;
        for (chunk, range) in ChunkSplitRange::new(original.clone()) {
            for i in range {
                assert_eq!((chunk << 4) + i, original.next().unwrap());
            }
        }
    }
}
