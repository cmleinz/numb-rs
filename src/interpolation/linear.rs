use ordered_float::Float;

use super::interface::{Interpolation, InterpolationGrid};

pub struct LinearInterpolation<T>
where
    T: Float,
{
    grid: InterpolationGrid<T>,
}

impl<T> Interpolation<T> for LinearInterpolation<T>
where
    T: Float,
{
    fn get_value(&self, x: T) -> Result<T, super::interface::InterpolationError> {
        todo!()
    }

    fn get_values(&self, xs: Vec<T>) -> Result<Vec<T>, super::interface::InterpolationError> {
        todo!()
    }

    fn insert_value(&mut self, x: T) -> Option<T> {
        todo!()
    }

    fn insert_values(&mut self, x: Vec<T>) -> Option<Vec<T>> {
        todo!()
    }
}
