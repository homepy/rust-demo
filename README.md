# rust-demo
《The Rust Programming Language》  https://doc.rust-lang.org/book/  
《Rust权威指南》  https://kaisery.github.io/trpl-zh-cn/
《Asynchronous Programming in Rust》  https://rust-lang.github.io/async-book/


## 1 cargo
cargo new       创建项目  
cargo run       一步构建并运行项目  
cargo check     在不生成二进制文件的情况下构建项目来检查错误  
cargo build     构建项目，target/debug 下生成可执行文件  
cargo build --release       构建项目，target/release 下生成可执行文件  

cargo update        升级 crate  

cargo doc --open        构建所有本地依赖提供的文档，并在浏览器中打开  

cargo test      在测试模式下编译代码并运行生成的测试二进制文件  
cargo test -- --test-threads=1 --show-output  
cargo test -- --ignored      仅运行被忽略的测试。  
cargo test -- --include-ignored   

cargo fmt   
cargo fix   
cargo clippy   

---

match 类似switch

## 2 Ownership, lifetime of references   
https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html   
__所有权Ownership__ is a set of rules that govern how a Rust program __manages memory__.

### 内存中的栈（Stack）与堆（Heap）
在C和Rust中，编译期能确定内存size的数据才能放在栈上；不能确定内存size的放在堆上（malloc、Box\<T\>），栈上仅放其指针（指针的size是确定的）。  
可视化 Rust 各数据结构的内存布局  https://www.bilibili.com/video/BV1KT4y167f1/


### 变量作用域scope  
mem::drop()
core::ops::Drop Trait
在 C++ 中，item 在生命周期结束时释放资源的模式有时被称作 RAII资源获取即初始化（Resource Acquisition Is Initialization）。 


### 变量与数据交互的方式：copy或move
函数调用时，发生copy或者move。  
  
### References and Borrowing
__reference 引用 &__：使用值但不获取其所有权。创建一个引用的行为称为借用（borrowing）。  
默认不允许修改引用的值；而可变引用（mutable reference）为&mut 。  
如&String，表示一个指向String的引用。  
（C语言中，int * p = &a_int;  * p 是一个int，所以p是指向int的指针；&a_int 是对a_int变量做&取地址运算）  

__dereferencing 解引用 *__：  
std::ops::Deref trait  

### lifetime
生命周期的主要目标是避免悬垂引用（dangling references），后者会导致程序引用了非预期引用的数据。  


## 3 OOP
### 封装（encapsulation）
Rust 是面向对象的：结构体struct和枚举enum包含数据，而 impl 块提供了在结构体和枚举之上的方法。虽然带有方法的结构体和枚举并不被称为 Object，但是他们提供了与对象相同的功能。
pub

### 继承（Inheritance） 与 多态（polymorphism）
继承（Inheritance），Rust无。  
代码复用使用Trait；多态（polymorphism）则使用泛型、trait bounds（bounded parametric polymorphism）、trait objects。
#### Generic Types 泛型

#### Trait
类似Java interface   
std::marker::Copy trait，基本类型实现了Copy trait。   
std::ops::Drop trait   

   
#### Trait Object
运行时多态。  


#### 多态
impl Trait：静态分发（static dispatch）。在编译期就确定了具体返回类型，函数只能返回一种类型。   
dyn Trait：动态分发 （dynamic dispatch）。在运行时才能确定具体返回类型，函数可以返回多种类型。   


## 4 智能指针
alloc::boxed::Box
alloc::rc::Rc
core::cell::RefCell


## 5 并发
std::thread
std::sync::mpsc
std::sync::Mutex
std::sync::Arc

std::marker::{Sync, Send}

## 9 应用方向
系统级编程语言，基本上可替代Cpp。   
RustChinaConf 2020  https://www.bilibili.com/video/BV1Yy4y1e7zR/?spm_id_from=333.999.0.0   
RustChinaConf 2021~2022  https://space.bilibili.com/25566598/channel/collectiondetail?sid=603975   


### 操作系统、虚拟化    
- 清华大学的rCore，https://github.com/LearningOS/os-lectures/
- 斯坦福tockOS（RTOS），cs140e，https://cs140e.sergio.bz/
- System76的Redox
- 华为StratoVirt虚拟化
- 亚马逊Firecracker虚拟化
    
### 数据库
- PingCAP的TiKV
- RisingWave时序数据库
- 蚂蚁CeresDB时序数据库

### 中间件
- 分布式存储


### 区块链
- CITA
- Parity的Substrate


### 嵌入式、物联网、机器人、汽车


### 游戏引擎

### WebAsembly

### 客户端
- 字节跳动 飞书
