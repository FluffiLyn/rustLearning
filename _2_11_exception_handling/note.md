## 2.11 错误处理与返回值
### 2.11.1 panic深入解析
文件读取操作发生在系统启动阶段，所以一旦文件读取失败，那么系统启动也将失败，意味着该失败是不可恢复的错误。

对于这些严重到影响程序运行的错误，触发`panic`是很好的解决方式。
#### panic的触发方式
1、被动触发：如数组越界。

2、主动触发：`panic!`宏。当调用执行该宏时，程序会打印出一个错误信息，展开报错点往前的函数调用堆栈，最后退出程序。
```rust
fn main() {
    panic!("crash and burn");
}
```
输出：
```
thread 'main' panicked at 'crash and burn', src/main.rs:2:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

以下会详细讲解主动panic。

#### backtrace 栈展开
在真实场景中，错误往往涉及到很长的调用链甚至会深入第三方库，如果没有栈展开技术，错误将难以跟踪处理。

假设有一个代码：
```rust
fn main() {
    let v = vec![1, 2, 3];

    v[99];
}
```
很明显，数组越界访问了，会导致panic。报错信息如下：
```
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:4:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```
我们已经知道问题发生的位置，但如果我们想知道该问题之前经过了哪些调用环节，我们可以根据提示使用`RUST_BACKTRACE=1 cargo run`（类Unix系统）或`$env:RUST_BACKTRACE=1 ; cargo run`（Windows）来再一次运行程序，输出如下：
```
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:4:5
stack backtrace:
   0: rust_begin_unwind
             at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/library/std/src/panicking.rs:517:5
   1: core::panicking::panic_fmt
             at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/library/core/src/panicking.rs:101:14
   2: core::panicking::panic_bounds_check
             at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/library/core/src/panicking.rs:77:5
   3: <usize as core::slice::index::SliceIndex<[T]>>::index
             at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/library/core/src/slice/index.rs:184:10
   4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
             at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/library/core/src/slice/index.rs:15:9
   5: <alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index
             at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/library/alloc/src/vec/mod.rs:2465:9
   6: world_hello::main
             at ./src/main.rs:4:5
   7: core::ops::function::FnOnce::call_once
             at /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/library/core/src/ops/function.rs:227:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
这便是栈展开（或栈回溯）。

拓展：要获取栈展开信息，还需要开启`debug`标志。`debug`标志在使用`cargo run`或者`cargo build`时自动开启：
```shell
$ cargo run --debug
```

#### panic时的终止方式
程序提供了两种方式来处理终止流程：**栈展开**和**直接终止**。

1、栈展开：是默认方式，可以给出充分的报错信息和栈调用信息，便于事后的问题复盘。

2、直接终止：不清理数据就直接退出程序，善后工作交与操作系统来负责。在`Cargo.toml`中添加如下配置：
```toml
[profile.release]
panic = 'abort'
```
这样，当程序panic时，将直接终止程序。

- 对于绝大多数用户，使用默认选择是最好的，但是当你关心最终编译出的二进制可执行文件大小时（即不生成调试信息和其他额外数据），那么可以尝试去使用直接终止的方式。

#### 线程panic
如果是main线程，则程序会终止，如果是其它子线程，该线程会终止，但是不会影响main线程。因此，尽量不要在main线程中做太多任务，将这些任务交由子线程去做，就算子线程panic也不会导致整个程序的结束。

#### panic!宏的使用时机
先介绍一下`Result`类型，`Result`是一个枚举类型，有两个成员：`Ok`和`Err`。`Ok`表示成功，`Err`表示失败。

对于`Result`返回值的处理，最简单粗暴的就是`unwrap`和`expect`。

`unwrap`：如果`Result`是`Ok`，则返回`Ok`中的值；如果是`Err`，则调用`panic!`宏。

panic!宏使用时机如下：

##### 1、示例、原型、测试
这些是需要快速地搭建代码的场景，不需要考虑错误处理，直接使用`unwrap`或`expect`来处理。

并且回头做错误处理时，可以全局搜索这些方法，不遗漏地进行替换。

##### 2、你认为这段代码一定不会出错
用`unwrap`等方法直接进行处理。

例如：
```rust
use std::net::IpAddr;
let home: IpAddr = "127.0.0.1".parse().unwrap();
```
`"127.0.0.1"`就是ip地址，因此我们知道parse方法一定会成功，那么就可以直接用unwrap方法进行处理。

相反地，如果是用户输入的数据，那么就必须要进行错误处理。你也不想一天崩溃几万次吧。

##### 3、可能导致全局有害状态时
**有害状态**大致分为几类：
* 非预期的错误
* 后续代码的运行会受到显著影响
* 内存安全的问题

#### 拓展
展开的过程是一帧一帧的去回溯整个栈，每个帧的数据都会随之被丢弃，但是在展开过程中，你可能会遇到被用户标记为`catching`的帧（通过`std::panic::catch_unwind()`函数标记），此时用户提供的`catch`函数会被调用，展开也随之停止：当然，如果`catch`选择在内部调用`std::panic::resume_unwind()`函数，则展开还会继续。