## 2.10 初步了解生命周期
大多数情况下，我们无需手动的声明生命周期，因为编译器可以自动进行推导。
* 编译器大多数时候也可以自动推导生命周期
* 在多种类型存在时，编译器往往要求我们手动标明类型：当多个生命周期存在，且编译器无法推导出某个引用的生命周期时，就需要我们手动标明生命周期

### 生命周期的主要作用
生命周期的主要作用是避免**悬垂引用**，即避免引用一个已经被释放的对象。

示例：
```rust
{
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r);
}
```
* `let r`的声明方式貌似存在悬垂引用的风险，但实际上当我们不初始化它就使用时，编译器会报错。
* r 引用了内部花括号中的 x 变量，但是 x 会在内部花括号 `}` 处被释放，因此回到外部花括号后，r 会引用一个无效的 x。

报错如下：
```
error[E0597]: `x` does not live long enough
  --> main.rs:7:17
   |
6  |             let x = 5;
   |                 - binding `x` declared here
7  |             r = &x;
   |                 ^^ borrowed value does not live long enough
8  |         }
   |         - `x` dropped here while still borrowed
9  |     
10 |         println!("r: {}", r);
   |                           - borrow later used here
```

### 借用检查（Borrow Checker）
Rust 使用了一个借用检查器(Borrow checker)，来检查我们程序的借用正确性。

对于上述代码，修改后，x 变量只要比 r 活得久，那么 r 就能随意引用 x 且不会存在危险。

### 函数中的生命周期
例子：longest函数返回两个字符串切片中较长者：
```rust
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
longest函数的参数和返回值都是字符串切片。看起来很不错吧？但是编译器会报错：
```
error[E0106]: missing lifetime specifier
 --> src/main.rs:9:33
  |
9 | fn longest(x: &str, y: &str) -> &str {
  |               ----     ----     ^ expected named lifetime parameter // 参数需要一个生命周期
  |
  = help: this function's return type contains a borrowed value, but the signature does not say whether it is
  borrowed from `x` or `y`
  = 帮助： 该函数的返回值是一个引用类型，但是函数签名无法说明，该引用是借用自 `x` 还是 `y`
help: consider introducing a named lifetime parameter // 考虑引入一个生命周期
  |
9 | fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  |           ^^^^    ^^^^^^^     ^^^^^^^     ^^^
```
原因在于存在多个引用时，编译器有时会无法自动推导生命周期，此时就需要我们手动去标注，通过为参数标注合适的生命周期来帮助编译器进行借用检查的分析。

### 生命周期的标注
记住，**标记生命周期只是告诉编译器，让编译器不要难为我们；实际上并不会改变任何引用的实际作用域。**

例如一个变量，只能活一个花括号，那么就算你给它标注一个活全局的生命周期，它还是会在前面的花括号结束处被释放掉，并不会真的全局存活。

标注语法以`'`开头，名称往往是一个单独的小写字母，大多数人都用`'a`来作为生命周期的名称。

如果是引用类型的参数，那么生命周期会位于引用符号 & 之后，并用一个空格来将生命周期和引用参数分隔开。

```rust
&i32        // 引用
&'a i32     // 带有显式生命周期的引用
&'a mut i32 // 带有显式生命周期的可变引用
```

例如，以下生命周期标注仅仅说明，这两个参数x和y至少活得和'a 一样久，至于到底活多久或者哪个活得更久，无法得知。
```rust
fn useless<'a>(x: &'a i32, y: &'a i32) {}
```

#### 函数签名中的生命周期标注
和泛型一样，使用生命周期参数，需要先声明`<'a>`。
```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
x、y和返回值**至少**活得和'a一样久（因为返回值要么是x，要么是y），但是实际上这两个参数的真实生命周期可能是不一样的。

#### 深入思考生命周期标注
函数的返回值如果是一个引用类型，那么它的生命周期只会来源于：
* 函数参数的生命周期
* **函数体中**某个新建引用的生命周期

后者则是典型的悬垂引用场景，如：
```rust
fn longest<'a>(x: &str, y: &str) -> &'a str {// 返回值与参数生命周期无关
    let result = String::from("really long string");
    result.as_str()
}
```
函数返回值引用了函数体内创建的字符串，显然这会导致悬垂引用，编译器会报错。

最好的办法就是返回内部字符串的**所有权**，然后把字符串的所有权转移给调用者。

修改后：
```rust
fn longest<'a>(_x: &str, _y: &str) -> String {
    String::from("really long string")
}

fn main() {
   let s = longest("don't", "cares");
}
```

总结：生命周期语法用来将函数的多个**引用参数**和**返回值**的*作用域*关联到一起，一旦关联到一起后，Rust 就拥有充分的信息来确保我们的操作是**内存安全**的。

### 结构体中的生命周期
为什么之前的章节不在结构体中使用字符串字面量或切片，而是统一使用 String 类型？因为前者是引用，它们不能为所欲为；后者在结构体初始化时，只要转移所有权即可。

如果要在结构体中使用**引用**，就需要**标注**生命周期，例如：
```rust
struct ImportantExcerpt<'a> {//类似泛型声明
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
```
该生命周期标注说明，结构体`ImportantExcerpt`所引用的字符串`str`生命周期需要大于等于该结构体的生命周期。

上述代码中`ImportantExcerpt`的生命周期从第4行开始，到main函数末尾结束，而该结构体引用的字符串从第一行开始，也是到 main 函数末尾结束，因此是符合要求的。

反之，如果这样写main函数：
```rust
fn main() {
    let i;
    {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        i = ImportantExcerpt {
            part: first_sentence,
        };
    }
    println!("{:?}",i);
}
```
那么结构体i的生命周期就会比字符串的生命周期长，编译器会报错。

### 生命周期消除
编译器为了简化用户的使用，运用了生命周期消除的技术——编译器可以自动推导出生命周期，无需手动标注。

例如：
```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```
对于 first_word 函数，它的返回值是一个引用类型，那么该引用只有两种情况：
* 从参数获取；
* 从函数体内部新创建的变量获取。

很明显，返回值是从参数 s 中获取的，因此返回值的生命周期就是参数 s 的生命周期，编译器可以自动推导。

#### 三条消除规则
有两点要注意：
* 该规则并非万能，若编译器不能确定某件事是正确时，会直接判为不正确，那么你还是需要手动标注生命周期。
* 函数或者方法中，参数的生命周期被称为`输入生命周期`，返回值的生命周期被称为`输出生命周期`。

编译器使用三条消除规则来确定哪些场景不需要显式地去标注生命周期。其中第一条规则应用在输入生命周期上，第二、三条应用在输出生命周期上。若编译器发现三条规则都不适用时，就会报错，提示你需要手动标注生命周期。

规则一：**每一个引用参数都会获得独自的生命周期**。
* `fn foo<'a, 'b>(x: &'a i32, y: &'b i32)`，x 和 y 有不同的生命周期 'a 和 'b 。

规则二：**若只有一个输入生命周期（函数参数中只有一个引用类型），那么该生命周期会被赋给所有的输出生命周期**，也就是所有返回值的生命周期都等于该输入生命周期
* `fn foo<'a>(x: &'a i32) -> &'a i32`，返回值的生命周期等于输入生命周期 'a。

规则三：**若有多个输入生命周期，且其中一个是`&self`或`&mut self`，则`&self`的生命周期被赋给所有的输出生命周期**
* 拥有 &self 形式的参数，说明该函数是一个**方法**，该规则让方法更方便使用。

### 方法中的生命周期
具有生命周期的结构体实现方法时，语法跟泛型参数语法相似。

注意：
* `impl`中必须使用结构体的完整名称，包括`<'a>`，因为生命周期标注也是结构体类型的一部分。
* 方法签名中，往往不需要标注生命周期，得益于生命周期消除的第一和第三规则。
```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}
```

如果要令生命周期`'b`比`'a`小，可以用约束语法（类似泛型、特征约束）：`impl<'a: 'b, 'b> xxx`

这表示`'a`必须至少和生命周期`'b`一样长，或者比`'b`更长。


### 静态生命周期
静态生命周期`'static`是特殊的生命周期：
* 生命周期 'static 意味着能和程序活得一样久，例如字符串字面量和特征对象
* 实在遇到解决不了的生命周期标注问题，可以尝试 T: 'static，有时候它会给你奇迹

关于 'static, 有两种用法: &'static 和 T: 'static，这会在高级进阶阶段学习。