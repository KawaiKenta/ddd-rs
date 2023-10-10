use crate::{
    domain_object::{
        domain_service::UserService,
        entity::{User, UserName},
    },
    repository::IUserRepository,
};

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
