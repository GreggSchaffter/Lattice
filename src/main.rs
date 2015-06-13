use std::collections::LinkedList;

struct DataObj{
    data: String,
    t: String,
}
impl DataObj{
    fn new(data: String, t: String) -> DataObj{
        DataObj{
        data: data,
        t: t,
      }
    }
}
struct Column{
    name: String,
    dataobjs: LinkedList<DataObj>,
    t: String,
}
impl Column{
    fn new(name: String, t: String) -> Column{
      Column{
        name: name,
        dataobjs: LinkedList::<DataObj>::new(),
        t: t,
      }
    }
}
struct Table{
    columns: Vec<Column>,
}
impl Table{
    fn new() -> Table{
      Table{
        columns: Vec::<Column>::new(),
      }
    }
}
struct db{
  tables: Vec<Table>,
}
impl db{
    fn new() -> db{
      db{
        tables: Vec::<Table>::new(),
      }
    }
}
fn main(){

}
