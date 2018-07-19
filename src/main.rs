use std::any::Any;

trait AsAny {
    fn as_any(&self) -> &Any;
}

impl<T: Any> AsAny for T {
    fn as_any(&self) -> &Any { self }
}

trait Conv {
    fn to_string(&self) -> String;
}

trait AConv: Conv + AsAny {}

struct Count (
    i32,
);

impl Conv for Count {
    fn to_string(&self) -> String {
        "TODO".to_string()
    }
}

impl AConv for Count {}

struct Custom {
    v: Box<dyn AConv>,
}

fn getv(c: Custom) {
    if let Some(cnt) = (*c.v).as_any().downcast_ref::<Count>() {
        println!("got cnt {}", cnt.0);
    } else {
        println!("not cnt?");
    }
}

fn main() {
    println!("helo");
    let c = Custom { v: Box::new(Count(13)) };
    getv(c);
}

/*
*/
