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
pub struct User {
    id: uuid::Uuid,
    first_name: String,
    last_name: String,
}

impl User {
    pub fn new(first_name: String, last_name: String) -> User {
        let id = Uuid::new_v4();
        User {
            id: id,
            first_name,
            last_name,
        }
    }

    // 可変なので、値を変更することができる
    pub fn change_name(&mut self, first_name: String, last_name: String) {
        self.first_name = first_name;
        self.last_name = last_name;
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
            self.id, self.first_name, self.last_name
        )
    }
}

// test
#[cfg(test)]
mod user_tests {
    use super::*;

    #[test]
    fn test_user() {
        let user = User::new("John".to_string(), "Doe".to_string());
        assert_eq!(user.first_name, "John".to_string());
        assert_eq!(user.last_name, "Doe".to_string());
    }

    #[test]
    fn test_eq() {
        let me = User::new("Taro".to_string(), "Yamada".to_string());
        let doppelganger = User::new("Taro".to_string(), "Yamada".to_string());
        assert_ne!(me, doppelganger);
    }

    #[test]
    fn test_overwrite() {
        let mut user = User::new("John".to_string(), "Doe".to_string());
        user.change_name("Jane".to_string(), "Doe".to_string());
        assert_eq!(user.first_name, "Jane".to_string());
    }
}
