use crate::stats::{PERFECT_EVS, PERFECT_IVS, Stat, StatSet};
use crate::types::Type;
use crate::utils::IntegerSquareRoot;

#[derive(Debug)]
pub struct Pokemon {
    pub level: u8,
    pub base_stats: StatSet,
    pub evs: StatSet,
    pub ivs: StatSet,
    pub types: [Option<Type>; 2],
}

impl Pokemon {
    /// Calculate the stat value given all other parameters.
    ///
    /// This computes the raw, unmodified stat based on the level,
    /// base stats, effort values and individual values of the pokemon.
    pub fn get_stat(&self, stat: Stat) -> u16 {
        match stat {
            // Querying these doesn't make sense.
            Stat::Accuracy | Stat::Evasion => unimplemented!(),
            stat => {
                let l = u16::from(self.level);
                let ev = self.evs[usize::from(stat)];
                let bs = self.base_stats[usize::from(stat)];
                let iv = self.ivs[usize::from(stat)];

                let s = ev.saturating_sub(1).isqrt() + 1;
                let s = s / 4;
                let s = s.min(63);
                let s = s + 2 * (iv + bs);

                let c_l = if stat == Stat::HP { l + 10 } else { 5 };

                s * l / 100 + c_l
            }
        }
    }

    /// Check if this pokemon has the wanted type.
    pub fn has_type(&self, wanted: Type) -> bool {
        self.types.iter().any(|&x| x == Some(wanted))
    }

    pub fn get_types(&self) -> &[Option<Type>] {
        &self.types
    }
}

/// Generic pokemon stats.
///
/// The default pokemon has the base stats and type of Rattata, perfect IVs and EVs, and is
/// at level 100. Override as needed.
impl Default for Pokemon {
    fn default() -> Self {
        Pokemon {
            level: 100,
            base_stats: [30, 56, 35, 25, 72],
            evs: PERFECT_EVS,
            ivs: PERFECT_IVS,
            types: [Some(Type::Normal), None],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stat_calc() {
        let mew = Pokemon {
            base_stats: [100; 5],
            ..Default::default()
        };

        assert_eq!(403, mew.get_stat(Stat::HP));
        assert_eq!(298, mew.get_stat(Stat::Special));
    }

    #[test]
    fn test_has_type() {
        let pokemon = Pokemon {
            types: [Some(Type::Normal), Some(Type::Fire)],
            ..Default::default()
        };

        assert!(pokemon.has_type(Type::Normal));
        assert!(pokemon.has_type(Type::Fire));
        assert!(!pokemon.has_type(Type::Dragon));
    }
}
