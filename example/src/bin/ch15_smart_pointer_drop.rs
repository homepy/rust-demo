/// 使用 Drop Trait 运行清理代码  

/// 结构体 CustomSmartPointer，其实现了放置清理代码的 Drop trait
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    {
        let c = CustomSmartPointer {
            data: String::from("my stuff"),
        };
        let d = CustomSmartPointer {
            data: String::from("other stuff"),
        };
        println!("CustomSmartPointers created.");
    } // 在值离开作用域后，自动调用drop
    println!("--------");
    {
        let c = CustomSmartPointer {
            data: String::from("some data"),
        };
        println!("CustomSmartPointer created.");
        drop(c); // 在值离开作用域之前，调用 std::mem::drop 显式清理
        println!("CustomSmartPointer dropped before the end of main.");
    }
}
