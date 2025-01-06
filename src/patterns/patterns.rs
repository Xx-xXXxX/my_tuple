

use std::cell::RefCell;

use crate::m_tup;
use crate::m_tup_t;
use crate::tuple::IntoTuple;
use crate::tuple::TupleEnd;
use crate::tuple_select::Spliter;
use crate::utils;
use crate::utils::TupleGet;
use crate::utils::TupleNodeGetT;

use super::tuple::TupleNode;
use wacky_traits::contains::Contains;
/*
use wacky_traits::ref_self::collector::Collectable;
use wacky_traits::ref_self::collector::Collector;
use wacky_traits::ref_self::collectors::MapperCollector;
 */
use wacky_traits::inv_iterator::inv_typed_iterator::InvIterCollectable;
use wacky_traits::inv_iterator::inv_typed_iterator::InvTypedIterator;
use wacky_traits::ref_self::folder::Foldable;
use wacky_traits::ref_self::folder::Folder;
use wacky_traits::ref_self::mapper::Mapper;
use wacky_traits::ref_self::visitor::Visitee;
use wacky_traits::ref_self::visitor::Visitor;
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

    fn map(&self,value:TIn)->Self::Output {
       TupleNode::new(value, TupleEnd)
    }
}
/*
impl<TIn,TNext> Collector<TIn,TNext> for CollectAsTuple
    //where TNext:;
{
    type Output=TupleNode<TIn,TNext>;

    fn collect(&self,value:TIn,next:TNext)-><Self as Collector<TIn,TNext>>::Output {
        TupleNode::new(value, next)
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

    fn collected(self,collector:&TCollector)->Self::Output {
        let (value,_next)=self.unwrap();
        collector.map(value)
    }
}
impl<T,TT,TTNext,TCollector> Collectable<TCollector> for TupleNode<T,TupleNode<TT,TTNext>>
    where TupleNode<TT,TTNext>:Collectable<TCollector>,
    TCollector:Collector<T,<TupleNode<TT,TTNext> as Collectable<TCollector>>::Output>//+Clone
{
    type Output= <TCollector as Collector<T,<TupleNode<TT,TTNext> as Collectable<TCollector>>::Output>>::Output ;

    fn collected(self,collector:&TCollector)->Self::Output {
        let (value,next)=self.unwrap();
        let n=next.collected(collector);
        collector.collect(value,n)
    }
}

pub struct ForTupleToReverseCollect<T>(T);

impl<T,TCollector> Collectable<TCollector> for ForTupleToReverseCollect<TupleNode<T,TupleEnd>>
    where 
    TCollector:Mapper<T>//+Clone
{
    type Output= <TCollector as Mapper<T>>::Output ;

    fn collected(self,collector:&TCollector)->Self::Output {
        let (value,_next)=self.0.unwrap();
        collector.map(value)
    }
}
impl<T,TT,TTNext,TCollector> Collectable<TCollector> for ForTupleToReverseCollect<TupleNode<T,TupleNode<TT,TTNext>>>
    where TupleNode<TT,TTNext>:Collectable<TCollector>,
    TCollector:Collector<T,<TupleNode<TT,TTNext> as Collectable<TCollector>>::Output>//+Clone
{
    type Output= <TCollector as Collector<T,<TupleNode<TT,TTNext> as Collectable<TCollector>>::Output>>::Output ;

    fn collected(self,collector:&TCollector)->Self::Output {
        let (value,next)=self.0.unwrap();
        let n=next.collected(collector);
        collector.collect(value,n)
    }
}
*/
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

/*
impl<TVisitor> Visitee<TVisitor> for TupleEnd{
    fn visited(self,_visitor:&mut TVisitor) {
    }
}

impl<T,TNext,TVisitor> Visitee<TVisitor> for TupleNode<T,TNext>
    where TNext:Visitee<TVisitor>,
    TVisitor:Visitor<T>
{
    fn visited(self,visitor:&mut TVisitor) {
        let (v,n)=self.unwrap();
        visitor.visit(v);
        n.visited(visitor);
    }
} */



pub struct ForTupleToBeSelected<'a,T>(pub &'a T);

impl<'a,Tup,TSel> Mapper<TSel> for ForTupleToBeSelected<'a,Tup>
    where &'a Tup:IntoTuple,
    TSel:Spliter< <&'a Tup as IntoTuple>::Output >,
    <TSel as Spliter<<&'a Tup as IntoTuple>::Output>>::Output1: TupleNodeGetT,
    <<TSel as Spliter<<&'a Tup as IntoTuple>::Output>>::Output1 as TupleNodeGetT>::T: 'a
{
    type Output=TupleGet<<&'a Tup as IntoTuple>::Output,TSel>;

    fn map(&self,value:TSel)->Self::Output {
        utils::tuple_get(self.0.into_tuple(), &value)
    }
}

/*
/// WARNING: this may create multiple &mut
pub  struct TupleSelectedBySelectorMut<'a,T>(pub &'a mut T);

impl<'a,Tup,TSel> Mapper<TSel> for TupleSelectedBySelectorMut<'a,Tup>
    where &'a mut Tup:IntoTuple,
    TSel:Spliter< <&'a mut Tup as IntoTuple>::Output >,
    <TSel as Spliter<<&'a mut Tup as IntoTuple>::Output>>::Output1: TupleNodeGetT,
    <<TSel as Spliter<<&'a mut Tup as IntoTuple>::Output>>::Output1 as TupleNodeGetT>::T: 'a
{
    type Output=TupleGet<<&'a mut Tup as IntoTuple>::Output,TSel>;

    fn map(&self,value:TSel)->Self::Output {
        
        let anothermut=unsafe {
            transmute_copy::<TupleSelectedBySelectorMut<'a,Tup>,&'a mut Tup>(self)
        }; 
        utils::tuple_get(anothermut.into_tuple(), &value)
    }
} */
/*
type TupleToTupleRefCell<T>=<T as Collectable<MapperCollector<ForToRefCell,CollectAsTuple> >>::Output;

//type CRCCAT<T>=<T as Collectable<MapperCollector<ToRefCell, CollectAsTuple>>>::Output;
pub struct TupleSelectedMut<T>
    where T: wacky_traits::collector::Collectable<wacky_traits::collectors::MapperCollector<ForToRefCell, CollectAsTuple>>
{
    pub values:TupleToTupleRefCell<T>
}

impl<T> TupleSelectedMut<T> 
where T: Collectable<MapperCollector<ForToRefCell, CollectAsTuple>>
{
    pub fn new(v:T)->Self{
        Self{
            values:v.collected(&MapperCollector(ForToRefCell, CollectAsTuple))
        }
    }
}



impl<T> TupleSelectedMut<T> 
where T: Collectable<MapperCollector<ForToRefCell, CollectAsTuple>>,
    TupleToTupleRefCell<T>:Collectable<MapperCollector<ForRefCellIntoInner,CollectAsTuple>>,
    <TupleToTupleRefCell<T> as Collectable<MapperCollector<ForRefCellIntoInner,CollectAsTuple>> >::Output :Into<T>
{
    pub fn unwrap(self)->T{
        self.values.collected(&MapperCollector(ForRefCellIntoInner,CollectAsTuple)).into()
    }
}

impl<Tup,TSel,
    TupRC,
    TupRCR,
    Seled
> Mapper<TSel> for TupleSelectedMut<Tup>
    where Tup: Collectable<MapperCollector<ForToRefCell, CollectAsTuple>,Output = TupRC>,
        for<'a> &'a TupRC : IntoTuple <Output = TupRCR>,
        TSel:Spliter<TupRCR,Output1 = Seled>,
        Seled:TupleNodeGetT
        //CRCCAT<Tup>:,
        //TSel:Spliter<TupleToTupleRefCell<Tup>>, 
        //<TSel as Spliter<<Tup as Collectable<MapperCollector<ToRefCell, CollectAsTuple>>>::Output>>::Output1: TupleNodeGetT
{
    type Output=<Seled as TupleNodeGetT>::T;//TupleGet<TupleToTupleRefCell<Tup>,TSel>;

    fn map(&self,value:TSel)->Self::Output {
        utils::tuple_get((&self.values).into_tuple(), &value)
    }
} */
/*
pub struct CollectAsTupleInvIter<TOld>(pub TOld);

impl<T,TOld> InvTypedIterator<T> for CollectAsTupleInvIter<TOld> {
    type Next=CollectAsTupleInvIter<TupleNode<T,TOld>>;

    fn collect(self,value:T)->Self::Next {
        CollectAsTupleInvIter(TupleNode(value,self.0))
    }
}

pub struct ForTupleToForwardCollect<T>(pub T);
impl<T> Contains for ForTupleToForwardCollect<T> {
    type Output=T;

    fn unwrap(self)->Self::Output {
        self.0
    }
}

impl<T,TNext,TInvIter,TInvIterNext,Output> InvIterCollectable<TInvIter> for ForTupleToForwardCollect<TupleNode<T,TNext>> 
    where TInvIter:InvTypedIterator<T,Next = TInvIterNext>,

    TNext:InvIterCollectable<TInvIterNext,Output = Output>
{
    type Output=Output;

    fn collected(self,collector:TInvIter)->Self::Output {
        let t=self.0;
        let n=collector.collect(t.0);
        return t.1.collected(n);
    }
}

impl<TInvIter> InvIterCollectable<TInvIter> for ForTupleToForwardCollect<TupleEnd> {
    type Output=TInvIter;

    fn collected(self,collector:TInvIter)->Self::Output {
        collector
    }
}

pub struct ForTupleToBackwardCollect<T>(pub T);

impl<T> Contains for ForTupleToBackwardCollect<T> {
    type Output=T;

    fn unwrap(self)->Self::Output {
        self.0
    }
}

impl<T,TNext,TInvIter,TInvIterNext,Output> InvIterCollectable<TInvIter> for ForTupleToBackwardCollect<TupleNode<T,TNext>> 
    where TInvIterNext:InvTypedIterator<T,Next = Output>,

    TNext:InvIterCollectable<TInvIter,Output = TInvIterNext>,
{
    type Output=Output;

    fn collected(self,collector:TInvIter)->Self::Output {
        let t=self.0;
        let n=t.1.collected(collector);
        return n.collect(t.0);
    }
}

impl<TInvIter> InvIterCollectable<TInvIter> for ForTupleToBackwardCollect<TupleEnd> {
    type Output=TInvIter;

    fn collected(self,collector:TInvIter)->Self::Output {
        collector
    }
}
 */