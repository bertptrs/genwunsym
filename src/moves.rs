use std::num::NonZeroU8;

use rand::Rng;

use crate::types::Type;

#[derive(Copy, Clone, Debug)]
pub enum MoveEffect {
    /// Normal, damaging move. The u8 is its power.
    Damage(u8),
    /// Move with recoil. Divider is the fraction of the damage that will be dealt as recoil.
    Recoil { power: u8, divider: NonZeroU8 },
}

const STRUGGLE: Move = Move {
    accuracy: unsafe { Some(NonZeroU8::new_unchecked(255)) },
    effect: MoveEffect::Recoil {
        power: 50,
        divider: unsafe { NonZeroU8::new_unchecked(2) },
    },
    move_type: Type::Normal,
    pp: 0,
};

/// A move a pokemon could use.
pub struct Move {
    /// Accuracy for the move on a 0..255 scale. None means move always hits.
    accuracy: Option<NonZeroU8>,
    effect: MoveEffect,
    move_type: Type,
    /// Initial PP that a move would have. 0 means infinite.
    pp: u8,
}

impl Move {
    /// Fallback move for when no other move is available.
    ///
    /// This move returns a reference to some internal constant for the move struggle.
    pub fn fallback() -> &'static Move {
        &STRUGGLE
    }

    pub fn hits(&self, rand: &mut impl Rng) -> bool {
        if let Some(acc) = self.accuracy {
            let r = rand.gen();
            acc.get() > r
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
        assert_eq!(true, m.hits(&mut rng));
        // Second should be affected by the 1/256 glitch.
        assert_eq!(false, m.hits(&mut rng));
    }
}
