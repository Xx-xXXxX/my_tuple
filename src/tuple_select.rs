


use std::marker::PhantomData;

use wacky_traits::mapper::Mapper;

use super::{tuple::{TupleNode}};

pub trait Selector<Tuple>{
    type Output1;
    type Output2;
    
    fn split(t:Tuple)->(Self::Output1,Self::Output2);
}
#[derive(Default)]
pub struct SelectNode<Next>{p:PhantomData<Next>}

#[derive(Default)]
pub struct SelectEnd;

impl Selector<()> for SelectEnd
{
    type Output1=();

    type Output2=();

    fn split(t:())->(Self::Output1,Self::Output2) {
        ((),t)
    }
}

impl<T,TNext> Selector<TupleNode<T,TNext>> for SelectEnd
{
    type Output1=();

    type Output2=TupleNode<T,TNext>;

    fn split(t:TupleNode<T,TNext>)->(Self::Output1,Self::Output2) {
        ((),t)
    }
}


impl<T,TNext,SNext> Selector<TupleNode<T,TNext>> for SelectNode<SNext>
    where //Tuple:traits::TupleNode,
    SNext:Selector<TNext>
{
    type Output1=TupleNode<T,< SNext as Selector<TNext>> ::Output1>;
    
    type Output2=< SNext as Selector<TNext>> ::Output2;
    
    fn split(t:TupleNode<T,TNext>)->(Self::Output1,Self::Output2) {
        let (v,n)=t.unwrap();
        let (o1,o2)=< SNext as Selector<TNext>>::split(n);
        (TupleNode::new(v, o1),o2)
    }
    /*
    type Output=SNext::Output;
    #[inline]
    fn get(t:TupleNode<T,TNext>)->Self::Output {
        SNext::get(t.next())
    } */
}

/*
mapper
pub trait ModifyTuple<Tuple> {
    type Output;
}
 */
/*
pub trait MapTuple<Tuple,TMapper>:Selector<Tuple>
    where TMapper:Mapper< <Self as Selector<Tuple>>::Output >
{
    type Output;
    fn apply(t:Tuple,mapper:&TMapper)->Self::Output{
        mapper.map(<Self as Selector<Tuple>>::get(t))
    }
    //type Output= <TMapper as Mapper< <Self as Selector<Tuple>>::Output >>::Output ;
}

impl<Tuple,TMapper> MapTuple<Tuple,TMapper> for SelectEnd 
    where TMapper:Mapper< <Self as Selector<Tuple>>::Output >,
    Tuple:traits::Tuple
{
    type Output=<TMapper as Mapper< Tuple >>::Output;
}

impl<TOld,TNext,SNext,TMapper> MapTuple<TupleNode<TOld,TNext>,TMapper> for SelectNode<SNext>
    where 
    SNext:MapTuple<TNext,TMapper>,
    TMapper:Mapper< <Self as Selector<TupleNode<TOld,TNext>>>::Output >
{
    type Output=TupleNode<TOld, <SNext as MapTuple<TNext,TMapper> >::Output >;
}
 */