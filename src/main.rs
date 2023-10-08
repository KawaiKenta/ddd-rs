mod entity;
mod value_object;

use entity::User;

fn main() {
    let me = User::new("Taro".to_string(), "Yamada".to_string());
    let doppelganger = User::new("Taro".to_string(), "Yamada".to_string());
    println!("me: {:?}", me);
    println!("doppelganger: {:?}", doppelganger);
    println!("me == doppelganger: {}", me == doppelganger);
    println!("me == me: {}", me == me)
}
