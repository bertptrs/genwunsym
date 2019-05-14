use std::ops::Add;

use num::rational::Ratio;

/// Stat boost modifier.
///
/// Represents any of the 13 levels of stat boost; 6 levels in either
/// direction.
#[derive(Copy, Clone, Debug)]
pub enum Modifier {
    Min6,
    Min5,
    Min4,
    Min3,
    Min2,
    Min1,
    Neutral,
    Plus1,
    Plus2,
    Plus3,
    Plus4,
    Plus5,
    Plus6,
}

impl Modifier {
    pub fn modify(self, stat: u8) -> u16 {
        let stat = Ratio::from_integer(stat as u16);

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
        use self::Modifier::*;

        match self {
            // Negative cases need to be spelled out for specific rounding issues
            Min6 => Ratio::new(1, 4),
            Min5 => Ratio::new(28, 100),
            Min4 => Ratio::new(33, 100),
            Min3 => Ratio::new(40, 100),
            Min2 => Ratio::new(1, 2),
            Min1 => Ratio::new(66, 100),
            // Neutral and positive cases follow a nice pattern.
            x => {
                let rank: i8 = x.into();
                Ratio::new(rank as u16 + 2, 2)
            }
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
        let cur: i8 = self.into();
        let new = rhs + cur;
        new.into()
    }
}

impl From<i8> for Modifier {
    fn from(level: i8) -> Self {
        use self::Modifier::*;

        match level {
            -128..=-6 => Min6,
            -5 => Min5,
            -4 => Min4,
            -3 => Min3,
            -2 => Min2,
            -1 => Min1,
            0 => Neutral,
            1 => Plus1,
            2 => Plus2,
            3 => Plus3,
            4 => Plus4,
            5 => Plus5,
            6..=127 => Plus6,
        }
    }
}

impl From<Modifier> for i8 {
    fn from(m: Modifier) -> Self {
        use self::Modifier::*;
        match m {
            Min6 => -6,
            Min5 => -5,
            Min4 => -4,
            Min3 => -3,
            Min2 => -2,
            Min1 => -1,
            Neutral => 0,
            Plus1 => 1,
            Plus2 => 2,
            Plus3 => 3,
            Plus4 => 4,
            Plus5 => 5,
            Plus6 => 6,
        }
    }
}
