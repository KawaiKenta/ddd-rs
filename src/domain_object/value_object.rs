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
pub struct Money {
    amount: usize,
    currency: String,
}

impl Money {
    pub fn new(amount: usize, currency: String) -> Money {
        Money { amount, currency }
    }
    // 値オブジェクトは振る舞いを持てる
    pub fn add(&self, other: Money) -> Result<Money, String> {
        if self.currency != other.currency {
            return Err("通貨単位が異なります".to_string());
        }

        Ok(Money {
            amount: self.amount + other.amount,
            currency: self.currency.clone(),
        })
    }
}

#[cfg(test)]
mod money_tests {
    use super::*;

    #[test]
    fn test_money() {
        let money = Money::new(1000, "JPY".to_string());
        assert_eq!(money.amount, 1000);
        assert_eq!(money.currency, "JPY".to_string());
    }

    #[test]
    fn test_eq() {
        let money = Money::new(1000, "JPY".to_string());
        let same_money = Money::new(1000, "JPY".to_string());
        assert_eq!(money, same_money);
    }

    #[test]
    fn test_add() {
        let me = Money::new(1000, "JPY".to_string());
        let other = Money::new(3000, "JPY".to_string());

        let result = match me.add(other) {
            Ok(money) => money,
            Err(err) => panic!("{}", err),
        };
        assert_eq!(result.amount, 4000);
    }

    #[test]
    fn test_err_with_diff_currency() {
        let me = Money::new(1000, "JPY".to_string());
        let other = Money::new(3000, "USD".to_string());

        let result = me.add(other);
        assert_eq!(result, Err("通貨単位が異なります".to_string()));
    }
}
