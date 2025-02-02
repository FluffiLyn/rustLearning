## 2.7 方法（Method）
Rust 的方法往往跟结构体、枚举、特征(Trait)一起使用。
函数会在闭包中详细讨论，这里只讨论方法。

* 方法在 C++ 中是被称为**成员函数**。因此，在 C++ 中的“方法”和“函数”的区别，就是“成员函数”和“函数”的区别
* Java 只有方法，没有函数。
  
### 方法的定义
在 Rust 中，**方法**是定义在`impl`块中的函数，并且它们的**第一个参数是`self`**，表示方法是与某个实例相关联的。

Rust 使用 `impl` 来定义方法，例如以下代码：
```rs
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    // new是Circle的关联函数，因为它的第一个参数不是self。它用于初始化结构体的实例。new并不是关键字，仅是约定俗成的命名。
    fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle {
            x: x,
            y: y,
            radius: radius,
        }
    }

    // Circle的方法，&self表示借用当前的Circle结构体
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

```
`impl Circle` 表示为 `Circle` 结构体实现方法。`fn new` 是一个**关联函数**，因为它的第一个参数不是 `self`。`fn area` 是一个**方法**，因为它的第一个参数是 `self`。

其它语言中所有定义都在 class 中，但是 Rust 的对象定义和方法定义是分离的，这种数据和使用分离的方式，会给予使用者极高的灵活度。

![方法的区别](https://pica.zhimg.com/80/v2-0d848e960f3279999eab4b1317f6538e_1440w.png)

#### self、&self、&mut self （重点）
`&self`是`self: &Self`的简写（注意大小写）。在一个impl块内：
* `Self`指代被实现方法的**结构体类型**
* `self`指代此类型的**实例**。
`self`的使用也要严格遵守所有权规则。我们并不想获取所有权，也无需去改变它，只是希望能够读取结构体中的数据。如果想要在方法中去改变当前的结构体，需要将第一个参数改为`&mut self`。

使用方法代替函数有以下好处：
* 不用在函数签名中重复书写`self`对应的类型
* 代码的组织性和内聚性更强，对于代码维护和阅读更友好

#### 方法名跟结构体字段名相同
Rust是允许此行为的，往往适用于实现`getter`访问器。
```rs
//impl Rectangle块内
pub fn height(&self) -> u32 {
    return self.height;
}
```

相对于c++，Rust 并没有一个与 -> 等效的运算符；相反，Rust 有一个叫 自动引用和解引用的功能。方法调用是 Rust 中少数几个拥有这种行为的地方。当使用 object.something() 调用方法时，Rust 会自动为 object 添加 &、&mut 或 * 以便使 object 与方法签名匹配。
```rs
p1.distance(&p2);
(&p1).distance(&p2);
```
两者是等价的，但是第一种写法更加简洁。

### 关联函数
定义在`impl`中且没有`self`的函数被称之为关联函数。

因为它没有`self`，不能用`f.read()`的形式调用，因此它是一个函数而不是方法，它又在`impl`中，与结构体紧密关联，因此称为关联函数。

我们需要用`::`来调用，例如`let sq = Rectangle::new(3, 3);`。这个方法位于结构体的命名空间中：`::`语法用于关联函数和模块创建的命名空间。

### 多个impl块
是允许的，提供更多的灵活性和代码组织性。

### 为枚举实现方法
我们可以像结构体一样，为枚举实现方法：
```rs
#![allow(unused)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // 在这里定义方法体
    }
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();
}
```