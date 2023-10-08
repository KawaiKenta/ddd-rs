mod domain_object;

use domain_object::{
    domain_service::{UserRepository, UserService},
    entity::{User, UserName},
};
fn main() {
    let mut user_service = UserService::new(UserRepository::new());
    let first_name = "Taro".to_string();
    let last_name = "Yamada".to_string();
    match create_user(&mut user_service, first_name, last_name) {
        Ok(_) => println!("ユーザーを作成しました"),
        Err(e) => println!("{}", e),
    }
}

fn create_user(us: &mut UserService, first_name: String, last_name: String) -> Result<(), String> {
    let name = match UserName::new(first_name, last_name) {
        Ok(name) => name,
        Err(e) => panic!("{}", e),
    };
    let user = User::new(name);
    if us.exists(&user) {
        return Err("ユーザーは既に存在しています".to_string());
    }
    us.save(user);
    Ok(())
}
