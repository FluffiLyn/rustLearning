## 可变性
Rust 的变量在默认情况下是不可变的。如果变量 a 不可变，那么一旦为它绑定值，就不能再修改 a。

可以通过 mut 关键字让变量变为可变的。

## 下划线可忽略关于“未使用的变量”的警告
```
warning: unused variable: `y`
 --> src/main.rs:3:9
  |
3 |     let y = 10;
  |         ^ help: 如果 y 故意不被使用，请添加一个下划线前缀: `_y`
  |
  = note: `#[warn(unused_variables)]` on by default

```

## 变量解构
```rs
fn main() {
    let (a, mut b): (bool,bool) = (true, false);
    // a = true,不可变; b = false，可变
    println!("a = {:?}, b = {:?}", a, b);

    b = true;
    assert_eq!(a, b);
}
```

## 常量
常量不是不能更改的变量。
* 常量不允许使用 mut。常量不仅仅默认不可变，而且自始至终不可变，因为常量在编译完成后，已经确定它的值。
* 常量使用 const 关键字而不是 let 关键字来声明，并且值的类型必须标注。

## 变量遮蔽（shadowing）
Rust 允许声明相同的变量名，在后面声明的变量会遮蔽掉前面声明的。
```rs
fn main() {
    let x = 5;

    // 在main函数的作用域内对之前的x进行遮蔽
    let x = x + 1;

    {
        // 在当前的花括号作用域内，对之前的x进行遮蔽
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
}
```
输出：
```
The value of x in the inner scope is: 12
The value of x is: 6
```
变量遮蔽的用处在于，如果你在某个作用域内无需再使用之前的变量（在被遮蔽后，无法再访问到之前的同名变量），就可以重复的使用变量名字，而不用绞尽脑汁去想更多的名字。

但是如果没有let（没有声明），就不行。
