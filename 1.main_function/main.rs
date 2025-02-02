//主函数，没有返回值
fn main()
{
    let a = 10;//自动类型推断默认i32，不可变变量
    let b: i32 = 20;// 显式指定类型
    let mut c = 30i32;// 可变变量
    c = 40;
    let d = 30_i32;// 下划线分隔数字，提高可读性
    let e = add(add(a, b), add(c,d));// 函数调用
    println!("(a+b)+(c+d)={}", e);// 宏调用（不是函数），{}为占位符
}

fn add(i:i32,j:i32) -> i32
{
    i+j
}
//当最后一行是表达式时，可以省略return关键字