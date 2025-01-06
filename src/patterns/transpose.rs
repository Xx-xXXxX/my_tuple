

use crate::{m_tup, m_tup_t, tuple::{IntoTuple, TupleEnd, TupleNode}};

pub struct ToMerge2Tup<T1,T2>(pub T1,pub T2);

impl IntoTuple for ToMerge2Tup<TupleEnd,TupleEnd> {
    type Output=TupleEnd;

    fn into_tuple(self)->Self::Output {
        TupleEnd
    }
}

impl<T1,T1Next> IntoTuple for ToMerge2Tup<TupleNode<T1,T1Next>,TupleEnd> {
    type Output=TupleNode<T1,T1Next>;

    fn into_tuple(self)->Self::Output {
        self.0
    }
}

impl<T1,T1Next,T2,T2Next> IntoTuple for ToMerge2Tup<TupleNode<T1,T1Next>,TupleNode<T2,T2Next>> 
    where ToMerge2Tup<T1Next,T2Next>:IntoTuple
{
    type Output=TupleNode<m_tup_t!(T1,T2), <ToMerge2Tup<T1Next,T2Next> as IntoTuple>::Output >;

    fn into_tuple(self)->Self::Output {
        let (v1,n1)=(self.0.0,self.0.1);//self.0.unwrap();
        let (v2,n2)=(self.1.0,self.1.1);
        TupleNode(m_tup!(v1,v2),<ToMerge2Tup<T1Next,T2Next> as IntoTuple>::into_tuple(ToMerge2Tup(n1,n2)))
    }
}

/// useful when you need another way to iterate
/// e.g.
/// let a_mat:AMatType=m_tup!(
///     m_tup!(2,3f32,4f32),
///     m_tup!(5,6f32,7f32)
/// );
/// let a_mat_t=a_mat.transpose();
/// assert!(a_mat_t,m_tup!(
///     m_tup!(2,5),
///     m_tup!(3f32,6f32),
///     m_tup!(4f32,7f32))
/// )
pub trait Transposable {
    type Output;
    fn transpose(self)->Self::Output;
}

impl Transposable for TupleEnd {
    type Output=TupleEnd;

    fn transpose(self)->Self::Output {
        TupleEnd
    }
}

impl<T,Next> Transposable for TupleNode<T,Next>
    where 
        Next:Transposable,
        ToMerge2Tup<T, <Next as Transposable>::Output >:IntoTuple
{
    type Output= <ToMerge2Tup<T, <Next as Transposable>::Output > as IntoTuple>::Output ;

    fn transpose(self)->Self::Output {
        ToMerge2Tup(self.0,self.1.transpose()).into_tuple()
    }
}