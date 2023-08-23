use rust_examples::animals::dog::Dog;

fn main() {
    let dog = Dog {
        name: "Rusty",
        age: 4,
    };
    println!("Hello: {}", dog.name);
}
