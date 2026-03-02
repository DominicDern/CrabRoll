use rand::prelude::*;

enum Die {
    D2,
    D4,
    D6,
    D8,
    D10,
    D12,
    D20,
    Percentile,
}

// Rolls are made up
pub struct Roll {
    rolls: Vec<MonoRoll>,
}
struct MonoRoll {
    dice: Die,
    modifier: u8,
    dmg_type: String,
}

impl MonoRoll {
    fn roll(&self) -> (u16, String) {
        let roll: u16 = match &self.dice {
            Die::D2 => rand::random_range(1..=2),
            Die::D4 => rand::random_range(1..=4),
            Die::D6 => rand::random_range(1..=6),
            Die::D8 => rand::random_range(1..=8),
            Die::D10 => rand::random_range(1..=10),
            Die::D12 => rand::random_range(1..=12),
            Die::D20 => rand::random_range(1..=20),
            Die::Percentile => rand::random_range(1..=100),
        };
        (roll, self.dmg_type.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

        // let result = add(2, 2);
        // assert_eq!(result, 4);
    }
}
