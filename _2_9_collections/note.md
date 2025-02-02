## 2.9 集合类型
### 2.9.1 动态数组Vector
动态数组类型用`Vec<T>`表示。包含在`prelude`中，因此不需要额外引入。

动态数组只能存储相同类型的元素。如果需要存储不同类型的元素，可以使用枚举类型或特征对象。
#### 创建动态数组
##### 1、`Vec::new()`方式
这是最符合Rust风格的创建方式，调用了`new()`关联函数。

创建空数组时，必须要指定泛型类型：
```rust
let v: Vec<i32> = Vec::new();
```

但是，如果在创建后就插入元素，就可以省略：
```rust
let mut v = Vec::new();
v.push(1);
```

此外，如果预先知道要存储的元素数量，可以使用`Vec::with_capacity(capacity)`方法，这样可以减少态扩容的开销：
```rust
let mut v = Vec::with_capacity(10);
```

##### 2、`vec![]`宏方式
`vec![]`宏可以用来创建包含**初始值**的动态数组：
```rust
let v = vec![1, 2, 3];
```
注意vec是小写，且不用标注类型。

#### 增删查
##### 1、增
将数组声明为可变的，然后调用`push()`方，即可在数组末尾插入元素：
```rust
let mut v = Vec::new();
v.push(1);
```

##### 2、删
在超出作用域时，Rust会自动调用`drop()`方法，删除数组。

##### 3、查
1、可以通过下标索引或者`get()`方法来访问数组元素。
```rust
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
println!("第三个元素是 {}", third);

match v.get(2) {
    Some(third) => println!("第三个元素是 {third}"),
    None => println!("没有第三个元素！"),
}
```
`&v[2]`表示借用v中的第三个元素，最终会获得该元素的引用。

而`v.get(2)`也是访问第三个元素，但是有所不同的是，它返回了`Option<&T>`，因此还需要额外的match来匹配解构出具体的值。正因此，get调用的安全性更高，因为在数组越界时，它会返回None，而不是直接报错。

当你确定索引不会越界时，就用索引访问，否则使用get方法访问。天知道那些用户会输入什么。

2、如果要依次访问数组的所有元素，可以使用迭代器，也比下标索引更安全：
```rust
let v = vec![100, 32, 57];
for i in &v {
    println!("{}", i);
}
```

也可以在迭代过程中修改数组元素：
```rust
let mut v = vec![100, 32, 57];
for i in &mut v {// 可变借用
    *i += 50;
}
```

#### 同时借用多个元素
```rust
let mut v = vec![1, 2, 3, 4, 5];

let first = &v[0];

v.push(6);

println!("The first element is: {first}");
```
首先`first = &v[0]`进行了不可变借用，`v.push`进行了可变借用，如果 `first`在`v.push`之后不再使用，那么该段代码可以成功编译。

然而这段代码中，`first`这个不可变借用在可变借用`v.push`后被使用了，那么编译器就会报错：
```
$ cargo run
Compiling collections v0.1.0 (file:///projects/collections)
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable 无法对v进行可变借用，因此之前已经进行了不可变借用
--> src/main.rs:6:5
|
4 |     let first = &v[0];
|                  - immutable borrow occurs here // 不可变借用发生在此处
5 |
6 |     v.push(6);
|     ^^^^^^^^^ mutable borrow occurs here // 可变借用发生在此处
7 |
8 |     println!("The first element is: {}", first);
|                                          ----- immutable borrow later used here // 不可变借用在这里被使用

For more information about this error, try `rustc --explain E0502`.
error: could not compile `collections` due to previous error
```
这样报错的原因是：**数组的大小是可变的，当旧数组的大小不够用时，Rust 会重新分配一块更大的内存空间，然后把旧数组拷贝过来。这种情况下，之前的引用显然会指向一块无效的内存。**

这非常符合Rust风格 —— 对用户进行严格的教育。

我们在c++复现一下：
```cpp
#include <iostream>
#include <vector>
using namespace std;

int main()
{
    vector<int>* v = new vector<int>({ 1 });
    v->push_back(1);
    int& ptr = v->at(0); // 定义为引用类型
    cout << ptr << endl; // 输出引用的值
    v->push_back(1); // 可能导致重新分配内存
    cout << ptr << endl; // 访问无效的引用
    system("pause");
    return 0;
}
```
输出：
```
1
-572662307 // 无效的引用
```

#### 存储不同类型的元素
1、使用枚举类型：
```rust
#[derive(Debug)]
enum Cell {
    Int(i32),
    Float(f64),
}

fn main() {
    let v = vec![
        Cell::Int(3),
        Cell::Float(10.12),
    ];

    for i in &row {
        println!("{:?}", i);
    }
}
```
数组存储了不同类型的元素，但都属于`Cell`枚举类型。

2、使用特征对象：
```rust
trait Cell{
    fn display(&self);
}

struct Int(i32);
impl Cell for Int{
    fn display(&self){
        println!("{}", self.0);
    }
}

struct Float(f64);
impl Cell for Float{
    fn display(&self){
        println!("{}", self.0);
    }
}

fn main() {
    let v: Vec<Box<dyn Cell>> = vec![
        Box::new(Int(3)),
        Box::new(Float(10.12)),
    ];

    for i in &v {
        i.display();
    }
}
```
创建了`Cell`特征对象，然后定义了`Int`和`Float`元组结构体，并为它们实现了`Cell`特征对象。最后，创建了一个`Vec<Box<dyn Cell>>`类型的数组，用来存储不同类型的元素。

使用枚举时，增加类型得改枚举代码；使用特征对象时，仅需增加实现代码。

#### 其他常用方法
```rust
let mut v =  vec![1, 2];
assert!(!v.is_empty());         // 检查 v 是否为空

v.insert(2, 3);                 // 在指定索引插入数据，索引值不能大于 v 的长度， v: [1, 2, 3] 
assert_eq!(v.remove(1), 2);     // 移除指定位置的元素并返回, v: [1, 3]
assert_eq!(v.pop(), Some(3));   // 删除并返回 v 尾部的元素，v: [1]
assert_eq!(v.pop(), Some(1));   // v: []
assert_eq!(v.pop(), None);      // 记得 pop 方法返回的是 Option 枚举值
v.clear();                      // 清空 v, v: []

let mut v1 = [11, 22].to_vec(); // append 操作会导致 v1 清空数据，增加可变声明
v.append(&mut v1);              // 将 v1 中的所有元素附加到 v 中, v1: []
v.truncate(1);                  // 截断到指定长度，多余的元素被删除, v: [11]
v.retain(|x| *x > 10);          // 保留满足条件的元素，即删除不满足条件的元素

let mut v = vec![11, 22, 33, 44, 55];
// 删除指定范围的元素，同时获取被删除元素的迭代器, v: [11, 55], m: [22, 33, 44]
let mut m: Vec<_> = v.drain(1..=3).collect();    

let v2 = m.split_off(1);        // 指定索引处切分成两个 vec, m: [22], v2: [33, 44]
```

还有类似数组切片的方式获取部分元素：
```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let s = &v[1..3]; // 获取索引1到3的元素，但不包括3
    let s1 = &v[1..=3]; // 获取索引1到3的元素，包括3
    println!("{:?}", s);
    println!("{:?}", s1);
}
```

#### 排序
在 rust 里，实现了两种排序算法，分别为稳定的排序`sort`和`sort_by`，以及非稳定排序`sort_unstable`和`sort_unstable_by`。

非稳定排序的算法的速度会**优于**稳定排序算法，同时，稳定排序还会**额外分配**原数组一半的空间。

对于整数数组，可直接使用`sort`方法：
```rust
fn main() {
    let mut v = vec![1, 5, 10, 2, 15];
    v.sort();
    println!("{:?}", v);
}
```

对于浮点数数组：
```rust
fn main() {
    let mut vec = vec![1.0, 5.6, 10.3, 2.0, 15f32];    
    vec.sort_unstable();    
    assert_eq!(vec, vec![1.0, 2.0, 5.6, 10.3, 15f32]);
}
```
报错：
```
error[E0277]: the trait bound `f32: Ord` is not satisfied
    --> src/main.rs:29:13
     |
29   |         vec.sort_unstable();
     |             ^^^^^^^^^^^^^ the trait `Ord` is not implemented for `f32`
     |
     = help: the following other types implement trait `Ord`:
               i128
               i16
               i32
               i64
               i8
               isize
               u128
               u16
             and 4 others
note: required by a bound in `core::slice::<impl [T]>::sort_unstable`
    --> /home/keijack/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:2635:12
     |
2635 |         T: Ord,
     |            ^^^ required by this bound in `core::slice::<impl [T]>::sort_unstable`

For more information about this error, try `rustc --explain E0277`.
```
这是因为在浮点数当中，存在一个`NAN`的值，这个值无法与其他的浮点数进行对比，因此，浮点数类型并没有实现**全数值**可比较（即`Ord`）的特性，而是实现了部分可比较的特性`PartialOrd`。

因此，对于浮点数数组，如果你确定数组中没有NAN值，需要使用这种方式：
```rust
fn main() {
    let mut vec = vec![1.0, 5.6, 10.3, 2.0, 15f32];    
    vec.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());    
    assert_eq!(vec, vec![1.0, 2.0, 5.6, 10.3, 15f32]);
}
```
其中`|a, b| a.partial_cmp(b).unwrap()`是一个闭包，含义如下：
* `|a, b|`表示闭包的参数，这里是两个浮点数；
* `a.partial_cmp(b)`函数返回一个`Option<Ordering>`，表示a和b的比较结果
  * 返回值可能是`Some(Ordering::Less)`、`Some(Ordering::Equal)`、`Some(Ordering::Greater)`或`None`；
* `unwrap()`方法用于获取`Option`的值，如果是`None`，则会直接panic。

同理，对于结构体数组，也要指定排序的字段：
```rust
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: String, age: u32) -> Person {
        Person { name, age }
    }
}

fn main() {
    let mut people = vec![
        Person::new("Zoe".to_string(), 25),
        Person::new("Al".to_string(), 60),
        Person::new("John".to_string(), 1),
    ];
    // 定义一个按照年龄倒序排序的对比函数
    people.sort_unstable_by(|a, b| b.age.cmp(&a.age));

    println!("{:?}", people);
}
```
这样就可以按照年龄倒序排序了。

事实上，实现`Ord`需要我们实现`Ord、Eq、PartialEq、PartialOrd`这些属性，但是你可以通过`derive`宏来自动实现这些属性：
```rust
#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
struct Person {
    name: String,
    age: u32,
}
...
```
此时会依据属性的顺序依次比较：当name的值相同时，就会比较年龄。

#### 练习笔记
`v.extend([x,y,z])`可以将数组[x,y,z]的元素依次插入到v中。

### 2.9.2 哈希映射HashMap
其他语言中也称为dictionary（c#）、map等。

使用时需要`use std::collections::HashMap`。

#### 创建Hashmap

##### 1、`HashMap::new()`方式
```rust
use std::collections::HashMap;
let mut gems = HashMap::new();

// 将宝石类型和对应的数量写入表中
gems.insert("diamond", 1);
gems.insert("emerald", 2);
gems.insert("shit", 100);
```

如果预先知道要存储的键值对个数，可以使用`HashMap::with_capacity(capacity)`创建指定大小的哈希映射，避免频繁的内存分配和拷贝，提升性能。

##### 2、使用迭代器和collect方法
实际应用场景是，从另一个数据结构中获取对应的数据，然后生成HashMap，此时不能依次插入。

如何将`Vec<String, u32>`中的数据快速写入`HashMap<String, u32>`中？
```rust
fn main() {
    use std::collections::HashMap;

    let teams_list = vec![
        ("中国队".to_string(), 100),
        ("美国队".to_string(), 10),
        ("日本队".to_string(), 50),
    ];

    let teams_map: HashMap<_,_> = teams_list.into_iter().collect();
    
    println!("{:?}",teams_map)
}
```
`let teams_map: HashMap<_,_> = teams_list.into_iter().collect();`的解释如下：
* `into_iter()`方法将`teams_list`转化为迭代器。
* `collect()`方法收集迭代器中的元素。
* `HashMap<_,_>`是类型标注，用于配合`collect()`方法，告诉编译器将迭代器中的元素收集为 HashMap 集合类型，具体类型让编译器自己推导。

要是你忘了标注，编译器会提示你：
```
error[E0282]: type annotations needed // 需要类型标注
  --> src/main.rs:10:9
   |
10 |     let teams_map = teams_list.into_iter().collect();
   |         ^^^^^^^^^ consider giving `teams_map` a type // 给予 `teams_map` 一个具体的类型
```

注意：通过该方式创建的HashMap，键值对的顺序会被打乱。
```rust
fn main(){
    use std::collections::HashMap;
    let v = vec!{
        ("A".to_string(),1),
        ("B".to_string(),2),
        ("C".to_string(),3),
        ("D".to_string(),4),
        ("E".to_string(),5),
    };

    let map = v.into_iter().collect::<HashMap<_,_>>();
    println!("{:?}",map);
}
```
输出：
```
{"D": 4, "B": 2, "C": 3, "A": 1, "E": 5}
// 每次运行的输出顺序可能不同
```

#### 所有权转移
HashMap的所有权规则与其他类型没有区别：
* 若类型实现 Copy 特征，该类型会被复制进 HashMap，因此无所谓所有权
* 若没实现 Copy 特征，所有权将被转移给 HashMap 中

如果要将**引用类型**放入 HashMap 中，确保该引用的**生命周期**至少跟 HashMap 活得一样久！

#### 增删查改

##### 增
`insert(key, value)`

##### 删
`remove(key)`

##### 查
```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

let team_name = String::from("Blue");
let score: Option<&i32> = scores.get(&team_name);
```
`get(key)`，返回`Option<&V>`。注意是不可变引用。

如果要返回值本身，可以：
```rust
let s: i32 = scores.get(&team_name).copied().unwrap_or(0);
```
Option的`copied()`方法用于将引用泛型参数`Option<&i32>`转化为具体的值`Option<i32>`，然后使用`unwrap_or()`方法获取Some值，如果是`None`，则返回默认值0。

相较于`unwrap()`，`unwrap_or()`方法可以避免出现None值，进而导致panic。

##### 改
示例：
```rust
fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert("Blue", 10);

    // 覆盖已有的值
    let old = scores.insert("Blue", 20);
    assert_eq!(old, Some(10));

    // 查询新插入的值
    let new = scores.get("Blue");
    assert_eq!(new, Some(&20));

    // 查询Yellow对应的值，若不存在则插入新值
    let v = scores.entry("Yellow").or_insert(5);
    assert_eq!(*v, 5); // 不存在，插入5

    // 查询Yellow对应的值，若不存在则插入新值
    let v = scores.entry("Yellow").or_insert(50);
    assert_eq!(*v, 5); // 已经存在，因此50没有插入
}
```
`insert`方法会返回旧值，若没有旧值，则返回None。

`entry`方法在 map 中获取给定键的对应值的可变引用。会返回一个`Entry`枚举类型，这个枚举有两个值:
```rust
pub enum Entry<'a, K: 'a, V: 'a> {
    Occupied(OccupiedEntry<'a, K, V>),
    Vacant(VacantEntry<'a, K, V>),
}
```
* `Occupied`表示已经存在的值；
* `Vacant`表示不存在的值。

`or_insert`方法会返回一个可变引用，如果查询值不存在，则插入新值。

#### 哈希函数
一个类型是否可以作为 HashMap 的键，取决于该类型是否实现了`Eq`特征，即是否可以进行相等性比较。

例如`f32`和`f64`类型，没有实现`Eq`特征，因此不能作为 HashMap 的键。

标准库的`HashMap`使用了性能略低但较为安全的哈希函数`SipHash`。`SipHash`在中等大小的键上，性能相当不错，但是对于小型的键（例如整数）或者大型键（例如字符串）来说，性能还是不够。此外，它不是有序的。

如果你要：
* 有序的哈希映射，可以使用`BTreeMap`，它是基于红黑树实现的。
* 更快但不安全的哈希函数，可以使用`ahash`库。
