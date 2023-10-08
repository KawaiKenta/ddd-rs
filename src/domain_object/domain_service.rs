use super::entity::User;

pub struct UserService {
    user_repository: UserRepository,
}

impl UserService {
    pub fn new(user_repository: UserRepository) -> UserService {
        UserService {
            user_repository: user_repository,
        }
    }

    pub fn exists(&self, user: &User) -> bool {
        // repositoryに存在するかどうかを確認する
        for u in &self.user_repository.users {
            if u == user {
                return true;
            }
        }
        false
    }

    pub fn save(&mut self, user: User) {
        self.user_repository.users.push(user);
    }
}

pub struct UserRepository {
    users: Vec<User>,
}

impl UserRepository {
    pub fn new() -> UserRepository {
        UserRepository { users: vec![] }
    }
}
