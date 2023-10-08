mod domain_object;
mod repository;

use domain_object::domain_service::UserService;
use domain_object::entity::{User, UserName};
use repository::{IUserRepository, UserRepository};

fn main() -> Result<(), String> {
    let mut user_repository = UserRepository::new();
    let user_service = UserService::new(user_repository);

    let user_name = UserName::new("Joen".to_string(), "Doe".to_string())?;

    let user = User::new(user_name);
    if user_service.exists(&user) {
        return Err("ユーザーは既に存在しています".to_string());
    }
    // serviceでrepositoryの所有権を奪っているので、ここでuser_repositoryを使うことはできない
    user_repository.save(user.clone());
    Ok(())
}
