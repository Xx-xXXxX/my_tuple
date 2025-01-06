

/// The Node of a tuple, contains the value and Tuple of values after
#[derive(Clone,PartialEq, Eq,Default)]
pub struct TupleNode<T,TNext>(pub T,pub TNext);

#[derive(Clone,PartialEq, Eq,Default,Debug)]
pub struct TupleEnd;

impl<T,TNext> TupleNode<T,TNext> {
    
    #[inline]
    pub fn unwrap(self)->(T,TNext) {
        (self.0,self.1)
    } 
}

/// converts object to its tuple version.
/// 
/// In all conitions, structs are needed for different impl
/// so the only (and must) way to use a 'tuple' is transform them into TupleNode and TupleEnd
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

impl<'a,T,TNext,TNextRef> IntoTuple for &'a TupleNode<T,TNext>
    where &'a TNext:IntoTuple<Output = TNextRef>
{
    type Output=TupleNode<&'a T,TNextRef>;
    #[inline]
    fn into_tuple(self)->Self::Output {
        TupleNode( &self.0,self.1.into_tuple())
    }
}

impl<'a,T,TNext,TNextRefMut> IntoTuple for &'a mut TupleNode<T,TNext>
    where &'a mut TNext:IntoTuple<Output = TNextRefMut>
{
    type Output=TupleNode<&'a mut T,TNextRefMut>;
    #[inline]
    fn into_tuple(self)->Self::Output {
        TupleNode( &mut self.0,self.1.into_tuple())
    }
}
