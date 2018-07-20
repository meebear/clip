use std::any::Any;

trait AsAny {
    fn as_any(&self) -> &Any;
}

impl<T: Any> AsAny for T {
    fn as_any(&self) -> &Any { self }
}

trait Conv: AsAny {
    fn to_string(&self) -> String;
}

struct Count (
    i32,
);

impl Conv for Count {
    fn to_string(&self) -> String {
        "TODO".to_string()
    }
}

struct Custom {
    v: Box<dyn Conv>,
}

fn getvt<T: Conv + 'static>(c: &Custom) -> Option<&T> {
    if let Some(value) = (*c.v).as_any().downcast_ref::<T>() {
        Some(value)
    } else {
        None
    }
}

fn main() {
    println!("helo");
    let c = Custom { v: Box::new(Count(13)) };
    let v: Option<&Count> = getvt(&c);
    match v {
        Some(cnt)=> println!("got cnt {}", cnt.0),
        None=> println!("n"),
    }
}
