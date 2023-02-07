# rust-demo

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
scope  
drop  

在 C++ 中，这种 item 在生命周期结束时释放资源的模式有时被称作 资源获取即初始化（Resource Acquisition Is Initialization (RAII)）。  

__reference 引用 &__：使用值但不获取其所有权。创建一个引用的行为称为借用（borrowing）。  
默认不允许修改引用的值；而可变引用（mutable reference）为&mut 。  

__dereferencing 解引用 *__：  

## Tailt
类似Java interface   

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
