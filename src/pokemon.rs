use crate::stats::{Stat, StatSet};
use crate::utils::IntegerSquareRoot;

#[derive(Debug)]
pub struct Pokemon {
    pub level: u8,
    base_stats: StatSet,
    evs: StatSet,
    ivs: StatSet,
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
}

#[cfg(test)]
mod tests {
    use crate::stats::{PERFECT_EVS, PERFECT_IVS};

    use super::*;

    #[test]
    fn test_stat_calc() {
        let mew = Pokemon {
            level: 100,
            base_stats: [100; 5],
            evs: PERFECT_EVS,
            ivs: PERFECT_IVS,
        };

        assert_eq!(403, mew.get_stat(Stat::HP));
        assert_eq!(298, mew.get_stat(Stat::Special));
    }
}
