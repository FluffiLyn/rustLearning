## 2.8 泛型和特征（Generics and Traits）
### 2.8.1 泛型
泛型就是一种多态。
#### 基本用法
```rust
fn add<T>(a: T, b: T) -> T {
    a + b
}
```
在函数名后面的尖括号中声明了一个泛型参数T。这个名称越短越好，除非需要表达含义，否则一个字母是最完美的。

注意，T可以是任何类型，但不是任何类型都支持运算、比较等操作。因此：
* 对于比较，使用`std::cmp::PartialOrd`特征（Trait）对 T 进行限制
* 对于运算，使用`std::ops::Add<Output = T>`特征对 T 进行限制


#### 显式指定泛型类型参数
当编译器无法推断你想要的泛型参数类型时（编译器会列出满足条件的类型），你需要显式指定泛型类型参数。

运行以下代码：
```rs
use std::fmt::Display;

fn create_and_print<T>() where T: From<i32> + Display {
    let a: T = 100.into(); // 创建了类型为 T 的变量 a，它的初始值由 100 转换而来
    println!("a is: {}", a);
}

fn main() {
    create_and_print();
}
```
其中，
* where 关键字用于在泛型函数或类型的定义中指定类型约束。
* From 特征用于定义一种类型如何从另一种类型转换。into() 是 From 特征的一个方法。
* Display 特征用于定义如何格式化输出。println! 宏会调用 Display 特征的实现来格式化输出。


编译器会报错：
```
  |
3 | fn create_and_print<T>() where T: From<i32> + Display {
  |                                   ^^^^^^^^^ required by this bound in `create_and_print`
help: consider specifying the generic argument // 尝试指定泛型参数
  |
9 |     create_and_print::<T>();
  |                     +++++
```
根据提示，我们可以尝试指定泛型参数：`create_and_print::<T>()`：
```rs
use std::fmt::Display;

fn create_and_print<T>() where T: From<i32> + Display {
    let a: T = 100.into(); // 创建了类型为 T 的变量 a，它的初始值由 100 转换而来
    println!("a is: {}", a);
}

fn main() {
    create_and_print::<i64>();// 显式指定泛型参数
}
```

#### 泛型的使用
##### 结构体

```rs
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}
```

如果想让x和y的类型不同，可以这样写：
```rs
struct Point<T, U> {
    x: T,
    y: U,
}
```

但是如果你的结构体变成这鬼样：`struct DontDoThis<T,U,V,W,X>`，那么你需要考虑拆分这个结构体。

##### 枚举
前几章提及的`Option`就是泛型枚举
```rs
enum Option<T> {
    Some(T),
    None,
}
```

还有`Result`：
```rs
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
Result一般用于文件操作、网络请求等可能出错的操作。

##### 方法
```rs
struct Point<T>{
    x: T,
    y: T,
}

impl<T> Point<T>{
    fn x(&self) -> &T{
        &self.x
    }
}

fn main(){
    let p = Point{x: 5, y: 10};
    println!("p.x = {}", p.x());
}
```
我们还能在该结构体的方法中定义额外的泛型参数：
```rs
impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
```
还能针对特定的具体类型，进行方法定义：
```rs
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```
这样就能实现只有f32类型的Point才能调用distance_from_origin方法，其他类型的Point不调用。

#### const泛型
前文提到的泛型是针对数据类型的泛型，而const泛型是针对值的泛型。

例如，在某些情况下不能传引用，只能传值，但是又想要传递一个常量，这时就可以使用const泛型：
```rs
fn display_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    println!("{:?}", arr);
}
fn main() {
    let arr: [i32; 3] = [1, 2, 3];
    display_array(arr);

    let arr: [i32; 2] = [1, 2];
    display_array(arr);
}
```
这里的`const N: usize`就是const泛型，它基于的值类型是`usize`。它替代了数组的长度，这样就能在不引用的情况下传递任意长度的数组。

##### const fn 常量函数
const fn 允许我们在编译期对函数进行求值，从而实现更高效、更灵活的代码设计。

为什么需要 const fn？Rust圣经原话：
* 通常情况下，函数是在运行时被调用和执行的。然而，在某些场景下，我们希望在编译期就计算出一些值，以提高运行时的性能或满足某些编译期的约束条件。例如，定义数组的长度、计算常量值等。

* 有了 const fn，我们可以在编译期执行这些函数，从而将计算结果直接嵌入到生成的代码中。这不仅以高了运行时的性能，还使代码更加简洁和安全。

例如：
```rs
const fn add(a: usize, b: usize) -> usize {
    a + b
}

const RESULT: usize = add(5, 10);

fn main() {
    println!("The result is: {}", RESULT);
}
```

将 const fn 与 const 泛型 结合。例如，创建一个固定大小的缓冲区结构，其中缓冲区大小由编译期计算确定：
```rs
struct Buffer<const N: usize> {
    data: [u8; N],
}

const fn compute_buffer_size(factor: usize) -> usize {
    factor * 1024
}

fn main() {
    const SIZE: usize = compute_buffer_size(4);
    let buffer = Buffer::<SIZE> {
        data: [0; SIZE],
    };
    println!("Buffer size: {} bytes", buffer.data.len());
}
```

#### 泛型的性能
在 Rust 中泛型是**零成本**的抽象，完全不用担心运行时性能。

代价就是编译速度慢、生成文件体积大。

Rust 通过在编译时进行泛型代码的**单态化(monomorphization)**来保证效率：单态化是一个通过填充编译时使用的具体类型，将通用代码转换为特定代码的过程。

编译器寻找所有泛型代码被调用的位置并针对具体类型生成代码。这个单态化过程正是 Rust 泛型在运行时极其高效的原因。

### 2.8.2 特征（Traits）
特征定义了一组可以被共享的行为，只要实现了特征，你就能使用这组行为。

简单来说，特征就是一种接口，是对行为的抽象。

**特征**和**特征约束**是 Rust 中的重中之重。

#### 定义特征

```rs
pub trait HasArea {
    fn area(&self) -> f64;
}
```
我们只定义特征方法的签名，而不进行实现，此时方法签名结尾是 ;，而不是 {}。

#### 为类型实现特征
语法：`impl 特征名 for 类型名 {}`
```rs
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}
```

然后调用：
```rs
fn main() {
    let c = Circle {
        x: 0.0,
        y: 0.0,
        radius: 1.0,
    };
    println!("Circle c has area: {}", c.area());
}
```

##### 孤儿规则
关于特征实现与定义的位置，有一条非常重要的原则：
* **如果你想要为类型 A 实现特征 T，那么 A 或者 T 至少有一个是在`当前作用域`中定义的**。
 
简单来说，孤儿规则要求当你为某类型实现某特征时，必须要求**类型**或者**特征**至少有一个是在当前包中定义的，你不能为第三方包的类型实现第三方的trait。

这个原则被称为**孤儿规则**，为了避免各种trait实现的冲突。

##### 默认实现
可以在特征中定义默认实现，这样其它类型无需再实现该方法，或者也可以选择重载该方法。
```rs
trait HasArea {
    // 默认实现
    fn area(&self) -> f64 {
        0.0
    }
}
```

测试：
```rs
// Circle结构体
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

// Square结构体
struct Square {
    x: f64,
    y: f64,
    side: f64,
}

trait HasArea {
    fn area(&self) -> f64 {
        0.0
    }
}

impl HasArea for Circle {// 使用重载方法
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

impl HasArea for Square{}// 使用默认实现

fn main() {
    let c = Circle {
        x: 0.0,
        y: 0.0,
        radius: 1.0,
    };
    println!("Circle c has area: {}", c.area());// 使用重载的方法

    let s = Square {
        x: 1.0,
        y: 1.0,
        side: 1.0,
    };
    println!("Square s has area: {}", s.area());// 使用默认方法
}
```

#### 使用特征作为函数参数
这是特征常用的地方。例如：
```rs
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```
`impl Summary`的意思是实现了Summary特征的item参数。

#### 特征约束（Trait Bound）
实际上`impl Trait`是一种语法糖。前面nofify函数的完整写法是：
```rs
fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```
形如 T: Summary 被称为**特征约束**。

在简单的场景下`impl Trait`这种语法糖就足够使用，但是对于**复杂的场景**，特征约束可以让我们拥有更大的灵活性。

例如一个函数接受两个`impl Summary`的参数：
1. 语法糖写法：
```rs
fn notify(item1: &impl Summary, item2: &impl Summary) {}
```
这样写会导致两个参数的类型可能不同。

2. 如果我们想要两个参数的类型相同，可以使用特征约束写法：
```rs
fn notify<T: Summary>(item1: &T, item2: &T) {}
```
泛型类型T说明了item1和item2必须拥有**同样的类型**，同时 T: Summary 说明了T必须**实现Summary特征**。

##### 多个特征约束
多个特征约束可以用 + 连接。

例如除了让参数实现 Summary 特征外，还可以让参数实现 Display 特征以控制它的格式化输出。

1. 语法糖写法：
```rs
fn notify(item: &(impl Summary + Display)) {}
```

2. 特征约束写法：
```rs
fn notify<T: Summary + Display>(item: &T) {}
```

##### where 约束
当特征约束变得很多时，可以使用 where 关键字来简化代码：

例如，下面的代码：
```rs
fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {}
```

可以简化为：
```rs
fn some_function<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{}
```

#### 返回实现了特征的类型
可以通过 impl Trait 来说明一个函数返回了一个类型，该类型实现了某个特征。
```rs
fn returns_summarizable() -> impl Summary {
    Weibo {
        username: String::from("sunface"),
        content: String::from(
            "m1 max太厉害了，电脑再也不会卡",
        )
    }
}
```
因为 Weibo 实现了 Summary，因此这里可以用它来作为返回值。但是对于 returns_summarizable 的调用者而言，他只知道返回了一个实现了 Summary 特征的对象，但是并不知道返回了一个 Weibo 类型。

这种返回值方式的作用是：当返回类型非常复杂，不知道如何声明时，可以使用 impl Trait 简单返回。（例如闭包和迭代器，它们的类型是非常复杂的，那么交给编译器解决吧）

这种返回值方式有一个很大的**限制**：只能有**一个**具体的类型。

#### 通过derive派生特征
被 derive 标记的对象会自动实现对应的默认特征代码，继承相应的功能。

例如`#[derive(Debug)]`可以让我们直接使用println!宏，以`println!("{:?}", s) `的形式打印结构体的实例。

#### 引入特征
在一些场景中，使用 as 关键字做类型转换会有比较大的限制，因为你想要在类型转换上拥有完全的控制，例如处理转换错误，那么你将需要引入`TryInto`特征。

这是因为**如果你要使用一个特征的方法，那么你需要将该特征引入当前的作用域中**。

### 2.8.3 特征对象（Trait Objects）
场景：现在在做一款游戏，需要将多个对象渲染在屏幕上，这些对象属于**不同的类型**，存储在列表中，渲染的时候，需要**循环该列表**并顺序渲染每个对象，在 Rust 中该怎么实现？

解法一：枚举。
* 缺点：当对象集合并不能事先明确地知道、或者要添加新的对象类型时，枚举就不再适用。

解法二：特征对象。

#### 特征对象的概念
概念：**特征对象（Trait Objects）**是一种使用特征进行**动态分发**的机制。它指向实现了某特征的类型的`实例`，这种映射关系是存储在一张表中，可以在运行时通过特征对象找到具体调用的类型方法。

#### 定义特征对象
特征对象通常通过使用`dyn`关键字和特征名称来创建，例如`dyn TraitName`。

通常会使用指针（如&或Box）来引用它，因为特征对象的大小在编译时是未知的。

例如：
```rust
trait Draw {
    fn draw(&self);
}

fn draw_it(x: &dyn Draw) {
    x.draw();
}
```
draw_it函数可以接受任何类型的参数，只要该参数类型实现了Draw特征。

##### 场景问题解决
编写这个UI库时，我们无法知道所有的UI对象类型，只知道的是：
* UI对象的类型不同
* 需要一个统一的类型来处理这些对象，无论是作为函数参数还是作为列表中的一员
* 需要对每一个对象调用`draw`方法

Step1：为UI组件定义一个特征：
```rust
pub trait Draw{
    fn draw(&self);
}
```

Step2：为`Button`和`SelectBox`实现特征：
```rust
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // 绘制按钮的代码
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // 绘制SelectBox的代码
    }
}
```

Step3：实现 Screen，有一个动态数组，里面元素的类型是 Draw 特征对象：
```rust
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}
```

为Screen实现一个`run`方法，遍历`components`并调用`draw`方法：
```rust
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
```

Step4：使用：
```rust
fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No")
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}
```
上面使用`Box::new(T)`的方式来创建了两个`Box<dyn Draw>`特征对象，
如果以后还需要增加一个UI组件，那么让该组件实现Draw特征，
则可以很轻松的将其渲染在屏幕上，甚至用户可以引入我们的库作为三方库，
然后在自己的库中为自己的类型实现Draw特征，然后进行渲染。

##### 鸭子类型（Duck Typing）
在动态类型语言中有一种叫做**鸭子类型**的概念。就是只关心值长啥样，而不关心它实际是什么。当一个东西走起来像鸭子，叫起来像鸭子，那么它就是一只鸭子，就算它实际上是一个**夜鹭**，也不重要，我们就当它是鸭子。

在上例中，Screen 在调用run的时候，我们并不需要知道各个组件的具体类型，也不需要检查组件是谁的实例，只要它实现了Draw特征，就能通过`Box::new`包装成`Box<dyn Draw>`特征对象，然后被渲染在屏幕上。

使用类似鸭子类型操作的优势是：**无需在运行时检查一个值是否实现了特定方法或者担心在调用时因为值没有实现方法而产生错误**。如果值没有实现特征对象所需的特征， 那么编译器会报错。

#### 特征对象的动态分发
* 静态分发（static dispatch）：编译器会为每一个泛型参数对应的具体类型生成一份代码。（是编译期行为）
* 动态分发（dynamic dispatch）：直到运行时，才能确定需要调用什么方法。（是运行时行为）

当使用特征对象时，Rust 必须使用动态分发。为此，Rust 在运行时使用特征对象中的指针来知晓需要调用哪个方法。

这张图解释了静态分发`Box<T>`和动态分发`Box<dyn Trait>`的区别：
![静态分发和动态分发的区别](https://pic1.zhimg.com/80/v2-b771fe4cfc6ebd63d9aff42840eb8e67_1440w.jpg)

#### 特征对象的限制
只有**对象安全**的特征才能拥有特征对象。当一个特征的所有方法都有如下属性时，它的对象才是安全的：
* 方法的返回类型不能是`Self`
* 方法不能有泛型参数

解释：如果特征方法返回了具体的Self类型，但是特征对象忘记了其真正的类型，那这个Self就没人知道它是谁了。但是对于泛型类型参数来说，当使用特征时其会放入具体的类型参数：此具体类型变成了实现该特征的类型的一部分。而当使用特征对象时其具体类型被抹去了，故而无从得知放入泛型参数类型到底是什么。

典型反例就是标准库中的Clone特征：
```rust
pub trait Clone {
    fn clone(&self) -> Self;// 返回类型是Self
}
```
如果违反了对象安全的规则，编译器会提示你。例如，如果尝试使用之前的 Screen 结构体来存放实现了 Clone 特征的类型：
```rust
pub struct Screen {
    pub components: Vec<Box<dyn Clone>>,
}
```
编译器会报错：
```
error[E0038]: the trait `std::clone::Clone` cannot be made into an object
 --> src/lib.rs:2:5
  |
2 |     pub components: Vec<Box<dyn Clone>>,
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::clone::Clone`
  cannot be made into an object
  |
  = note: the trait cannot require that `Self : Sized`
```


#### 总结
来自评论区：(https://course.rs/basic/trait/trait-object.html)

用户：NinjaSlayerSang

特征对象其实就是“多态”。多态在不同的语言里面有不同的名称、语法和实现方式，比如c语言就是用不安全的指针粗暴地实现，c++用虚函数来实现，java通过抽象类与接口来实现，像go，swift这种很新的语言都倾向于通过更灵活的**鸭式**辨型的抽象来实现**多态**和**动态派发**，go里面叫interface，swift里面叫protocol，而rust里面就叫trait。

### 2.8.4 深入了解特征

#### 关联类型（Associated Types）
关联类型与泛型类似：
* 关联类型是**特征**中的一种类型占位符，与特征的实现绑定。实现特征的类型在实现时指定该占位符的具体类型。
* 泛型在**函数或结构体定义**中使用，代表一类类型的占位符。泛型必须在使用时显式地指定具体类型。


例如标准库中的`Iterator`特征：
```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```
它有一个关联类型Item以及一个函数next，返回值是Option< Self::Item >。

假设我们有一个Counter结构体，现在为它实现Iterator特征：
```rust
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // --snip--
    }
}

fn main() {
    let c = Counter{..}
    c.next()
}
```
Self::Item用来指代该类型实现中定义的Item类型。

##### 相较于泛型的优势
使用关联类型可以显著减少泛型参数的数量。

例如，使用泛型：
```rust
trait Container<A,B>{
    fn contains(&self, a: A, b: B) -> bool;
}

fn differece<A,B,C>(container: &C) -> i32 {
    where C: Container<A,B> {
        //...
    }
}
```
由于使用了泛型，我们必须在函数头部添加泛型的声明。

而使用关联类型：
```rust
trait Container {
    type A;
    type B;

    fn contains(&self, a: Self::A, b: Self::B) -> bool;
}

fn difference<C: Container>(container: &C) -> i32 {
    //...
}
```
相比之下，关联类型更加简洁。

#### 默认泛型类型参数
当使用泛型类型参数时，可以为其指定一个默认的具体类型，例如标准库中的`std::ops::Add`特征：
```rust
trait Add<RHS=Self> {// 默认泛型参数
    type Output;

    fn add(self, rhs: RHS) -> Self::Output;
}
```
泛型参数`RHS`被赋予了默认值`Self`，这意味着当用户不指定`RHS`时，默认使用两个同样类型的值进行相加，然后返回一个关联类型`Output`。

使用：我们有一个Point结构体，为它实现Add特征：（为 Point 结构体提供 + 的能力，是运算符重载）
```rust
use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {// 使用了 RHS 的默认类型
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
               Point { x: 3, y: 3 });
}
```
*目前来说，只有定义在`std::ops`中的运算符才能进行重载*。

相反，当我们要求x和y的类型不同时，可以这样写：
```rust
use std::ops::Add;

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {// 指定了 RHS 的类型 Meters
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}
```

总结：默认泛型类型参数用于两个方面：
1. 减少实现的样板代码
2. 扩展类型但是无需大幅修改现有的代码

大多数情况下用不到这个内容。

#### 调用同名的方法
设有一个Human结构体，有两个特征：`Fly`和`Swim`，这三者都实现了`fly`的方法。
```rust
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}
```

##### 1、优先调用类方法
当调用 Human 实例的 fly 时法时，会优先调用 Human 类型的 fly 方法：
```rust
fn main() {
    let person = Human;
    person.fly();
}
```
则会输出`*waving arms furiously*`

##### 2、显式调用特征上的方法
为了能够调用两个特征的方法，需要使用显式调用的语法：
```rust
fn main() {
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
}
```
因为 fly 方法的参数是 self，当显式调用时，编译器就可以根据调用的类型(self 的类型)决定具体调用哪个方法。

但是关联函数（没有self）呢？

##### 3、同名关联函数调用
背景：设想一下，狗妈妈称呼自己的狗宝宝为Spot，其它动物称呼狗宝宝为puppy。当有动物不知道如何称呼狗宝宝时，它需要查询"puppy"字段。

```rust
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}
```
错误的调用方式：
* 错误1：`Dog::baby_name()`。会输出`Spot`，这不是我们想要的。


* 错误2：`Animal::baby_name()`。会报错，因为`baby_name`是关联函数，不是实例方法。编译器无法得到任何有效的信息：实现 Animal 特征的类型可能有很多，你究竟是想获取哪个动物宝宝的名称？狗宝宝？猪宝宝？还是熊宝宝？

正确的调用方式：使用**完全限定语法**。
```
<Type as Trait>::function(receiver_if_method, next_arg, ...);
```
其中第一个参数`receiver_if_method`是方法接收器，只有方法才拥有接收器，关联函数没有接收器。

例如：
```rust
fn main() {
    println!(<Dog as Animal>::baby_name());
}
```
通过`as`关键字，我们向Rust编译器提供了类型注解，也就是：`Animal`就是`Dog`，而不是其他动物。因此最终会调用`impl Animal for Dog`中的方法，获取到其它动物对狗宝宝的称呼：`puppy`。

总结：因此大多数时候，我们都无需使用完全限定语法。只有当存在多个同名函数或方法，且 Rust 无法区分出你想调用的目标函数时，才需要使用完全限定语法。

#### 特征定义中的特征约束
背景：有时，我们会需要让某个特征 A 能使用另一个特征 B 的功能(另一种形式的特征约束)，这种情况下，不仅仅要为类型实现特征 A，还要为类型实现特征 B 才行，这就是基特征( super trait )。

假设有一个自定义特征`OutlinePrint`，它的定义如下：
```rust
use std::fmt::Display;

trait OutlinePrint: Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}
```
可知它有一个方法可以对当前的实现类型进行格式化输出。但是它需要 所实现的类型 实现`Display`特征，才能进行格式化输出。

例如：
```rust
struct Point {
    x: i32,
    y: i32,
}

impl OutlinePrint for Point{}
```
编译器会报错。
```
error[E0277]: the trait bound `Point: std::fmt::Display` is not satisfied
  --> src/main.rs:20:6
   |
20 | impl OutlinePrint for Point {}
   |      ^^^^^^^^^^^^ `Point` cannot be formatted with the default formatter;
try using `:?` instead if you are using a format string
   |
   = help: the trait `std::fmt::Display` is not implemented for `Point`

```

解决方法：实现`Display`特征。
```rust
use std::fmt;

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
```
之后就可以正常使用了。

#### 在外部类型上实现外部特征（newtype模式）
前文有提到过孤儿规则：特征或者类型必需至少有一个是本地的，才能在此类型上定义特征。

但是有时候我们并不能修改这个类型，例如标准库中的`Vec<T>`，我们无法为它实现我们自己的特征。

因此，我们可以使用**newtype模式**绕过孤儿规则：
* 为一个**元组结构体**创建**新类型**，该元组结构体**封装**有一个**字段**，该字段就是希望实现特征的具体类型。该封装类型是**本地的**，因此我们可以为此类型实现外部的特征。

该功能的优点是：
* 为了实现特征，我们不需要修改原始类型
* 不会有运行时损耗（会被编译器自动忽略）

例如，我们想要为`Vec<T>`实现`Display`特征，直接实现是不行的：
```
error[E0117]: only traits defined in the current crate can be implemented for arbitrary types
--> src/main.rs:5:1
|
5 | impl<T> std::fmt::Display for Vec<T> {
| ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^------
| |                             |
| |                             Vec is not defined in the current crate
| impl doesn't use only types from inside the current crate
|
= note: define and implement a trait or new type instead
```
编译器提示我们`define and implement a trait or new type instead`，那么我们就使用newtype模式：

1、定义一个元组结构体：
```rust
struct Wrapper(Vec<String>);
```

2、为这个结构体实现`Display`特征：
```rust
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}
```
你会发现访问`Vec`的方法时，需要先从`Wrapper`中取出数组: `self.0`，然后才能执行`join`方法。

当然，Rust提供了`Deref`（解引用）特征，可以让我们在使用`Wrapper`时，就像使用`Vec`一样，而不需要每次都写`self.0`。
```rust
use std::ops::Deref;

impl Deref for Wrapper {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
```
`Deref`在进阶章节4.4.2中有详细介绍。

同时，如果不想`Wrapper`暴露底层数组的所有方法，我们还可以为`Wrapper`去重载这些方法，实现隐藏的目的。

### 2.8总结
1、特征定义中的特征约束，不能理解为“继承”。Rust没有继承的概念，取而代之的是“约束”。

例如`trait B: A`的含义是：实现B的结构体必须也实现A，即B的实现必须满足A的约束。类似集合论中$ B \subseteq A $。

理解成“B继承A”是错的，因为B并不继承A的方法，而是实现了A的方法。

2、newtype的出现就是为了绕过孤儿规则手动封装一层。有一句经典名言：没有什么是加一层解决不了的，如果不行那就再加一层。