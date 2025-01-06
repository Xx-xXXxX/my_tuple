use wacky_traits::ref_self::folder::{Foldable, Folder};

use crate::tuple::{TupleEnd, TupleNode};


pub struct ForTupleToForwardFold<T>(pub T);

impl<TFirst,T,TNext,TFolder,
TSecond,Output> Foldable<TFolder,TFirst> for ForTupleToForwardFold<TupleNode<T,TNext>>
    where 
        TFolder:Folder<TFirst,T,Output = TSecond>,
        ForTupleToForwardFold<TNext>:Foldable<TFolder,TSecond,Output = Output>,
{
    type Output=Output;

    fn folded(self,folder:&TFolder,a:TFirst)->Self::Output {
        let t=self.0;
        let sec=folder.fold(a, t.0);
        ForTupleToForwardFold(t.1).folded(folder, sec)
    }
}

impl<TFirst,TFolder> Foldable<TFolder,TFirst> for ForTupleToForwardFold<TupleEnd> {
    type Output=TFirst;

    fn folded(self,_:&TFolder,a:TFirst)->Self::Output {
        a
    }
}

pub struct ForTupleToBackwardFold<T>(pub T);

impl<TFirst,TFolder> Foldable<TFolder,TFirst> for ForTupleToBackwardFold<TupleEnd> {
    type Output=TFirst;

    fn folded(self,_:&TFolder,a:TFirst)->Self::Output {
        a
    }
}

impl<TFirst,T,TNext,TFolder,
TSecond,Output> Foldable<TFolder,TFirst> for ForTupleToBackwardFold<TupleNode<T,TNext>>
    where 
    ForTupleToBackwardFold<TNext>:Foldable<TFolder,TFirst,Output = TSecond>,
    TFolder:Folder<TSecond,T,Output = Output>
{
    type Output=Output;

    fn folded(self,folder:&TFolder,a:TFirst)->Self::Output {
        let t=self.0;
        let sec=ForTupleToBackwardFold(t.1).folded(folder, a);
        folder.fold(sec, t.0)
    }
}


pub struct CollectAsTupleFolder;

impl<T,TNext> Folder<T,TNext> for CollectAsTupleFolder {
    type Output=TupleNode<TNext,T>;

    fn fold(&self,value:T,next:TNext)->Self::Output {
        TupleNode(next,value)
    }
}