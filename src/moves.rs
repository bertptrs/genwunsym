use std::num::NonZeroU8;

use num::rational::Ratio;
use rand::Rng;

use crate::stats::Modifier;
use crate::types::Type;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MoveEffect {
    /// Normal, damaging move. The u8 is its power.
    Normal,
    /// Move with recoil. Divider is the fraction of the damage that will be dealt as recoil.
    Recoil(NonZeroU8),
    // Explosion or self-destruct
    SelfKO,
}

const STRUGGLE: Move = Move {
    power: unsafe { Some(NonZeroU8::new_unchecked(50)) },
    accuracy: unsafe { Some(NonZeroU8::new_unchecked(255)) },
    effect: unsafe { MoveEffect::Recoil(NonZeroU8::new_unchecked(2)) },
    move_type: Type::Normal,
};

/// A move a pokemon could use.
pub struct Move {
    power: Option<NonZeroU8>,
    /// Accuracy for the move on a 0..255 scale. None means move always hits.
    accuracy: Option<NonZeroU8>,
    effect: MoveEffect,
    move_type: Type,
}

impl Move {
    /// Fallback move for when no other move is available.
    ///
    /// This move returns a reference to some internal constant for the move struggle.
    pub fn fallback() -> &'static Move {
        &STRUGGLE
    }

    pub fn hits(&self, rand: &mut impl Rng, accuracy: Modifier, evasion: Modifier) -> bool {
        if let Some(acc) = self.accuracy {
            let acc = Ratio::from_integer(u16::from(acc.get()));
            let acc = acc * accuracy.get_ratio();
            let acc = acc.trunc() / evasion.get_ratio();
            let r: u8 = rand.gen();

            acc.to_integer() > u16::from(r)
        } else {
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::rngs::mock::StepRng;

    use super::*;

    #[test]
    fn test_hit_calc() {
        let m = Move::fallback();
        let mut rng = StepRng::new(254, 1);
        // First one struggle should hit.
        assert_eq!(true, m.hits(&mut rng, Modifier::Neutral, Modifier::Neutral));
        // Second should be affected by the 1/256 glitch.
        assert_eq!(
            false,
            m.hits(&mut rng, Modifier::Neutral, Modifier::Neutral)
        );

        // Test how many hits we get for all possible random numbers
        let hits = (0..=0xff)
            .filter(|_| m.hits(&mut rng, Modifier::Min3, Modifier::Plus1))
            .count();

        // Should hit 255 * (2 / 5) / (3 / 2) = 68 times
        assert_eq!(68, hits);
    }
}
