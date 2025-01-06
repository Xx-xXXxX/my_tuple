use std::marker::PhantomData;

use wacky_traits::ref_self::mapper::{Mapper, MapperOnce};

use crate::tuple::{TupleEnd, TupleNode};
use crate::tuple_select::*;


pub trait TupleNodeGetT {
    type T;
    fn get(self)->Self::T;
}
impl<T,TNext> TupleNodeGetT for TupleNode<T,TNext> {
    type T = T;
    
    fn get(self)->Self::T {
        self.0
    }
}

pub type TupleGet<TTuple,TSpliter>=<<TSpliter as Spliter<TTuple>>::Output1 as TupleNodeGetT>::T;

pub fn tuple_get<TTuple,TSpliter:Spliter<TTuple>>(tuple:TTuple,_sel:&TSpliter)->TupleGet<TTuple,TSpliter>
    where <TSpliter as Spliter<TTuple>>::Output1: TupleNodeGetT
{
    TSpliter::split_get1(tuple).get()
}

#[derive(Default,Clone, Copy)]
pub struct ToCountSpliter<T>{p:PhantomData<T>}

pub trait ToCountSpliterTrait {
    const COUNT:usize;
}

impl ToCountSpliterTrait for ToCountSpliter<SelectEnd> {
    const COUNT:usize=0;
}

impl<SN> ToCountSpliterTrait for ToCountSpliter<SelectNode<SN>>
    where ToCountSpliter<SN>:ToCountSpliterTrait
{
    const COUNT:usize=  <ToCountSpliter<SN> as ToCountSpliterTrait>::COUNT+1;
}

pub struct SpliterMapCount;

impl<T> Mapper<T> for SpliterMapCount
    where ToCountSpliter<T>:ToCountSpliterTrait
{
    type Output=usize;
    #[inline]
    fn map(&self,_value:T)->Self::Output {
        <ToCountSpliter<T> as ToCountSpliterTrait>::COUNT
    }
}
/*
pub trait ToBindTupleTrait<Tuple2> {
    type Output;
    fn bind(self,tup2:Tuple2)->Self::Output;
}

impl<Tup2> ToBindTupleTrait<Tup2> for TupleEnd {
    type Output=Tup2;
    
    #[inline]
    fn bind(self,tup2:Tup2)->Self::Output {
        tup2
    }
}

impl<T,Next,Tup2> ToBindTupleTrait<Tup2> for TupleNode<T,Next>
    where Next:ToBindTupleTrait<Tup2>
{
    type Output=TupleNode<T,Next::Output>;

    #[inline]
    fn bind(self,tup2:Tup2)->Self::Output {
        let (v,n)=self.unwrap();
        TupleNode::new(v,n.bind(tup2))
    }
}
 */