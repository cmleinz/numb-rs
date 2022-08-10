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
pub struct InterpolationGrid<T>
where
    T: Float,
{
    x_y: HashMap<OrderedFloat<T>, OrderedFloat<T>>,
}

pub trait Interpolation<T>
where
    T: Float,
{
    fn init_grid(xs: Vec<T>, ys: Vec<T>) -> Result<InterpolationGrid<T>, InterpolationError> {
        if xs.len() != ys.len() {
            return Err(InterpolationError::SizeMismatch);
        }
        if !crate::math::monotonically_increasing(&xs) {
            return Err(InterpolationError::NotMonotonicallyIncreasing);
        }
        let x_ord = xs
            .iter()
            .map(|f| OrderedFloat(f.to_owned()))
            .collect::<Vec<OrderedFloat<T>>>();
        let y_ord = ys
            .iter()
            .map(|f| OrderedFloat(f.to_owned()))
            .collect::<Vec<OrderedFloat<T>>>();
        match create_map(x_ord, y_ord) {
            Ok(x_y) => return Ok(x_y),
            Err(e) => return Err(e),
        }
    }

    fn insert_value(&mut self, x: T) -> Option<T>;

    fn insert_values(&mut self, x: Vec<T>) -> Option<Vec<T>>;

    fn get_value(&self, x: T) -> Result<T, InterpolationError>;

    fn get_values(&self, xs: Vec<T>) -> Result<Vec<T>, InterpolationError>;
}

/// Creates a HashMap of from the x, y values.
///
/// Returns a new HashMap<T, T> from the input values. Errors if the function is input is not one to one.
pub fn create_map<T>(
    x: Vec<OrderedFloat<T>>,
    y: Vec<OrderedFloat<T>>,
) -> Result<InterpolationGrid<T>, InterpolationError>
where
    T: Float,
{
    let mut map = HashMap::new();
    for i in 0..x.len() {
        if let Some(_) = map.insert(x[i], y[i]) {
            return Err(InterpolationError::NotOneToOne);
        }
    }
    Ok(InterpolationGrid { x_y: map })
}
