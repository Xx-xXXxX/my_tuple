use crate::tuple::TupleEnd;

use super::tuple::TupleNode;
/*
impl ToString for ()
{
    fn to_string(&self) -> String {String::new()}
} */

impl<T> ToString for TupleNode<T,TupleEnd>
    where T:ToString,
    TupleNode<T,TupleEnd>:Clone
{
    fn to_string(&self) -> String {
        let (value,_next)=self.clone().unwrap();
        format!("{}",value.to_string())
    }
}


impl<T,TNext> ToString for TupleNode<T,TNext>
    where T:ToString,TNext:ToString,
    TupleNode<T,TNext>:Clone
{
    fn to_string(&self) -> String {
        let (value,next)=self.clone().unwrap();
        format!("{}, {}",value.to_string(),next.to_string())
    }
}
