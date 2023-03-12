# rust-demo
https://doc.rust-lang.org/book/  
https://kaisery.github.io/trpl-zh-cn/title-page.html  


## cargo
cargo new       创建项目。  
cargo run       一步构建并运行项目。  
cargo check     在不生成二进制文件的情况下构建项目来检查错误。  
cargo build     构建项目，target/debug 下生成可执行文件。  
cargo build --release       构建项目，target/release 下生成可执行文件。  

cargo update        升级 crate。  

cargo doc --open        构建所有本地依赖提供的文档，并在浏览器中打开。  

cargo test      在测试模式下编译代码并运行生成的测试二进制文件。  
cargo test -- --test-threads=1 --show-output  
cargo test -- --ignored      仅运行被忽略的测试。  
cargo test -- --include-ignored   

cargo fmt   
cargo fix   
cargo clippy   

---

match 类似switch

## Ownership， lifetime of references   
https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html   
__所有权Ownership__ is a set of rules that govern how a Rust program __manages memory__.

在C和Rust中，编译期能确定内存size的数据才能放在栈上；不能确定内存size的放在堆上（malloc、Box<T>），栈上仅放其指针（指针的size是确定的）。  
https://www.bilibili.com/video/BV1KT4y167f1/

作用域scope  
drop  

在 C++ 中，这种 item 在生命周期结束时释放资源的模式有时被称作 资源获取即初始化（Resource Acquisition Is Initialization (RAII)）。  

__reference 引用 &__：使用值但不获取其所有权。创建一个引用的行为称为借用（borrowing）。  
默认不允许修改引用的值；而可变引用（mutable reference）为&mut 。  
&String，表示一个指向String的引用。  
（C语言中，int * p = &a_int;  * p 是一个int，所以p是指向int的指针；&a_int 是对a_int变量做&取地址运算。）  

__dereferencing 解引用 *__：  
Deref trait  

__lifetime__
生命周期的主要目标是避免悬垂引用（dangling references），后者会导致程序引用了非预期引用的数据。  

## Trait
类似Java interface   
Copy trait，基本类型实现了Copy trait。   
Drop trait   
函数调用时，发生copy或者move。  
   
## Trait Object
运行时多态。  


## 多态（polymorphism）
impl Trait：静态分发（static dispatch）。在编译期就确定了具体返回类型，函数只能返回一种类型。   
dyn Trait：动态分发 （dynamic dispatch）。在运行时才能确定具体返回类型，函数可以返回多种类型。   




## 应用方向
系统级编程语言，基本上可替代Cpp。   

### 操作系统、虚拟化    
- 清华大学的rCore，https://github.com/LearningOS/os-lectures/
- 斯坦福tockOS（RTOS），cs140e
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
