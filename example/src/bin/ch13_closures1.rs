/// 捕获引用或者移动所有权
fn main() {
    {
        let list = vec![1, 2, 3];
        println!("Before defining closure: {:?}", list);

        // 定义并调用一个捕获不可变引用的闭包，即Fn trait
        let only_borrows = || println!("From closure: {:?}", list);

        println!("Before calling closure: {:?}", list);
        only_borrows();
        println!("After calling closure: {:?}", list);
    }

    {
        let mut list = vec![1, 2, 3];
        println!("Before defining closure: {:?}", list);

        // 定义并调用一个捕获可变引用的闭包，即FnMut trait
        let mut borrows_mutably = || list.push(7);

        // println!("Before calling closure: {:?}", list); // 这里println!无法编译
        borrows_mutably();
        println!("After calling closure: {:?}", list);
    }

    {
        use std::thread;
        let list = vec![1, 2, 3];
        println!("Before defining closure: {:?}", list);
        // 使用 move 来强制闭包为线程获取 list 的所有权，即FnOnce trait
        thread::spawn(move || println!("From thread: {:?}", list))
            .join()
            .unwrap();
    }
}
