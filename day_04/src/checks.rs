pub trait AdjacentGroup {
    fn has_adjacent_group(&self) -> bool;
}

impl AdjacentGroup for u32 {
    fn has_adjacent_group(&self) -> bool {
        let digits: Vec<char> = self.to_string().chars().collect();
        let mut has_group = false;

        for (i, digit) in digits.iter().enumerate() {
            if i == digits.len() - 1 {
                break;
            }

            if *digit == digits[i + 1] {
                has_group = true;
                break;
            }
        }

        has_group
    }
}

pub trait AdjacentPair {
    fn has_adjacent_pair(&self) -> bool;
}

impl AdjacentPair for u32 {
    fn has_adjacent_pair(&self) -> bool {
        let digits: Vec<char> = self.to_string().chars().collect();

        if digits.len() < 2 {
            return false;
        }

        if digits.len() == 2 {
            return digits[0] == digits[1];
        }

        let mut has_pair = false;

        for (i, digit) in digits.iter().enumerate() {
            if i == digits.len() - 2 {
                if *digit == digits[i + 1] && *digit != digits[i - 1] {
                    has_pair = true;
                }

                break;
            }

            if i == 0 {
                if *digit == digits[1] && *digit != digits[2] {
                    has_pair = true;
                    break;
                }

                continue;
            }

            if *digit != digits[i - 1] && *digit == digits[i + 1] && *digit != digits[i + 2] {
                has_pair = true;
                break;
            }
        }

        has_pair
    }
}

pub trait AlwaysIncreases {
    fn always_increases(&self) -> bool;
}

impl AlwaysIncreases for u32 {
    fn always_increases(&self) -> bool {
        let digits: Vec<u32> = self
            .to_string()
            .chars()
            .map(|x| x.to_digit(10).unwrap())
            .collect();

        let mut increases = true;

        for (i, digit) in digits.iter().enumerate() {
            if i == digits.len() - 1 {
                break;
            }

            if *digit > digits[i + 1] {
                increases = false;
                break;
            }
        }

        increases
    }
}

#[cfg(test)]
mod tests {
    use super::{AdjacentGroup, AdjacentPair, AlwaysIncreases};

    #[test]
    fn adjacent_group() {
        assert!(1234556789.has_adjacent_group());
        assert!(1111111111.has_adjacent_group());
        assert!(!123456789.has_adjacent_group());
        assert!(!123789.has_adjacent_group());
    }

    #[test]
    fn adjacent_pair() {
        assert!(1234556789.has_adjacent_pair());
        assert!(112233.has_adjacent_pair());
        assert!(111122.has_adjacent_pair());
        assert!(!1111111111.has_adjacent_pair());
        assert!(!123456789.has_adjacent_pair());
        assert!(!123444.has_adjacent_pair());
    }

    #[test]
    fn always_increases() {
        assert!(123456789.always_increases());
        assert!(111111111.always_increases());
        assert!(!987654321.always_increases());
        assert!(!1234567890.always_increases());
        assert!(!223450.always_increases());
    }
}
