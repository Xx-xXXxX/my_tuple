
/// macro to easily create a tuple with inferred type
/// # Example
/// m_tup!(1,2f64,String::from("3"))
#[macro_export]
macro_rules! m_tup {
    
    ($v:expr) => {
        $crate::tuple::TupleNode::new($v,$crate::tuple::TupleEnd)
    };
    ($v:expr,$($then:expr),*)=>{
        $crate::tuple::TupleNode::new($v,m_tup!($($then),*))
    }
}

/// macro to easily create a tuple type
/// # Example
/// m_tup_t!(i32,f64,String)
#[macro_export]
macro_rules! m_tup_t {
    ($v:ty) => {
        $crate::tuple::TupleNode::<$v,$crate::tuple::TupleEnd>
    };
    ($v:ty,$($then:ty),*)=>{
        $crate::tuple::TupleNode::<$v,m_tup_t!($($then),*)>
    }
}

#[macro_export]
macro_rules! m_tup_sel_def {
    (@inner $prev:ident,$cur:ident) => {
        type $cur = crate::tuple_select::SelectNode<$prev>;
    };
    (@inner $prev:ident,$cur:ident,$($thens:ident),+) => {
        type $cur = crate::tuple_select::SelectNode<$prev>;
        m_tup_sel_def!{@inner $cur,$($thens),*}
    };
    ($cur:ident,$($thens:ident),+)=>{
        type $cur = crate::tuple_select::SelectEnd;
        m_tup_sel_def!{@inner $cur,$($thens),*}
    }
}