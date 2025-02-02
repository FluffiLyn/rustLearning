fn main() {
    let mut cnt = 0;
    let result = loop {
        cnt += 1;
        if cnt == 10 {
            break cnt * 2;
        }
    };
    
    println!("result is {}", result);
}