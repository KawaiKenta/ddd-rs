use crate::domain_object::entity::User;

/***
 * リポジトリ
 * - リポジトリはデータを永続化し、再構築するといった処理を抽象的に扱うためのオブジェクト
 * - ドメインオブジェクトが直接的にデータストアに書き込みを⾏う処理を実⾏するのではなく、リポジトリにインスタンスの永続化を依頼します
 */
pub trait IUserRepository {
    fn save(&mut self, user: User);
    fn find(&self, user: User) -> Option<User>;
}

pub struct UserRepository {
    users: Vec<User>,
}

impl UserRepository {
    pub fn new() -> UserRepository {
        UserRepository { users: vec![] }
    }
}

impl IUserRepository for UserRepository {
    fn save(&mut self, user: User) {
        self.users.push(user);
    }

    fn find(&self, user: User) -> Option<User> {
        for u in &self.users {
            if u == &user {
                return Some(u.clone());
            }
        }
        None
    }
}
