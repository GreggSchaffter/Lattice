use std::collections::LinkedList;

trait Graph {
    type E;

    fn edges(&self) -> Vec<Self::E>;
}

struct MyGraph;

impl Graph for MyGraph {
    type E = i32;

    fn edges(&self) -> Vec<Self::E> {
        Vec::new()
    }
}
fn main(){
  let graph = MyGraph;
  let obj = Box::new(graph) as Box<Graph<E=i32>>;
}
