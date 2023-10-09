use uuid::Uuid;
/***
 * エンティティ
 * - エンティティは可変である
 * - エンティティは同じ属性であっても区別される
 * - エンティティは同一性によって区別される
 *
 * 値オブジェクトとエンティティの違いは、同じ属性を持っていても区別されるかどうかである
 * 例えば、ユーザーの名前は同じでも、ユーザーのIDが異なれば、別のユーザーとして区別される
 * 加えて、エンティティは可変であるため、ライフタイムを通じて値が変更される可能性がある
 * 基本的には値オブジェクトを使い、必要に応じてエンティティへと変更する
 */
#[derive(Clone)]
pub struct User {
    id: uuid::Uuid,
    user_name: UserName,
}

impl User {
    pub fn new(name: UserName) -> User {
        let id = Uuid::new_v4();

        User {
            id: id,
            user_name: name,
        }
    }

    // 可変なので、値を変更することができる
    pub fn change_name(&mut self, first_name: String, last_name: String) {
        self.user_name.first_name = first_name;
        self.user_name.last_name = last_name;
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
}

// impl Eq for User {}
impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl std::fmt::Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "User: {} {} {}",
            self.id, self.user_name.first_name, self.user_name.last_name,
        )
    }
}

#[cfg(test)]
mod user_tests {
    use super::*;

    #[test]
    fn test_user() {
        let name = UserName::new("John".to_string(), "Doe".to_string()).unwrap();
        let user = User::new(name);
        assert_eq!(user.user_name.first_name, "John".to_string());
        assert_eq!(user.user_name.last_name, "Doe".to_string());
    }

    #[test]
    fn test_eq() {
        let name = UserName::new("Taro".to_string(), "Yamada".to_string()).unwrap();
        let me = User::new(name);
        let same_name = UserName::new("Taro".to_string(), "Yamada".to_string()).unwrap();
        let doppelganger = User::new(same_name);
        assert_ne!(me, doppelganger);
    }

    #[test]
    fn test_rename() {
        let name = UserName::new("Taro".to_string(), "Yamada".to_string()).unwrap();
        let mut me = User::new(name);
        me.change_name("Jiro".to_string(), "Sato".to_string());
        assert_eq!(me.user_name.first_name, "Jiro".to_string());
    }
}

#[derive(Clone)]
pub struct UserName {
    first_name: String,
    last_name: String,
}

impl UserName {
    pub fn new(first_name: String, last_name: String) -> Result<UserName, String> {
        // 3文字以下の名前は許容しない
        if first_name.len() < 3 || last_name.len() < 3 {
            return Err("名前は3文字以上です".to_string());
        }
        Ok(UserName {
            first_name: first_name,
            last_name: last_name,
        })
    }
}

mod user_name_tests {
    use super::*;

    #[test]
    fn test_new() {
        let name = UserName::new("Taro".to_string(), "Yamada".to_string());
        assert_eq!(name.is_ok(), true);
    }

    #[test]
    fn test_new_error() {
        let name = UserName::new("Taro".to_string(), "Ya".to_string());
        assert_eq!(name.is_err(), true);
    }
}
