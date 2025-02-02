## 2.4 复合类型
### 2.4.1 字符串
#### 切片（slice）
切片允许你引用集合中部分连续的元素序列，而非整个集合。

创建切片的语法是`[开始索引..终止索引]`，其中`..`是rs的range语法。表示的范围是**左闭右开区间**。

在切片数据结构内部会保存开始的位置和切片的长度，其中长度是`终止索引 - 开始索引`。

对于字符串，切片就是对 String 类型中某一部分的引用。如：
```rs
let s = String::from("hello world");
let hello = &s[0..5];
let world = &s[6..11];
```

如果省略开始索引，将从0开始；如果省略终止索引，将一直到字符串的末尾；如果两者都省略，将得到整个字符串的切片。如：
```rs
let s = String::from("hello world");
let hello = &s[..5];
let world = &s[6..];
let whole = &s[..];
```
<color style="color: red">注意：切片的索引必须位于有效的UTF-8字符边界位置，否则程序会崩溃。</color>例如中文在UTF-8中占**3个字节**，如果切片的索引位于中文字符的中间，程序会崩溃：
```rs
let s = "中国人";
let a = &s[0..2];
println!("{}",a);
```

除了字符串，其他集合类型也可以使用切片，如数组：
```rs
let a = [1, 2, 3, 4, 5];
let slice = &a[1..3];
assert_eq!(slice, &[2, 3]);
```

#### 字符串字面量
字符串字面量是切片，它的类型是`&str`，是不可变引用，因此字符串字面量是不可变的。

#### 什么是字符串？
字符串是由字符组成的连续集合。**Rust 中的字符是 Unicode 类型**，因此每个字符占据 4 个字节内存空间，但是在字符串中不一样，**字符串是 UTF-8 编码**，也就是字符串中的字符所占的字节数是变化的(1 ~ 4)。

在标准库中，有多种字符串类型，最常用的是`String`和`&str`。

#### String与&str的转换
&str转换为String：
```rs
let s = String::from("hello");
let s1 = "hello world".to_string();
```

String转换为&str：取引用
```rs
fn main()
{
    let s = String::from("hello");
    say_hello(&s);
    say_hello(&s[..]);
    say_hello(s.as_str());
}

fn say_hello(s: &str)
{
    println!("{}", s);
}
```

#### 字符串索引
与其他语言不同，在Rust中，不可以使用`[]`操作符直接访问字符串的字符。

Rust字符串底层的存储格式是`[u8]`，一个UTF-8字符数组。每个字符的字节长度是不固定的，例如"Hola"是4个字节，"你好"是6个字节（大部分汉字是3个字节）。假设能够直接使用`[]`访问字符串的字符，那么就会返回无意义字节。

#### 操作字符串

##### 追加
`push(c: char)`追加一个`字符`，`push_str(s: &str)`追加一个`字符串`。

这两个方法都是在**原字符串**上进行操作，因此需要将字符串声明为`mut`。

##### 插入
`insert(idx: i32,c: char)`在指定位置插入一个字符，`insert_str(idx: i32,s: &str)`在指定位置插入一个字符串。

这个索引是**字节索引**，而不是字符索引，因此要注意前文提到的UTF-8字符的字节长度问题。

同样，这两个方法都是在**原字符串**上进行操作，因此需要将字符串声明为`mut`。

```rs
let mut s = String::from("hello");
s.insert(1,'a');
s.insert_str(2,"world");
println!("{}",s); // haworldello
```

##### 替换
1、 `replace(old: &str,new: &str)`

该方法会替换所有匹配到的字符串。适用于`String`和`&str`。会返回一个**新的字符串**，而不是操作原字符串。

```rs
let s_replace = String::from("Hello rust");
let s_new = s_replace.replace("rust","world");
dbg!(s_new); // "Hello world"
```

2、`replacen(old: &str,new: &str,count: usize)`

该方法会替换前`count`个匹配到的字符串。适用于`String`和`&str`。会返回一个**新的字符串**，而不是操作原字符串。

```rs
let s_replacen = String::from("Crab crab crab");
let s_new = s_replacen.replacen("crab","world",2);
dbg!(s_new); // "Crab world world"
```

3、`replace_range(range: Range<usize>,new: &str)`

该方法会替换指定范围的字符串。仅适用于`String`。会**直接操作原来的字符串**，而不是返回新的字符串。（故需要将变量声明为`mut`）

```rs
let mut s_replace_range = String::from("Rust crab");
s_replace_range.replace_range(0..4,"Hello");
dbg!(s_replace_range); // "Hello crab"
```

##### 删除
四个方法仅适用于`String`,它们都是**直接操作原来的字符串**。

1、`pop() -> Option<char>`

该方法删除并返回字符串的最后一个字符。这个返回值类型是`Option<char>`，因为字符串可能为空。

```rs
let mut s_pop = String::from("让我们说中文");
let p1 = s_pop.pop();
let p2 = s_pop.pop();
dbg!(p1); // Some('文')
dbg!(p2); // Some('中')
dbg!(s_pop); // "让我们说"
```

2、`remove(idx: usize) -> char`

该方法删除指定位置的字符。这个索引是**字节索引**，而不是字符索引。

```rs
let mut s_remove = String::from("让我们说中文");
println!(
    "s_remove 占 {} 个字节",
    std::mem::size_of_val(s_remove.as_str())
);
// 删除第一个字符
s_remove.remove(0);

// s_remove.remove(1)会报错
dbg!(s_remove);
```

3、`truncate(idx: usize) -> char`

删除字符串中从指定位置开始到结尾的全部字符，这个索引是**字节索引**，而不是字符索引。

```rs
let mut s_truncate = String::from("让我们说中文");
s_truncate.truncate(6);
dbg!(s_truncate); // "让我"
```

4、`clear()`

清空字符串。等于`truncate(0)`。

##### 连接
1、`+`或`+=`

要求右边的参数必须为Slice类型，即`&str`。返回一个**新的字符串**，所以变量声明可以不需要`mut`修饰。

```rs
let s_append = String::from("Hello");
let s_new = String::from(" world");
let s_result = s_append + &s_new;

let mut s_result = s_result + "!";// 变量遮蔽
s_result += "!!!";

dbg!(s_result); // "Hello world!!!!"
```

分析：
- `+`操作符的实现是String类的方法`fn add(self, s: &str) -> String`，因此`self`是`String`类型，`s`是`&str`类型。
- 这种写法使得+操作符可以链式调用，即String + &str return String。
- 在上例中，经过+操作后，s_append的所有权被转移，因此不能再使用。

2、`format!()`

`format!()`宏可以将多个字符串连接成一个字符串。适用于`String`和`&str`。返回一个**新的字符串**。

```rs
let s_format = String::from("Hello");
let s_new = String::from(" rust");
let s_result = format!("{}{}",s_format,s_new);
```

#### 字符串转义
可以用`\`加十六进制数转义输出一个ASCII或Unicode字符。

```rs
println!("Hello\x21");// Hello!
println!("Hello, \u{1F600}");// Hello, 😁
```

#### 字符串遍历
1、字符遍历
```rs
for c in "中国".chars()
{
    println!("{}",c);
}
```

输出：
```rs   
中
国
```

2、字节遍历
```rs
for b in "中国".bytes()
{
    println!("{}",b);
}
```

输出：
```rs
228
184
173
229
155
189
```

### 2.4.2 元组（tuple）
元组是由多种类型组合到一起形成的，因此它是复合类型，元组的长度是固定的，元组中元素的顺序也是固定的。

元组常用于函数返回值。

创建元组：
```rs
let tup : (i32,&str,u8) = (100,"hello",255);
```
注意：超过12个元素的元组无法被打印。


#### 用模式匹配解构元组
解构：用相同的模式匹配元组的元素，将元组的元素绑定到变量上。

通过解构可以为多个变量同时赋值：
```rs
let tup = (100,"hello",255);
let (a,b,c) = tup;
println!("a = {},b = {},c = {}",a,b,c);
```

#### 用.访问元组元素
索引从0开始：
```rs
let tup = (100,"hello",255);
let a = tup.0;
let b = tup.1;
let c = tup.2;
println!("a = {},b = {},c = {}",a,b,c);
```

### 2.4.3 结构体
一个结构体包含多个字段，每个字段有自己的类型。例如：

#### 结构体语法
1、定义结构体：
```rs
struct User
{
    active: bool,
    usrname: String,
    email: String,
}
```

2、创建结构体实例
```rs
let usr1 = User{
    active: true,
    usrname: String::from("usr1"),
    email: String::from("testtest@example.com")
}
```
注意：
1. 每个字段都要初始化
2. 初始化顺序无需一致
3. 可以将结构体实例整体声明为`mut`，但是不能单独声明某个字段为`mut`。

3、简化结构体创建

同名字段在初始化时可以省略冒号和字段名：
```rs
fn build_user(usrname: String,email: String) -> User
{
    User{
        active: true,
        usrname,
        email,
    }
}
```

4、`.`操作符访问和修改结构体字段
```rs
usr1.active = false;
```

5、更新结构体

有一种情况下，我们需要根据已有的结构体实例创建新实例。

Rust提供了结构体更新语法`..`。必须在最后一个字段后面使用`..`。
```rs
let usr2 = User{
    active: false,//显式初始化
    ..usr1
}
```
凡是没有显式初始化的字段，将会从`usr1`中复制。
#### 元组结构体（tuple struct）
字段没有名称的结构体称为元组结构体。当你希望有一个整体名称，但又不关心字段名时，可以使用元组结构体。
```rs
struct Point(i32,i32,i32);
let origin = Point(0,0,0);
```

#### 类单元结构体（unit-like struct）
没有任何字段和属性的结构体称为类单元结构体。当你定义一个类型，但不关心其具体内容，仅关心它的行为时，可以使用类单元结构体。
```rs
struct AlwaysEqual;//声明
let a = AlwaysEqual;
//实现某个特征（trait）
impl AlwaysEqual
{
    fn equal(&self) -> bool
    {
        true
    }
}
```

### 2.4.4 枚举
枚举是一个类型，它会包含所有可能的`枚举成员`，而`枚举值`是该类型中的具体`某个成员的实例`。

可以理解为枚举就是一个结构体集合。不同的结构体，它们的类型是不同的，而将它们内聚在一起，相当于赋予了一个namespace，取个新名字叫枚举。（Conclusion by zhenzhenChange）

#### 枚举值
假设我们有一个扑克花色的枚举类：
```rs
enum PokerSuit
{
    Clubs,
    Spades,
    Diamonds,
    Hearts,
}
```

可以创建该枚举类型的两个成员实例：
```rs
//通过::访问枚举成员
let c1 = PokerSuit::Hearts;
let c2 = PokerSuit::Diamonds;
```

接着可以将将数据与枚举成员关联，使得枚举值可以携带数据。
```rs
enum PokerSuit
{
    Clubs(u8),
    Spades(u8),
    Diamonds(u8),
    Hearts(u8),
}

fn main()
{
    let c1 = PokerSuit::Spades(5);
    let c2 = PokerSuit::Diamonds(13);
}
```

任何类型的数据都可以放入枚举成员中：
```rs
enum Msg
{
    Quit,//不携带数据
    Move{x: i32,y: i32},//结构体
    Write(String),//String
    ChangeColor(i32,i32,i32),//元组
}
```

#### Option枚举处理空值
当我们操作空值时，会导致程序崩溃。

Rust没有`null`，取而代之的是`Option`枚举。它包含在`Prelude`中，因此不需要手动引入。

其定义如下：Some(T)表示该枚举成员的数据类型为T，None表示数据为空。
```rs
enum Option<T>{
    Some(T),
    None,
}
```

Option枚举的**用法**是：当程序员不确定一个值是否为空时，可将值显式地放入对应类型的`Option<T>`中。接着，当使用该值时，必须要明确处理值为空的情况。

只要一个值不是`Option<T>`类型，就可以认定它的值不为空。这便是Rust的空值处理机制。


### 2.4.5 数组
Rust的数组是一个：
- 固定长度的
- 元素类型相同的
- 依次线性排列的
数据类型。

#### 创建数组
1、直接创建
```rs
let a = [1,2,3,4,5];//注意用方括号
```

2、指定元素类型和长度
```rs
let a: [i64; 5] = [1,2,3,4,5];
```
在指定数组的类型和长度时应按照`[type; size]`的格式来操作。

3、初始化相同元素
```rs
let a = [3; 5];//创建一个[3,3,3,3,3]数组
```
用[var; size]的格式来创建某个值重复size次的数组。

#### 访问数组元素
可以用索引访问数组元素。如a[2]。下标从0开始。

#### 越界访问
Rust不会允许越界访问数组。若尝试访问数组的越界元素，程序会panic。

#### 数组元素是非基础类型
以下代码会报错：
```rs
let array = [String::from("Rust is bad!"); 8];
```
因为复杂类型没有实现深拷贝，所以Rust不知道如何复制这个元素，只能一个一个创建。

不推荐写法：
```rs
let a = [String::from("AA"), String::from("AA")];// 又长又臭
```

正确写法是用`std::array::from_fn`:
```rs
let array: [String; 8] = std::array::from_fn(|_i| String::from("AA"));

println!("{:#?}", array);
```
调用`from_fn`函数，并传递一个闭包`|_i| String::from("AA")`。这个闭包接受一个参数 _i（表示当前元素的索引，但在这里没有使用），并返回字符串 "AA"。

输出宏中的:#？表示以多行格式`#`、调试格式`:?`打印变量。

#### 数组切片
数组切片允许我们引用数组的一部分：
```rs
let a: [i32; 5] = [1,2,3,4,5];
let slice = &a[1..3];
assert_eq!(slice, &[2,3]);
```
* 创建切片的开销很小，只是针对底层数组的一个引用。
* 切片类型 [T] 拥有不固定的大小，而切片引用类型 &[T] 则具有固定的大小，因为 Rust 很多时候都需要固定大小数据类型，因此 &[T] 更有用，&str 字符串切片也同理。