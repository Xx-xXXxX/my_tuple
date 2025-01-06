
#[cfg(test)]
mod examples{
    use crate::*;
    use folder::{CollectAsTupleFolder, ForTupleToBackwardFold, ForTupleToForwardFold};
    use transpose::Transposable;
    use tuple::*;
    use patterns::*;
    use tuple_select::Spliter;
    //use utils::split::{tuple_bind, TupleBind};
    use wacky_traits::ref_self::{ collectors::*, folder::Foldable, folders::{FnFolder, MapperFolder}, mapper::*, mappers::FnMapper};
    /*
    fn to_string<'a,T>(a:&'a T)->String
        where T:ToString
    {
        a.to_string()
    } */
    #[derive(Clone)]
    pub struct Tostr;
    impl<'a,T:ToString> Mapper<&'a T> for Tostr {
        type Output=String;
    
        fn map(&self,value:&'a T)->Self::Output {
            value.to_string()
        }
    }

    

    #[test]
    fn example(){
        // type of a_tuple
        type ATupleType=m_tup_t!(i32,i32,i32,f32,f32,f32);

        // create a tuple (1,2,3,1f32,2f32,3f32)
        let mut a_tuple:ATupleType=m_tup!(1,2,3,1f32,2f32,3f32);

        // type of a_tuple_mut
        type ATupleTypeMut<'a>=<&'a mut ATupleType as IntoTuple>::Output;

        // create a tuple of &mut of a_tuple's items
        let a_tuple_mut:ATupleTypeMut=(&mut a_tuple).into_tuple();

        // set the value of a_tuple
        *a_tuple_mut.unwrap().0=2;

        let a_tuple_ref=(& a_tuple).into_tuple();
        assert_eq!(*(a_tuple_ref).unwrap().0,2);

        // you need & or &mut by yourself
        // <del> so I don't need to impl them for 3 times! </del>
        // for better abstraction
        assert_eq!((&a_tuple).into_tuple().to_string(),"2, 2, 3, 1, 2, 3");

        // convert tuple's values to tuple of their to_string
        let a_tuple_str_tup=
            ForTupleToBackwardFold((& a_tuple).into_tuple()).folded( 
                &MapperFolder(
                    Tostr, // do to_string
                    CollectAsTupleFolder // connect each String together into tuple
                ),TupleEnd);
        
        assert!(
            a_tuple_str_tup==
            m_tup!(String::from("2"),String::from("2"),String::from("3"),String::from("1"),String::from("2"),String::from("3"))
        );
        
        // join Strings with ", "
        let a_tuple_str=ForTupleToForwardFold(a_tuple_str_tup).folded(
            &FnFolder(// treat fn as collector
                &|a,b|{format!("{}{}; ",a,b)}
            ),String::new());
        
        assert_eq!(a_tuple_str,"2; 2; 3; 1; 2; 3; ");

        // get selectors, SelC get the third value of tuple
        m_tup_sel_def!{SelA,SelB,SelC}
        
        let (get_c_a,get_c_b)=SelC::split((&a_tuple).into_tuple());
        let get_c=((&get_c_b).into_tuple()).unwrap().0;
        assert_eq!(**get_c,3);
        /* 
        type _TEESET=TupleBind<m_tup_t!(i32,f32),m_tup_t!(f32,i32)>;
        let _teeset2:_TEESET=ForTupleToBackwardFold(m_tup!(1,2f32)).folded(&CollectAsTupleFolder, m_tup!(3f32,2));
        let _teeset=tuple_bind(m_tup!(1,2f32), m_tup!(3f32,2));
            */
        /*
        // bind 2 tuple, insert 123f32 at C
        let _a_tuple_insert_c=tuple_bind(get_c_a.clone(), TupleNode::new(123f32, get_c_b.clone()));
        
        // bind 2 tuple, remove C
        let _a_tuple_del_c=utils::tuple_type_split_operate((&a_tuple).into_tuple(),&SelC::default(), &(crate::utils::pop_front));//get_c_a.bind(get_c_b.unwrap().1);

        let _a_tuple_get_c=utils::tuple_get((&a_tuple).into_tuple(),&SelC::default());
         */

        type  AMatType=m_tup_t!(
            m_tup_t!(i32,f32,f32),
            m_tup_t!(i32,f32,f32)
        );
        let a_mat:AMatType=m_tup!(
            m_tup!(2,3f32,4f32),
            m_tup!(5,6f32,7f32)
        );
        //let a_mat_t_bla=ToTranspose(a_mat);

        //let a_mat_t=a_mat_t_bla.transpose();
        let a_mat_t=a_mat.transpose();
        assert_eq!(a_mat_t,m_tup!(
            m_tup!(2,5),
            m_tup!(3f32,6f32),
            m_tup!(4f32,7f32))
        )
    }
    
}