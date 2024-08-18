use std::cell::{Cell, RefCell, Ref};

#[derive(Debug, Default)]
struct W {
    v: String,
}

thread_local!(static TL_V:RefCell<Vec<W>> = RefCell::<Vec<W>>::new(vec![]));

#[derive(Clone)]
#[derive(Debug)]
struct C {
    name: String,
    line: u32,
}

thread_local!(static C_STACK:RefCell<Vec<C>> = RefCell::<Vec<C>>::new(vec![]));

struct Stacker {
}

impl Stacker {
    fn new(name: &str, line: u32) -> Stacker {
        C_STACK.with(|v| {
            v.borrow_mut().push(C {name: String::from(name), line: line});
        });

        Stacker{}
    }

    fn stack() -> Vec<C> {
        let mut r: Vec<C> = vec![];
        C_STACK.with(|v| {
            r = v.borrow_mut().clone();
            //println!("{:?}", &v);
        });

        r
    }
}

impl Drop for Stacker {
    fn drop(&mut self) {
        C_STACK.with(|v| {
            v.borrow_mut().pop();
        });
    }
}

//#[macro_export]
macro_rules! test {
    ( $x:literal ) => {
        let _prusstak = Stacker::new($x, line!());
    }
}

fn main() {
    test!("main");

    // Cell
    let c = Cell::<i32>::new(10);
    c.set(c.get() + 1);
    println!("{:?}", &c);

    // RefCell
    let rc = RefCell::<Vec<i32>>::new(vec![1, 2]);
    {
        let mut a = rc.borrow_mut();
        a.push(100);

        println!("{:?}", &a);
    }
    {
        let a = rc.borrow();
        println!("{:?}", &a);
    }

    // RefCell + Vec + Struct
    #[derive(Debug)]
    #[derive(Default)]
    struct V {
        v: String,
    }
    let rc = RefCell::<Vec<V>>::new(vec![]);
    {
        rc.borrow_mut().push(V{v:String::from("str")});
        println!("{:?}", &rc);
    }
    {
        rc.borrow_mut().pop();
        println!("{:?}", &rc);
    }

    // thread_local
    TL_V.with(|v| {
        v.borrow_mut().push(W{v:String::from("www")});
        println!("{:?}", &v);
    });
    TL_V.with(|v| {
        v.borrow_mut().pop();
        println!("{:?}", &v);
    });

    println!("1 {:?}", Stacker::stack());
    {
        test!("in1");
        println!("1 {:?}", Stacker::stack());
        {
            test!("in2");
            println!("2 {:?}", Stacker::stack());
        }
        println!("1 {:?}", Stacker::stack());
    }
    println!("1 {:?}", Stacker::stack());
}
