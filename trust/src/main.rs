use trust::add;
use trust::div;
use trust::mul;
use trust::sub;

fn main() {
    println!("1 + 2 = {}", add(1, 2));
    println!("1 - 2 = {}", sub(1, 2));
    println!("1 * 2 = {}", mul(1, 2));
    println!("1 / 2 = {}", div(1, 2));
}
