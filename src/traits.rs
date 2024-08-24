pub trait Tuple{
    /// type of the value of the Tuple
    type T;
    /// get the value of the Tuple
    fn get(self)->Self::T;
}

pub trait TupleEnd:Tuple {
    fn unwrap(self)->Self::T;
}

pub trait TupleNode:Tuple {
    type TNext;
    fn next(self)->Self::TNext;

    fn unwrap(self)->(Self::T,Self::TNext);
}