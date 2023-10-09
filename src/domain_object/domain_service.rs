use crate::repository::IUserRepository;

use super::entity::User;

#[derive(Clone, Debug)]
pub struct UserService<Repo: IUserRepository> {
    repo: Repo,
}

impl<Repo> UserService<Repo>
where
    Repo: IUserRepository,
{
    pub fn new(repo: &Repo) -> Self {
        Self { repo: repo.clone() }
    }

    pub fn exists(&self, user: &User) -> bool {
        self.repo.find(user.clone()).is_some()
    }
}
