#[derive(Clone,Debug)]
pub struct Mat<T> {
    pub r: usize,
    pub c: usize,
    pub v: Vec<T>,
}
