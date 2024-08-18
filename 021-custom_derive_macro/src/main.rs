use custom_derive_macro::{HelloMacro};

trait HelloMacro {
    fn hello_macro();
    fn hello_macro1(&self);
}

#[derive(Debug)]
#[derive(HelloMacro)]
struct Hello {
    value: i32,
}

impl Hello {
    fn method1(&self) {
        println!("hello, method {}", self.value);
    }
}

mod test {
    #[custom_derive_macro::trace]
    pub fn dbl_v(a: u32) -> u32 {
        println!("self {} {}", module_path!(), line!());
        a * a
    }
}

fn main() {
    let h = Hello{value: 10};
    println!("{:?}", Hello::hello_macro());
    println!("{:?}", h.method1());
    println!("{:?}", h.hello_macro1());

    //dbl_v(1);
    println!("{}", test::dbl_v(2));
}
