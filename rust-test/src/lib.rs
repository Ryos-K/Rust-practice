pub struct Pizza {
    pub topping: String,
    pub inches: u8,
}

impl Pizza {
    pub fn pepperoni(inches: u8) -> Self {
        Pizza::bake("pepperoni", inches)
    }

    pub fn mozzarella(inches: u8) -> Self {
        Pizza::bake("mozzarella", inches)
    }

    fn bake(topping: &str, inches: u8) -> Self {
        Pizza {
            topping: String::from(topping),
            inches,
        }
    }
}

// mod ex2;
// /// Generally, the first line is a brief summary describing the function.
// ///
// /// The next lines present detailed documentation.
// /// Code blocks start with triple backticks. The code has an implicit `fn main()` inside and `extern crate <cratename>`,
// /// which means you can just start writing code.
// ///
// /// ```
// /// let result = rust_test::add(2, 3);
// /// assert_eq!(result, 5);
// /// ```
// pub fn add(a: i32, b: i32) -> i32 {
//     a + b
// }
