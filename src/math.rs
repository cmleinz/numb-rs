/// Check a vector to determine whether it is monotonically increasing
pub fn monotonically_increasing<T>(vals: &Vec<T>) -> bool
where
    T: PartialOrd,
{
    let mut iter = vals.iter();
    while let (Some(a), Some(b)) = (iter.next(), iter.next()) {
        if a >= b {
            return false;
        }
    }
    true
}
