mod application_service;
mod domain_object;
mod repository;

use application_service::UserApplicationService;
use domain_object::domain_service::UserService;
use repository::InmemoryUserRepository;

fn main() -> Result<(), String> {
    let repo = InmemoryUserRepository::new();
    // 内部でrepoをcloneしている。普通だと失敗しそうだが Arc<Mutex<HashMap<uuid::Uuid, User>>>であるため、repoの内部を共有できている
    let user_service = UserService::new(&repo);
    let user_application_service = UserApplicationService::new(repo, user_service);
    for i in 0..3 {
        user_application_service.register(format!("taro{}", i), format!("yamada{}", i))?;
    }
    Ok(())
}
