mod interpolation;
mod math;

pub struct Ndarray<T> {
    items: Vec<Vec<Option<T>>>,
    n_size: usize, // Number of arrays
    m_size: usize, // Length of the longest array (remainder are NoneType)
}
