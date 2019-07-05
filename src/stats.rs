use std::ops::Add;

use num::rational::Ratio;

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum Stat {
    HP,
    Attack,
    Defense,
    Special,
    Speed,
    Accuracy,
    Evasion,
}

impl From<Stat> for usize {
    fn from(stat: Stat) -> Self {
        use self::Stat::*;

        match stat {
            HP => 0,
            Attack => 1,
            Defense => 2,
            Special => 3,
            Speed => 4,
            Accuracy => 5,
            Evasion => 6,
        }
    }
}

/// Container for values for all stats.
pub type StatSet = [u16; 5];

/// Maximum attainable individual values.
pub const PERFECT_IVS: StatSet = [0xf; 5];
/// Maximum attainable effort values.
pub const PERFECT_EVS: StatSet = [0xffff; 5];

/// Stat boost modifier.
///
/// Represents any of the 13 levels of stat boost; 6 levels in either
/// direction.
///
/// Additions on this object are defined as saturating at the boundaries.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Modifier(i8);

impl Modifier {
    pub fn modify(self, stat: u8) -> u16 {
        let stat = Ratio::from_integer(u16::from(stat));

        let multiplier = self.get_ratio();
        let result = stat * multiplier;
        result.to_integer().min(999)
    }

    /// Get the ratio of the modifier for this level of boost.
    ///
    /// In RBY, this ratio is the same for all stats. In colosseum, a
    /// different table is used for evasion and accuracy. This ratio is
    /// the general one.
    pub fn get_ratio(self) -> Ratio<u16> {
        match self.0 {
            // Negative cases need to be spelled out for specific rounding issues
            -6 => Ratio::new(1, 4),
            -5 => Ratio::new(28, 100),
            -4 => Ratio::new(33, 100),
            -3 => Ratio::new(40, 100),
            -2 => Ratio::new(1, 2),
            -1 => Ratio::new(66, 100),
            // Neutral and positive cases follow a nice pattern.
            x => Ratio::new(x as u16 + 2, 2)
        }
    }
}

impl Add<i8> for Modifier {
    type Output = Modifier;

    /// Implement adding a change to a modifier.
    ///
    /// The resulting addition will saturate at the limits of
    /// the boost.
    fn add(self, rhs: i8) -> Self::Output {
        (self.0 + rhs).into()
    }
}

impl Add<Modifier> for Modifier {
    type Output = Modifier;

    fn add(self, rhs: Modifier) -> Self::Output {
        (self.0 + rhs.0).into()
    }
}

impl From<i8> for Modifier {
    fn from(level: i8) -> Self {
        let level = level.max(-6).min(6);
        Modifier(level)
    }
}
