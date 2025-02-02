## 2.5 流程控制
### 2.5.1 if
1. if语句的基本结构
```rs
if condition == true{
    // do something
} else {
    // do something
}
```
注意到if后面的条件表达式不需要加括号，与c++区别开。

2. if语句块是表达式，可以赋值给变量，因此有这种用法：
```rs
let num = if (2 > 1) == true { 2 } else { 1 };
```

3. else if处理多重条件
与c++类似，会按照自上至下的顺序执行每一个分支判断，一旦成功，则跳出 if 语句块。如果要进行大量的匹配，不建议使用`else if`，而是使用`match`语句。

### 2.5.2 循环
#### for循环
##### 1.基本语法
Rust `for` 的写法类似JavaScript：
```rs
for 元素 in 集合{
    // do something
}
```
例如，遍历一个range：
```rs
for i in 1..=5{
    println!("{}", i);
}
```

##### 2. 所有权问题
默认是不可修改遍历的元素的。此外使用`for`时我们往往使用集合的**引用**形式，否则会发生所有权转移，后续无法再次使用该集合。

| 使用方法 | 所有权 |
| --- | --- |
| `for i in v` | 转移所有权 |
| `for i in &v` | 不可变借用 |
| `for i in &mut v` | 可变借用 |

##### 3. 在循环中获取索引
Rust没有提供直接的索引访问方式，但是可以使用`enumerate`方法：
```rs
fn main() {
    let a = [4, 3, 2, 1];
    // `.iter()` 方法把 `a` 数组变成一个迭代器
    for (i, v) in a.iter().enumerate() {
        println!("第{}个元素是{}", i + 1, v);
    }
}
```

此外，若不想单独声明一个变量，可以使用`for _ in 0..5`，`_`表示忽略该值或类型。

##### 4. 两种循环的对比
```rs
let collection = [1, 2, 3, 4, 5];
//第一种：循环索引，然后通过索引访问数组元素
for i in 0..collection.len() {
    println!("{}", collection[i]);
}
//第二种：直接遍历数组元素
for item in collection {
    println!("{}", i);
}
```
Rust语言圣经原文：
* 性能：第一种使用方式中 collection[index] 的索引访问，会因为**边界检查(Bounds Checking)**导致运行时的性能损耗 —— Rust 会检查并确认 index 是否落在集合内，但是第二种直接迭代的方式就**不会触发**这种检查，因为编译器会在编译时就完成分析并证明这种访问是合法的
* 安全：第一种方式里对 collection 的索引访问是**非连续的**，存在一定可能性*在两次访问之间，collection 发生了变化，导致脏数据产生*。而第二种直接迭代的方式是连续访问，因此不存在这种风险( 由于**所有权限制**，在访问过程中，数据并不会发生变化)。

因此，对于**遍历集合**，`for item in collection`是最安全也是最常用的，它不需要索引，也不会触发运行时边界检查，并且能保证元素不变化，因此性能更好、更安全。

#### while循环
适用于：“当条件为true时继续循环；当条件为false时停止循环”的情况。

```rs
let mut num = 0;
while num < 5 {
    println!("{}", num);
    num += 1;
}
```

#### loop循环
loop是简单的无限循环，一般要通过`break'和`continue`来控制循环的终止和跳过。

```rs
let mut cnt = 0;
let result = loop {
    cnt += 1;
    if cnt == 10 {
        break cnt * 2;
    }
};

println!("result is {}", result);
```


* break可以单独使用，也可以带返回值
* loop是一个表达式，因此可以返回值

#### break和continue
当有多层循环时，可以使用 continue 或 break 来控制外层的循环：外部的循环必须拥有一个标签 'label, 然后在 break 或 continue 时指定该标签。

例：
```rs
fn main() {
    let mut count = 0;
    'outer: loop {
        'inner1: loop {
            if count >= 20 {
                // This would break only the inner1 loop
                break 'inner1; // `break` is also ok 
            }
            count += 2;
        }

        count += 5;

        'inner2: loop {
            if count >= 30 {
                // This breaks the outer loop
                break 'outer;
            }

            // This will continue the outer loop
            continue 'outer;
        }
    }

    assert!(count == 30)
}
```
