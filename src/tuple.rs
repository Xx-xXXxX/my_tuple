use crate::traits;
use crate::traits::Tuple as tTup;




/// The end of a tuple
#[derive(Clone,PartialEq, Eq)]
pub struct TupleEnd<T>{
    value:T
}

impl<T> tTup for TupleEnd<T> {
    type T=T;
    fn get(self)->T {
        self.value
    }
}

impl<T> TupleEnd<T> {
    pub fn new(value:T)->Self{Self{value}}
}

impl<T> traits::TupleEnd for TupleEnd<T> {
    fn unwrap(self)->Self::T {self.value}
}

#[derive(Clone,PartialEq, Eq)]
pub struct TupleNode<T,TNext>
    //where TNext:Tuple
{
    value:T,
    next:TNext
}

impl<T,TNext/*: Tuple */> tTup for TupleNode<T,TNext> {
    type T=T;

    fn get(self)->Self::T {
        self.value
    }
}

impl<T,TNext/*: Tuple */> TupleNode<T,TNext> {
    pub fn new(value:T,next:TNext)->Self{Self{value,next}}

    pub fn next(self)->TNext {
        self.next
    }

    pub fn unwrap(self)->(T,TNext) {
        (self.value,self.next)
    }
}

impl<T,TNext/*: Tuple */> traits::TupleNode for TupleNode<T,TNext> {
    type TNext=TNext;

    fn next(self)->TNext {
        self.next
    }

    fn unwrap(self)->(T,TNext) {
        (self.value,self.next)
    }
}

/// converts object to its tuple version
pub trait IntoTuple {
    type Output;
    fn into_tuple(self)->Self::Output;
}

impl<'a,T> IntoTuple for &'a TupleEnd<T> {
    type Output = TupleEnd<&'a T>;
    fn into_tuple(self)->Self::Output{TupleEnd::new(&self.value)}
}
impl<'a,T> IntoTuple for &'a mut TupleEnd<T> {
    type Output = TupleEnd<&'a mut T>;
    fn into_tuple(self)->Self::Output{TupleEnd::new(&mut self.value)}
}

impl<'a,T,TNext/*:Tuple */> IntoTuple for &'a TupleNode<T,TNext>
    where &'a TNext:IntoTuple
{
    type Output=TupleNode<&'a T,<&'a TNext as IntoTuple>::Output>;
    fn into_tuple(self)->Self::Output {
        TupleNode::new( &self.value,self.next.into_tuple())
    }
}

impl<'a,T,TNext/*:Tuple */> IntoTuple for &'a mut TupleNode<T,TNext>
    where &'a mut TNext:IntoTuple
{
    type Output=TupleNode<&'a mut T,<&'a mut TNext as IntoTuple>::Output>;
    fn into_tuple(self)->Self::Output {
        TupleNode::new( &mut self.value,self.next.into_tuple())
    }
}
/// macro to easily create a tuple with inferred type
/// # Example
/// m_tup!(1,2f64,String::from("3"))
#[macro_export]
macro_rules! m_tup {
    ($v:expr) => {
        $crate::tuple::TupleEnd::new($v)
    };
    ($v:expr,$($then:expr),*)=>{
        $crate::tuple::TupleNode::new($v,m_tup!($($then),*))
    }
}

/// macro to easily create a tuple type
/// # Example
/// m_tup_t!(i32,f64,String)
#[macro_export]
macro_rules! m_tup_t {
    ($v:ty) => {
        $crate::tuple::TupleEnd::<$v>
    };
    ($v:ty,$($then:ty),*)=>{
        $crate::tuple::TupleNode::<$v,m_tup_t!($($then),*)>
    }
}
