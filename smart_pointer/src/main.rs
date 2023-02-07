// Box<T> 和 Rc<T>
enum ListB {
    Consb(i32, Box<ListB>),
    Nil,
}
enum ListR {
    Consr(i32, Rc<ListR>),
    Nil,
}

use std::rc::Rc;

fn main() {
    {
        // Box<T>
        use crate::ListB::{Consb, Nil};
        let list = Consb(1, Box::new(Consb(2, Box::new(Consb(3, Box::new(Nil))))));

        let x = 5;
        let y = Box::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    {
        // Rc<T>
        use crate::ListR::{Consr, Nil};

        let a = Rc::new(Consr(5, Rc::new(Consr(10, Rc::new(Nil)))));
        println!("count after creating a = {}", Rc::strong_count(&a));
        let b = Consr(3, Rc::clone(&a));
        println!("count after creating b = {}", Rc::strong_count(&a));
        {
            let c = Consr(4, Rc::clone(&a));
            println!("count after creating c = {}", Rc::strong_count(&a));
        }
        println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    }

    {
        // Drop trait
        let c = CustomSmartPointer {
            data: String::from("my stuff"),
        };
        let d = CustomSmartPointer {
            data: String::from("other stuff"),
        };
        println!("CustomSmartPointers created.");

        drop(c);
        println!("CustomSmartPointer c dropped before the end of main.");
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
