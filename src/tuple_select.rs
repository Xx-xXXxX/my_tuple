use std::marker::PhantomData;

use crate::{traits::{self, Tuple}, tuple::{TupleEnd, TupleNode}};

pub trait Selector<Tuple>{
    type Output;
    fn get(t:Tuple)->Self::Output;
}
#[derive(Default)]
pub struct SelectNode<Next>{p:PhantomData<Next>}

#[derive(Default)]
pub struct SelectEnd;

impl<Tuple> Selector<Tuple> for SelectEnd
    where Tuple:traits::Tuple
{
    type Output=Tuple::T;
    fn get(t:Tuple)->Self::Output {
        t.get()
    }
}

impl<T,TNext,Next> Selector<TupleNode<T,TNext>> for SelectNode<Next>
    where //Tuple:traits::TupleNode,
    Next:Selector<TNext>
{
    type Output=Next::Output;
    fn get(t:TupleNode<T,TNext>)->Self::Output {
        Next::get(t.next())
    }
}
#[macro_export]
macro_rules! m_tup_sel_def {
    (@inner $prev:ident,$cur:ident) => {
        type $cur = crate::tuple_select::SelectNode<$prev>;
    };
    (@inner $prev:ident,$cur:ident,$($thens:ident),+) => {
        type $cur = crate::tuple_select::SelectNode<$prev>;
        m_tup_sel_def!{@inner $cur,$($thens),*}
    };
    ($cur:ident,$($thens:ident),+)=>{
        type $cur = crate::tuple_select::SelectEnd;
        m_tup_sel_def!{@inner $cur,$($thens),*}
    }
}