use crate::{traits::Tuple, tuple::{TupleEnd, TupleNode}};

impl<T> ToString for TupleEnd<T>
    where T:ToString,//+Clone
    TupleEnd<T>:Clone
{
    fn to_string(&self) -> String {
        self.clone().get().to_string()
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
