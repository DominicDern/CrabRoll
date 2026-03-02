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
    rolls: Vec<MicroRoll>,
}

impl Roll {}
struct MicroRoll {
    die: Die,
    modifier: u8,
    dmg_type: String,
}

impl MicroRoll {
    fn roll(&self) -> (u16, String) {
        let roll: u16 = match &self.die {
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

impl MicroRoll {
    fn new(die: Die, modifier: u8, dmg_type: String) -> Self {
        Self {
            die,
            modifier,
            dmg_type,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dice_testing() {
        for _ in 0..10 {
            let d2 = MicroRoll::new(Die::D2, 0, "test".to_string()).roll().0;
            let d4 = MicroRoll::new(Die::D4, 0, "test".to_string()).roll().0;
            let d6 = MicroRoll::new(Die::D6, 0, "test".to_string()).roll().0;
            let d8 = MicroRoll::new(Die::D8, 0, "test".to_string()).roll().0;
            let d10 = MicroRoll::new(Die::D10, 0, "test".to_string()).roll().0;
            let d12 = MicroRoll::new(Die::D12, 0, "test".to_string()).roll().0;
            let d20 = MicroRoll::new(Die::D20, 0, "test".to_string()).roll().0;
            let percentile = MicroRoll::new(Die::Percentile, 0, "test".to_string())
                .roll()
                .0;
            assert!((1..=40).contains(&d2));
            assert!((1..=80).contains(&d4));
            assert!((1..=120).contains(&d6));
            assert!((1..=160).contains(&d8));
            assert!((1..=200).contains(&d10));
            assert!((1..=240).contains(&d12));
            assert!((1..=400).contains(&d20));
            assert!((1..=2000).contains(&percentile));
        }
    }
}
