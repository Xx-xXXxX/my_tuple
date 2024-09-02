
/*
impl<T> traits::TupleEnd for TupleEnd<T> {
    fn unwrap(self)->Self::T {self.value}
} */

/// The Node of a tuple, contains the value and Tuple of values after
#[derive(Clone,PartialEq, Eq)]
pub struct TupleNode<T,TNext>
    //where TNext:Tuple
{
    value:T,
    next:TNext
}

#[derive(Clone,PartialEq, Eq)]
pub struct TupleEnd;


impl<T,TNext> TupleNode<T,TNext> {
    #[inline]
    pub fn new(value:T,next:TNext)->Self{Self{value,next}}
    /*
    #[inline]
    pub fn next(self)->TNext {
        self.next
    }
    pub fn get(self)->T {
        self.value
    } */
    #[inline]
    pub fn unwrap(self)->(T,TNext) {
        (self.value,self.next)
    }
}
/*
impl<T,TNext/*: Tuple */> traits::TupleNode for TupleNode<T,TNext> {
    type TNext=TNext;

    fn next(self)->TNext {
        self.next
    }

    fn unwrap(self)->(T,TNext) {
        (self.value,self.next)
    }
} */

/// converts object to its tuple version
pub trait IntoTuple {
    type Output;
    fn into_tuple(self)->Self::Output;
}

impl<'a> IntoTuple for &'a TupleEnd {
    type Output = TupleEnd;
    #[inline]
    fn into_tuple(self)->Self::Output{TupleEnd}
}
impl<'a> IntoTuple for &'a mut TupleEnd {
    type Output = TupleEnd;
    #[inline]
    fn into_tuple(self)->Self::Output{TupleEnd}
}

impl<'a,T,TNext/*:Tuple */> IntoTuple for &'a TupleNode<T,TNext>
    where &'a TNext:IntoTuple
{
    type Output=TupleNode<&'a T,<&'a TNext as IntoTuple>::Output>;
    #[inline]
    fn into_tuple(self)->Self::Output {
        TupleNode::new( &self.value,self.next.into_tuple())
    }
}

impl<'a,T,TNext/*:Tuple */> IntoTuple for &'a mut TupleNode<T,TNext>
    where &'a mut TNext:IntoTuple
{
    type Output=TupleNode<&'a mut T,<&'a mut TNext as IntoTuple>::Output>;
    #[inline]
    fn into_tuple(self)->Self::Output {
        TupleNode::new( &mut self.value,self.next.into_tuple())
    }
}
pub trait TuplePushBack<Tuple2> {
    type Output;
    fn push_back(self,tup2:Tuple2)->Self::Output;
}

impl<Tup2> TuplePushBack<Tup2> for TupleEnd {
    type Output=Tup2;
    
    #[inline]
    fn push_back(self,tup2:Tup2)->Self::Output {
        tup2
    }
}

impl<T,Next,Tup2> TuplePushBack<Tup2> for TupleNode<T,Next>
    where Next:TuplePushBack<Tup2>
{
    type Output=TupleNode<T,Next::Output>;

    fn push_back(self,tup2:Tup2)->Self::Output {
        let (v,n)=self.unwrap();
        TupleNode::new(v,n.push_back(tup2))
    }
}