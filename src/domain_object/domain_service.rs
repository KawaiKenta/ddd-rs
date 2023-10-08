use crate::repository::IUserRepository;

use super::entity::User;

pub struct UserService<T: IUserRepository> {
    user_repository: T,
}

impl<T: IUserRepository> UserService<T> {
    pub fn new(user_repository: T) -> UserService<T> {
        UserService {
            user_repository: user_repository,
        }
    }

    pub fn exists(&self, user: &User) -> bool {
        match self.user_repository.find(user.clone()) {
            Some(_) => true,
            None => false,
        }
    }
}
