use std::collections::LinkedList;

// DataObj: This stores the data within the Column.
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
// Column: Stores each dataobj of particular type.
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
// Table: Stores the columns
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
// Database: Stores a set of tables
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
struct Server{
    dbs: Vec<db>.
}
impl Server{
    fn new() -> Server{
        Server{
            dbs: Vec::<db>::new(),
        }
    }
}
fn main(){
    //TODO: Implement server aspect.
}
