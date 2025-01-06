use crate::{patterns::folder::{CollectAsTupleFolder, ForTupleToBackwardFold}, tuple::{TupleEnd, TupleNode}};
use wacky_traits::ref_self::{folder::Foldable, mapper::MapperOnce};
use crate::tuple_select::Spliter;


pub type TupleTypeSplitOperate<Tuple,TSelector,TMapper>=
    TupleBind<
        <TSelector as Spliter<Tuple>>::Output0 ,
        <TMapper as 
            MapperOnce<
                <TSelector as Spliter<Tuple>>::Output1
            >
        >::Output>
    ;

pub fn tuple_type_split_operate<Tuple,TSelector,TMapper>(t:Tuple,_selector:&TSelector,mapper:TMapper)->TupleTypeSplitOperate<Tuple,TSelector,TMapper>
    where TSelector:Spliter<Tuple>,
    TMapper:MapperOnce<<TSelector as Spliter<Tuple>>::Output1>,
    ForTupleToBackwardFold<<TSelector as Spliter<Tuple>>::Output0>:Foldable<CollectAsTupleFolder,
        <TMapper as 
            MapperOnce<
                <TSelector as Spliter<Tuple>>::Output1
            >
        >::Output
    >
{
    let (a,b)=TSelector::split(t);
    let res=(mapper).map_once(b);
    tuple_bind(a, res)
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