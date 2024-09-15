
use std::marker::PhantomData;

use crate::tuple::TupleEnd;

use super::tuple::TupleNode;
use wacky_traits::collector::Collectable;
use wacky_traits::collector::Collector;
use wacky_traits::mapper::Mapper;
/*
use common_traits::visitor::*;

impl<T,TVisitor> Visitable<TVisitor> for TupleEnd<T>
    where TVisitor:Visitor<T>
{
    type T=T;
    fn visited(self,visitor:TVisitor) {
        visitor.visit(self.get())
    }
}

impl<T,TNext,TVisitor> Visitable<TVisitor> for TupleNode<T,TNext>
    where 
    TVisitor:Visitor<T>,
    TNext:Visitable<TVisitor>,
    TVisitor:Visitor< <TNext as Visitable<TVisitor>>::T >,
    TVisitor:Clone
{
    type T=T;
    fn visited(self,visitor:TVisitor) {
        let (value,next)=self.unwrap();
        //let visitor2=visitor;
        visitor.clone().visit(value);
        next.visited(visitor);
    }
} */
/// collector which converts accepted values into a tuple
#[derive(Clone)]
pub struct CollectAsTuple;

impl<TIn> Mapper<TIn> for CollectAsTuple {
    type Output=TupleNode<TIn,TupleEnd>;

    fn map(self,value:TIn)->(Self::Output,CollectAsTuple) {
       (TupleNode::new(value, TupleEnd),CollectAsTuple)
    }
}

impl<TIn,TNext> Collector<TIn,TNext> for CollectAsTuple
    //where TNext:;
{
    type Output=TupleNode<TIn,TNext>;

    fn collect(self,value:TIn,next:TNext)->(<Self as Collector<TIn,TNext>>::Output,CollectAsTuple) {
        (TupleNode::new(value, next),CollectAsTuple)
    }
}
/*
impl<T,TCollector> Collectable<TCollector> for ()
    where TCollector:Mapper<T>
{
    type Output=TCollector::Output;

    fn collected(self,collector:TCollector)->(Self::Output,TCollector) {
        collector.map(self.get())
    }
} */

/*
impl<T,TCollector> Collectable<TCollector> for TupleNode<T,AUnit>
    where TCollector:Mapper<T>//+Clone
{
    type Output= <TCollector as Mapper<T>>::Output ;

    fn collected(self,collector:TCollector)->(Self::Output,TCollector) {
        let (value,next)=self.unwrap();
        let (n,c2)=next.collected(collector);
        c2.collect(value,n)
    }
}
 */
impl<T,TCollector> Collectable<TCollector> for TupleNode<T,TupleEnd>
    where 
    TCollector:Mapper<T>//+Clone
{
    type Output= <TCollector as Mapper<T>>::Output ;

    fn collected(self,collector:TCollector)->(Self::Output,TCollector) {
        let (value,_next)=self.unwrap();
        collector.map(value)
    }
}
impl<T,TT,TTNext,TCollector> Collectable<TCollector> for TupleNode<T,TupleNode<TT,TTNext>>
    where TupleNode<TT,TTNext>:Collectable<TCollector>,
    TCollector:Collector<T,<TupleNode<TT,TTNext> as Collectable<TCollector>>::Output>//+Clone
{
    type Output= <TCollector as Collector<T,<TupleNode<TT,TTNext> as Collectable<TCollector>>::Output>>::Output ;

    fn collected(self,collector:TCollector)->(Self::Output,TCollector) {
        let (value,next)=self.unwrap();
        let (n,c2)=next.collected(collector);
        c2.collect(value,n)
    }
}

/* 
impl<T,TNext,TCollector> Collectable<TCollector> for TupleNode<T,TNext>
    where TNext:Collectable<TCollector>,
    TCollector:Collector<T,<TNext as Collectable<TCollector>>::Output>//+Clone
{
    type Output= <TCollector as Collector<T,<TNext as Collectable<TCollector>>::Output>>::Output ;

    fn collected(self,collector:TCollector)->(Self::Output,TCollector) {
        let (value,next)=self.unwrap();
        let (n,c2)=next.collected(collector);
        c2.collect(value,n)
    }
}
*/