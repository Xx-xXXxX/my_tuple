


use std::marker::PhantomData;


use crate::tuple::TupleEnd;

use super::tuple::TupleNode;

pub trait Spliter<Tuple>{
    type Output0;
    type Output1;
    fn split(t:Tuple)->(Self::Output0,Self::Output1);
    fn split_get1(t:Tuple)->Self::Output1;
}
#[derive(Default)]
pub struct SelectNode<Next>{p:PhantomData<Next>}

#[derive(Default)]
pub struct SelectEnd;

impl Spliter<TupleEnd> for SelectEnd
{
    type Output0=TupleEnd;

    type Output1=TupleEnd;
    #[inline]
    fn split(_:TupleEnd)->(Self::Output0,Self::Output1) {
        (TupleEnd,TupleEnd)
    }
    
    fn split_get1(_:TupleEnd)->Self::Output1 {
        TupleEnd
    }
}

impl<T,TNext> Spliter<TupleNode<T,TNext>> for SelectEnd
{
    type Output0=TupleEnd;

    type Output1=TupleNode<T,TNext>;
    #[inline]
    fn split(t:TupleNode<T,TNext>)->(Self::Output0,Self::Output1) {
        (TupleEnd,t)
    }
    
    fn split_get1(t:TupleNode<T,TNext>)->Self::Output1 {
        t
    }
}


impl<T,TNext,SNext> Spliter<TupleNode<T,TNext>> for SelectNode<SNext>
    where //Tuple:traits::TupleNode,
    SNext:Spliter<TNext>
{
    type Output0=TupleNode<T,< SNext as Spliter<TNext>> ::Output0>;
    
    type Output1=< SNext as Spliter<TNext>> ::Output1;
    #[inline]
    fn split(t:TupleNode<T,TNext>)->(Self::Output0,Self::Output1) {
        let (v,n)=t.unwrap();
        let (o1,o2)=< SNext as Spliter<TNext>>::split(n);
        (TupleNode(v, o1),o2)
    }
    
    fn split_get1(t:TupleNode<T,TNext>)->Self::Output1 {
        <SNext as Spliter<TNext>>::split_get1(t.unwrap().1)
    }
}
