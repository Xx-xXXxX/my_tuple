


use std::marker::PhantomData;

use crate::{traits::{self}, tuple::{TupleEnd, TupleNode}};

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
    #[inline]
    fn get(t:Tuple)->Self::Output {
        t.get()
    }
}

impl<T,TNext,SNext> Selector<TupleNode<T,TNext>> for SelectNode<SNext>
    where //Tuple:traits::TupleNode,
    SNext:Selector<TNext>
{
    type Output=SNext::Output;
    #[inline]
    fn get(t:TupleNode<T,TNext>)->Self::Output {
        SNext::get(t.next())
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

pub trait ChangeTypeOfTuple<Tuple,NewT> {
    type Output;
}

impl<TOld,TNew> ChangeTypeOfTuple<TupleEnd<TOld>,TNew> for SelectEnd  {
    type Output=TupleEnd<TNew>;
}

impl<TOld,TNext,TNew> ChangeTypeOfTuple<TupleNode<TOld,TNext>,TNew> for SelectEnd  {
    type Output=TupleNode<TNew,TNext>;
}

impl<TOld,TNew,TNext,SNext> ChangeTypeOfTuple<TupleNode<TOld,TNext>,TNew> for SelectNode<SNext>
    where 
    SNext:ChangeTypeOfTuple<TNext,TNew>
{
    type Output=TupleNode<TOld, <SNext as ChangeTypeOfTuple<TNext,TNew>>::Output >;
}
