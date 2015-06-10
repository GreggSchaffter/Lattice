use std::collections::LinkedList;

struct RowQuery<T>{
    map: LinkedList<T>,
}
impl<T> RowQuery<T>{
    fn new() -> RowQuery<T>{
        let mut d = LinkedList::<T>::new();
        RowQuery{
            map: d,
        }
    }
}
struct HashMap<T>{
    map: Vec<RowQuery<T>>,
}
impl<T> HashMap<T>{
    fn new() -> HashMap<T>{
        let mut d: Vec<RowQuery<T>> = vec![];
        HashMap{
            map: d,
        }
    }
}
fn main(){

}
