use wacky_traits::mapper::{Mapper, MapperOnce};

use crate::tuple::{TupleBind, TupleNode};
use crate::tuple_select::*;

pub type TupleTypeSplitOperate<Tuple,TSelector,TMapper>=
    <
        <TSelector as Spliter<Tuple>>::Output1 
    as TupleBind< 
        <TMapper as MapperOnce<
            <TSelector as Spliter<Tuple>>::Output2>
        >::Output>
    >::Output;
pub fn tuple_type_split_operate<Tuple,TSelector,TMapper>(t:Tuple,_selector:&TSelector,mapper:TMapper)->TupleTypeSplitOperate<Tuple,TSelector,TMapper>
    where TSelector:Spliter<Tuple>,
    TMapper:MapperOnce<<TSelector as Spliter<Tuple>>::Output2>,
    <TSelector as Spliter<Tuple>>::Output1: TupleBind<<TMapper as MapperOnce<<TSelector as Spliter<Tuple>>::Output2>>::Output>
{
    let (a,b)=TSelector::split(t);
    let res=(mapper).map_once(b);
    a.bind(res)
}

pub fn pop_front<T,TNext>(t:TupleNode<T,TNext>)->TNext{t.1}
pub struct ToPushFront<T>(pub T);

impl<T,TTup> MapperOnce<TTup> for ToPushFront<T> {
    type Output=TupleNode<T,TTup>;

    fn map_once(self,value:TTup)->Self::Output {
        TupleNode::new(self.0, value)
    }
}
pub fn push_front<T>(v:T)->ToPushFront<T>
{
   ToPushFront(v)
}
pub struct ToChangeFront<T>(pub T);

impl<T,TT,TTNext> MapperOnce<TupleNode<TT,TTNext>> for ToChangeFront<T> {
    type Output=TupleNode<T,TTNext>;

    fn map_once(self,value:TupleNode<TT,TTNext>)->Self::Output {
        TupleNode::new(self.0, value.unwrap().1)
    }
}
pub fn change_front<T>(v:T)->ToChangeFront<T>
{
    ToChangeFront(v)
}

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

pub type TupleGet<TTuple,TSpliter>=<<TSpliter as Spliter<TTuple>>::Output2 as TupleNodeGetT>::T;

pub fn tuple_get<TTuple,TSpliter:Spliter<TTuple>>(tuple:TTuple,_sel:&TSpliter)->TupleGet<TTuple,TSpliter>
    where <TSpliter as Spliter<TTuple>>::Output2: TupleNodeGetT
{
    TSpliter::split(tuple).1.get()
}