enum MyEnum {
    Foo,
    Bar,
}

fn main() {
    let mut count = 0;

    let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
    for e in &v { // 使用引用
        if e == &MyEnum::Foo { // 比较引用
            count += 1;
        }
    }

    assert_eq!(count, 2);
}