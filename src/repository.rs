use std::{
    collections::HashMap,
    fmt,
    sync::{Arc, Mutex},
};

use crate::domain_object::entity::User;

/***
 * リポジトリ
 * - リポジトリはデータを永続化し、再構築するといった処理を抽象的に扱うためのオブジェクト
 * - ドメインオブジェクトが直接的にデータストアに書き込みを⾏う処理を実⾏するのではなく、リポジトリにインスタンスの永続化を依頼します
 */
pub trait IUserRepository: Clone + fmt::Debug {
    fn save(&self, user: User) -> Result<(), String>;
    fn find(&self, user: User) -> Option<User>;
}

#[derive(Clone, Debug)]
pub struct InmemoryUserRepository {
    store: Arc<Mutex<HashMap<uuid::Uuid, User>>>,
}

impl InmemoryUserRepository {
    pub fn new() -> Self {
        Self {
            store: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl IUserRepository for InmemoryUserRepository {
    fn save(&self, user: User) -> Result<(), String> {
        let store = self.store.clone();
        let mut store = store.try_lock().map_err(|_| "failed to lock")?;
        store.insert(user.id(), user);
        Ok(())
    }

    fn find(&self, user: User) -> Option<User> {
        let store = self.store.clone();
        let store = store.try_lock().map_err(|_| "failed to lock").unwrap();
        store.get(&user.id()).map(|u| u.clone())
    }
}
