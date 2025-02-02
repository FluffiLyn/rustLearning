
// 填空并修复错误
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
fn main() {
    let v = Point(0, 127, 255);
    check_color(v);
}   

fn check_color(p: Point) {
    let (x, y, z) = (p.0, p.1, p.2);
    assert_eq!(x, 0);
    assert_eq!(y, 127);
    assert_eq!(z, 255);
 }