
#[cfg(test)]
mod examples{
    use crate::*;
    use tuple::*;
    use patterns::*;
    use tuple_select::Selector;
    use wacky_traits::{collector::*,collectors::*,mapper::*,mappers::*};
    use macros::*;
    /*
    fn to_string<'a,T>(a:&'a T)->String
        where T:ToString
    {
        a.to_string()
    } */
    #[derive(Clone)]
    pub struct tostr;
    impl<'a,T:ToString> Mapper<&'a T> for tostr {
        type Output=String;
    
        fn map(self,value:&'a T)->(Self::Output,tostr) {
            (value.to_string(),tostr)
        }
    }

    

    
    pub struct GetString{
        pub str:String
    }
    impl<'a,T:ToString> Mapper<&'a T> for &'a mut GetString {
        type Output=();
    
        fn map(self,value:&'a T)->(Self::Output,Self) {
            (self.str.push_str(&value.to_string()),self)
        }
    }
    #[test]
    fn example(){

        // create a tuple (1,2,3,1f32,2f32,3f32)
        let mut a_tuple=m_tup!(1,2,3,1f32,2f32,3f32);

        // type of a_tuple
        type ATupleType=m_tup_t!(i32,i32,i32,f32,f32,f32);

        // create a tuple of &mut of a_tuple's items
        let a_tuple_mut=(&mut a_tuple).into_tuple();

        // type of a_tuple_mut
        type ATupleTypeMut=<ATupleType as IntoTuple>::Output;

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
            (& a_tuple).into_tuple().collected( 
                MapperCollector(
                    tostr, // do to_string
                    CollectAsTuple // connect each String together into tuple
                )).0;
        
        assert!(
            a_tuple_str_tup==
            m_tup!(String::from("2"),String::from("2"),String::from("3"),String::from("1"),String::from("2"),String::from("3"))
        );
        
        // join Strings with ", "
        let a_tuple_str=a_tuple_str_tup.collected(
            FnCollector(// treat fn as collector
                |a,b|{format!("{}, {}",a,b)}
            )).0;
        
        assert_eq!(a_tuple_str,"2, 2, 3, 1, 2, 3");

        let mut get_str=GetString{str:String::new()};
        
        (& a_tuple).into_tuple().collected( 
            MapperCollector(
                &mut get_str, // do to_string
                () // returns ()
            ));
        // notice this is reversed
        assert_eq!(get_str.str,"321322");

        // get selectors, SelC get the third value of tuple
        m_tup_sel_def!{SelA,SelB,SelC}
        
        let (get_c_a,get_c_b)=SelC::split((&a_tuple).into_tuple());
        let get_c=get_c_b.unwrap().0;
        assert_eq!(*get_c,3)

    }
    mod example_type_program{
        
        
    }
}