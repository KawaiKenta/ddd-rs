use crate::value_object::Money;

mod value_object;

fn main() {
    let me = Money::new(1000, "JPY".to_string());
    let other = Money::new(3000, "USD".to_string());

    let result = match me.add(other) {
        Ok(money) => money,
        Err(err) => panic!("{}", err),
    };
    println!("result: {:?}", result);
}
