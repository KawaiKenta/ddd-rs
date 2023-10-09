use std::{marker::PhantomData, ops::Add};

/***
 * 値オブジェクト
 * - 値オブジェクトは、値を表すオブジェクト
 * - 値オブジェクトは、不変である
 * - 値オブジェクトは、交換が可能である
 * - 値オブジェクトは、等価性によって比較される
 *
 * 値オブジェクトを定義することによって自分がどういうものであるか自己文書化を推し進めることができる
 */
#[derive(PartialEq, Eq, Debug)]
pub struct User {
    first_name: String,
    last_name: String,
}

impl User {
    pub fn new(first_name: String, last_name: String) -> User {
        User {
            first_name,
            last_name,
        }
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
        let user = User::new("John".to_string(), "Doe".to_string());
        let same_user = User::new("John".to_string(), "Doe".to_string());
        assert_eq!(user, same_user);
    }

    #[test]
    fn test_overwrite() {
        let mut user = User::new("John".to_string(), "Doe".to_string());
        user = User::new("Jane".to_string(), "Doe".to_string());
        assert_eq!(user.first_name, "Jane".to_string());
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Money<T> {
    amount: usize,
    currency: PhantomData<T>,
}

impl<T> Money<T> {
    pub fn new(amount: usize) -> Self {
        Self {
            amount: amount,
            currency: PhantomData::<T>,
        }
    }
}

impl<T> Add for Money<T> {
    type Output = Money<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.amount + rhs.amount)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum JPY {}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum USD {}

#[cfg(test)]
mod money_tests {
    use super::*;

    #[test]
    fn test_money() {
        let money = Money::<JPY>::new(1000);
        assert_eq!(money.amount, 1000);
        assert_eq!(money.currency, PhantomData::<JPY>);
    }

    #[test]
    fn test_eq() {
        let money = Money::<JPY>::new(1000);
        let same_money = Money::<JPY>::new(1000);

        assert_eq!(money, same_money);
    }

    #[test]
    fn test_add() {
        let me = Money::<JPY>::new(1000);
        let other = Money::<JPY>::new(3000);

        let result = me + other;
        assert_eq!(result.amount, 4000);
    }
}
