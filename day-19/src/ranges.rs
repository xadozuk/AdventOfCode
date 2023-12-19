use std::{ops::{Range, Sub}, iter::Sum};

#[derive(Debug, Clone)]
pub struct MultiRange<T>
{
    ranges: Vec<Range<T>>
}

impl<T> MultiRange<T>
    where T: Eq + Ord + Sub<Output = T> + Default + Sum + Copy
{
    pub fn from_range(range: Range<T>) -> MultiRange<T>
    {
        MultiRange { ranges: vec![range] }
    }

    pub fn len(&self) -> T
    {
        self.ranges.iter().map(|r| {
            if r.is_empty() { T::default() }
            else { r.end - r.start }
        }).sum()
    }

    pub fn exclude(&mut self, excl_range: &Range<T>)
    {
        let mut remaining_ranges = vec![];

        for range in &self.ranges
        {
            if let Some(mut remaining) = exclude(&range, &excl_range)
            {
                remaining_ranges.append(&mut remaining);
            }

        }

        self.ranges = remaining_ranges;
    }

    pub fn intersect(&mut self, incl_range: &Range<T>)
    {
        let mut remaining_ranges = vec![];

        for range in &self.ranges
        {
            if let Some(remaining) = intersect(&range, &incl_range)
            {
                remaining_ranges.push(remaining);
            }
        }

        self.ranges = remaining_ranges;
    }
}

fn intersect<T>(a: &Range<T>, b: &Range<T>) -> Option<Range<T>>
    where T: Ord + Copy
{
    let range = (a.start.max(b.start))..(a.end.min(b.end));

    if range.is_empty() { None }
    else { Some(range) }
}

fn exclude<T>(range: &Range<T>, exclude: &Range<T>) -> Option<Vec<Range<T>>>
    where T: PartialEq + PartialOrd + Copy
{
    let mut result = vec![];

    let left = range.start..exclude.start;
    let right = exclude.end..range.end;

    if !left.is_empty()  { result.push(left) }
    if !right.is_empty() { result.push(right) }

    if result.is_empty() { None }
    else { Some(result) }
}


#[cfg(test)]
mod tests
{
    use super::{intersect, exclude};

    #[test]
    fn test_intersect()
    {
        assert_eq!(Some(5..10), intersect(&(1..10), &(5..15)));
        assert_eq!(Some(20..50), intersect(&(1..100), &(20..50)));
        assert_eq!(None, intersect(&(0..10), &(20..30)));
        assert_eq!(None, intersect(&(0..10), &(10..30)));
        assert_eq!(None, intersect(&(0..10), &(50..30)));
    }

    #[test]
    fn test_exclude()
    {
        assert_eq!(Some(vec![(1..5)]), exclude(&(1..10), &(5..15)));
        assert_eq!(Some(vec![(5..10)]), exclude(&(1..10), &(1..5)));
        assert_eq!(Some(vec![1..3, 6..10]), exclude(&(1..10), &(3..6)));
        assert_eq!(None, exclude(&(1..10), &(0..100)));
    }
}
