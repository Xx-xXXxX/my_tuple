use crate::{patterns::folder::{CollectAsTupleFolder, ForTupleToBackwardFold}, tuple::{TupleEnd, TupleNode}};
use wacky_traits::ref_self::{folder::Foldable, mapper::MapperOnce};
use crate::tuple_select::Spliter;

pub type TupleBind<Tup1,Tup2>=<ForTupleToBackwardFold<Tup1> as Foldable<CollectAsTupleFolder,Tup2>>::Output;



pub fn tuple_bind<Tup1,Tup2>(t1:Tup1,t2:Tup2)->TupleBind<Tup1,Tup2>
    where ForTupleToBackwardFold<Tup1>:Foldable<CollectAsTupleFolder,Tup2>
{
    ForTupleToBackwardFold(t1).folded(&CollectAsTupleFolder, t2)
}