use std::marker::PhantomData;

use crate::traits;

pub trait Selector<Tuple>{
    type Output;
    fn get(t:Tuple)->Self::Output;
}
#[derive(Default)]
pub struct SelectNode<Next>{p:PhantomData<Next>}

pub struct SelectEnd;

impl<Tuple> Selector<Tuple> for SelectEnd
    where Tuple:traits::Tuple
{
    type Output=Tuple::T;
    fn get(t:Tuple)->Self::Output {
        t.get()
    }
}

impl<Tuple,Next> Selector<Tuple> for SelectNode<Next>
    where Tuple:traits::TupleNode,
    Next:Selector<Tuple::TNext>
{
    type Output=Next::Output;
    fn get(t:Tuple)->Self::Output {
        Next::get(t.next())
    }
} 