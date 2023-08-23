use rust_examples::calc;

fn main() {
    let x1 = 1;
    let x2 = 2;
    let x3 = calc::add(&x1, &x2);
    println!("{} + {} = {}", x1, x2, x3);
}
