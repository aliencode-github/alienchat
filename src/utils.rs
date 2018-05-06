pub mod utils{
    pub fn remove_ref_from_ref_vec<'a,T:PartialEq>(list:&mut Vec<&'a T>,element:&'a T) -> bool{
        list.iter()
            .position(|&n| n == element )
            .map(|e| list.remove(e))
            .is_some()
    }
/*
    pub fn remove_ref_from_vec<T: PartialEq>(list:&mut Vec<T>, element:&T) -> bool{
        list.iter()
            .position(|ref n| n == &element )
            .map(|e| list.remove(e))
            .is_some()
    }
*/
}