use std::ops::Range;

pub fn has_intersect<T>(a: &Range<T>, b: &Range<T>) -> bool
    where T: Ord
{
    // Put a on the "left" and b on the "right" of axis
    if b.start < a.start { return has_intersect(b, a) }

    // If first range end before second one, no intersect
    if a.end <= b.start { return false }

    return true
}

pub fn intersect<T>(a: &Range<T>, b: &Range<T>) -> Range<T>
    where T: Ord + Default + Copy
{
    if !has_intersect(a, b) { return T::default()..T::default() }

    let intersect_start = a.start.max(b.start);
    let intersect_end   = a.end.min(b.end);

    return intersect_start..intersect_end;
}