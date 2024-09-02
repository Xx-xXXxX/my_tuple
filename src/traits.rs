pub trait Tuple{
    /// type of the value of the Tuple
    type T;
    /// get the value of the Tuple
    fn get(self)->Self::T;
}