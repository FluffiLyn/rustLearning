## 2.6 模式匹配
模式是模式是 Rust 中特殊的语法，它用来匹配类型中的结构，通常由以下内容组合：
* 字面量
* 解构的数组、枚举、结构体或者元组
* 变量
* 通配符
* 占位符

模式匹配是函数式编程的重要概念，用于为复杂的类型系统提供一个轻松的解构能力。比`switch`更强大，更灵活。

### 2.6.1 match表达式
#### 基本用法
```rs
match target {
    模式1 => 表达式1,
    模式2 => {
        语句1;
        语句2;
        表达式2
    },
    _ => 表达式3
}
```
* 结构：
  * 每个分支相关联的代码是一个表达式，而表达式的结果值将作为整个 match 表达式的返回值。如果分支有多行代码，那么需要用 {} 包裹，同时最后一行代码需要是一个表达式。
  
  * match 后紧跟着的是一个表达式，跟 if 很像，但是 if 后的表达式必须是一个布尔值，而 match 后的表达式返回值可以是任意类型，只要能跟后面的分支中的模式匹配起来即可。

* 作用：将模式与 target 进行匹配，如果匹配成功，则执行对应的表达式或语句。如果没有匹配成功，则执行下一个模式，直到遇到`_`，表示默认情况。

示例：
```rs
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny =>  {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

#### 匹配多个模式
```rs
match target {
    模式1 | 模式2 | 模式3 => 表达式1,
    _ => 表达式2
}
```
注意或是单个竖线`|`，而不是两个竖线`||`。

#### 使用match表达式赋值
`match`本身也是表达式。
```rs
enum IPAddress{
    Ipv4,
    Ipv6
}

fn main() {
    let ip = IPAddress::Ipv4;
    let ip_str = match ip {
        IPAddress::Ipv4 => "127.0.0.1",
        IPAddress::Ipv6 => "::1",
        _ => "unknown"
    };
    println!("ip: {}", ip_str);
}
```

#### 取出绑定值
```rs
enum Action {
    Say(String),
    MoveTo(i32, i32),
    ChangeColorRGB(u16, u16, u16),
}

fn main() {
    let actions = [
        Action::Say("Hello Rust".to_string()),
        Action::MoveTo(1,2),
        Action::ChangeColorRGB(255,255,0),
    ];
    for action in actions {
        match action {
            Action::Say(s) => {
                println!("{}", s);
            },
            Action::MoveTo(x, y) => {
                println!("point from (0, 0) move to ({}, {})", x, y);
            },
            Action::ChangeColorRGB(r, g, _) => {
                println!("change color into '(r:{}, g:{}, b:0)', 'b' has been ignored",
                    r, g,
                );
            }
        }
    }
}
```
在匹配`Action`模式时，我们把它内部存储的值绑定到了括号内的变量上。

#### 穷尽匹配
match必须穷尽所有可能的情况，否则会报错。这种机制可以避免null陷阱。

#### _通配符
等效于`default`，表示默认情况。

#### matches!宏
`matches!`宏可以将一个表达式与模式进行匹配，返回一个布尔值。
```rs
let word = 'd';
assert!(matches!(word, 'a'..='z' | 'A'..='Z'));
```

#### if let表达式
若只有一个模式的值需要被处理，其它值直接忽略，则用if let表达式。
```rs
if let Some(3) = v {
    println!("three");
}
```

#### 变量遮蔽
无论是`match`表达式还是`if let`表达式，如果你使用**同名变量**，会发生变量遮蔽：
```rs
fn main() {
   let age = Some(30);
   println!("在匹配前，age是{:?}",age);
   // match的age是Option<i32>类型
   match age {
    // 以下age是i32类型
       Some(age) =>  println!("匹配出来的age是{}",age),
       _ => ()
   }
   println!("在匹配后，age是{:?}",age);
}
```
该遮蔽持续到match块结束。

不建议使用同名变量，match中的变量遮蔽不容易看出来，容易引起混淆。