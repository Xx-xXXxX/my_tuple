use wacky_traits::ref_self::folder::Foldable;

use crate::{m_tup, m_tup_t, patterns::folder::{CollectAsTupleFolder, ForTupleToBackwardFold}, utils::tuple_bind::{tuple_bind, TupleBind}};

#[test]
fn ded_1(){
    type _TEESET=TupleBind<m_tup_t!(i32,f32),m_tup_t!(f32,i32)>;
    let _teeset2:_TEESET=ForTupleToBackwardFold(m_tup!(1,2f32)).folded(&CollectAsTupleFolder, m_tup!(3f32,2));
    //let _teeset=tuple_bind(m_tup!(1,2f32), m_tup!(3f32,2));
}