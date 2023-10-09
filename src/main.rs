mod domain_object;
mod repository;

use domain_object::domain_service::UserService;
use domain_object::entity::{User, UserName};
use repository::{IUserRepository, InmemoryUserRepository};

fn main() -> Result<(), String> {
    let repo = InmemoryUserRepository::new();
    // 内部でrepoをcloneしている。普通だと失敗しそうだが Arc<Mutex<HashMap<uuid::Uuid, User>>>であるため、repoの内部を共有できている
    let user_service = UserService::new(&repo);
    let user_application_service = UserApplicationService::new(repo, user_service);
    for i in 0..3 {
        user_application_service.register(format!("taro{}", i), format!("yamada{}", i))?;
    }

    println!("{:#?}", user_application_service.repo);
    Ok(())
}

pub struct UserApplicationService<Repo: IUserRepository> {
    repo: Repo,
    user_service: UserService<Repo>,
}

impl<Repo> UserApplicationService<Repo>
where
    Repo: IUserRepository,
{
    pub fn new(repo: Repo, user_service: UserService<Repo>) -> Self {
        Self { repo, user_service }
    }

    pub fn register(&self, fname: String, lname: String) -> Result<(), String> {
        let user_name = UserName::new(fname, lname)?;
        let user = User::new(user_name);
        if self.user_service.exists(&user) {
            return Err("ユーザーは既に存在しています".to_string());
        }
        self.repo.save(user)
    }
}
