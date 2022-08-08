use ordered_float::{Float, OrderedFloat};
use std::{collections::HashMap, error::Error, fmt::Display};

#[derive(Debug, PartialEq, Eq)]
pub enum InterpolationError {
    InvalidNumberOfPoints,
    NotOneToOne,
    NotMonotonicallyIncreasing,
    SizeMismatch,
    OutOfBounds,
}

impl Display for InterpolationError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(fmt, "TODO")
    }
}

impl Error for InterpolationError {}

/// A structure of (x, y) values which serve as the known points.
///
/// Provides methods for conducting interpolation along a set of known points.
/// x: must be strictly increasing. I.e x_n < x_{n+1} for all n
/// y: The one to one
#[derive(Debug, PartialEq, Eq)]
pub struct Interpolation<T>
where
    T: Float,
{
    x_y: HashMap<OrderedFloat<T>, OrderedFloat<T>>,
}

impl<T> Interpolation<T>
where
    T: Float,
{
    pub fn from(x: Vec<T>, y: Vec<T>) -> Result<Interpolation<T>, InterpolationError> {
        if x.len() != y.len() {
            return Err(InterpolationError::SizeMismatch);
        }
        if !crate::math::monotonically_increasing(&x) {
            return Err(InterpolationError::NotMonotonicallyIncreasing);
        }
        let x_ord = x
            .iter()
            .map(|f| OrderedFloat(f.to_owned()))
            .collect::<Vec<OrderedFloat<T>>>();
        let y_ord = y
            .iter()
            .map(|f| OrderedFloat(f.to_owned()))
            .collect::<Vec<OrderedFloat<T>>>();
        match create_map(x_ord, y_ord) {
            Ok(x_y) => return Ok(x_y),
            Err(e) => return Err(e),
        }
    }

    fn in_x_bounds(&self, val: T) -> bool {
        let min = self.x_y.keys().min().unwrap();
        let max = self.x_y.keys().max().unwrap();
        &val >= min && &val <= max
    }

    // pub fn get_value(&self, x: T) -> Result<f64, InterpolationError> {
    //     // Must check to ensure it is within bound
    //     if self.in_x_bounds(x) {

    //     }
    //     Err(InterpolationError::OutOfBounds)
    // }
}

/// Creates a HashMap of from the x, y values.
///
/// Returns a new HashMap<T, T> from the input values. Errors if the function is input is not one to one.
fn create_map<T>(
    x: Vec<OrderedFloat<T>>,
    y: Vec<OrderedFloat<T>>,
) -> Result<Interpolation<T>, InterpolationError>
where
    T: Float,
{
    let mut map = HashMap::new();
    for i in 0..x.len() {
        if let Some(_) = map.insert(x[i], y[i]) {
            return Err(InterpolationError::NotOneToOne);
        }
    }
    Ok(Interpolation { x_y: map })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn interp_ok() {
        assert!(Interpolation::from(vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]).is_ok())
    }

    #[test]
    fn interp_mismatch_count() {
        assert_eq!(
            Interpolation::from(vec![1.0, 2.0], vec![3.0, 4.0, 5.0]),
            Err(InterpolationError::SizeMismatch)
        )
    }

    #[test]
    fn interp_not_monotonically_increasing() {
        assert_eq!(
            Interpolation::from(vec![1.111, 1.110, 1.112], vec![2.222, 2.223, 2.224]),
            Err(InterpolationError::NotMonotonicallyIncreasing)
        )
    }
}
