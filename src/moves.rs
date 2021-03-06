use std::num::NonZeroU8;

use num::rational::Ratio;
use rand::Rng;

use crate::battle::BattleState;
use crate::stats::{Modifier, Stat};
use crate::types::Type;
use crate::pokemon::Pokemon;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MoveEffect {
    /// Normal, damaging move. The u8 is its power.
    Normal,
    /// Move with recoil. Divider is the fraction of the damage that will be dealt as recoil.
    Recoil(NonZeroU8),
    /// Explosion or self-destruct
    SelfKO,
    /// High critical hit ratio
    HighCrit,
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

    /// Compute the damage for when the attacker hits the defender with this particular move.
    pub fn damage(
        &self,
        rand: &mut impl Rng,
        attacker: &BattleState,
        defender: &BattleState,
    ) -> u16 {
        if self.power.is_none() {
            return 0;
        }

        let power = u32::from(self.power.unwrap().get());
        let (mut attack, mut defense) = if self.move_type.is_physical() {
            (attacker[Stat::Attack], defender[Stat::Defense])
        } else {
            (attacker[Stat::Special], defender[Stat::Special])
        };

        // TODO: badge bonus
        if self.effect == MoveEffect::SelfKO {
            defense /= 2;
        }

        // Simultaneous reduction
        if attack > 255 || defense > 255 {
            attack /= 4;
            attack &= 0xff;
            defense /= 4;
            defense &= 0xff;
        }

        let mut damage = power * u32::from(attack).max(1);
        // TODO: critical hits
        damage *= 2 * u32::from(attacker.pokemon().level / 5 + 2);
        // TODO: light screen & reflect
        damage /= u32::from(defense).max(1);
        damage = 2 + 997.min(damage / 50);

        // Same-Type Attack bonus
        if attacker.pokemon().has_type(self.move_type) {
            damage = damage * 3 / 2;
        }

        damage = self.apply_type_effectiveness(defender.pokemon(), damage);

        // gen_range is open ended at the high end
        let r: u32 = rand.gen_range(217, 256);

        (damage * r / 255) as u16
    }

    fn apply_type_effectiveness(&self, defender: &Pokemon, damage: u32) -> u32 {
        let mut damage = Ratio::from_integer(damage);
        for defender_type in defender.get_types().iter().filter_map(|&x| x) {
            damage *= self.move_type.effectiveness(defender_type).get_modifier();
        }

        damage.to_integer()
    }

    /// Check if this move is a critical hit.
    ///
    /// This function implement the gen one (RBY, not stadium) algorithm for determining critical
    /// hits. As such, it is inherently random.
    pub fn is_critical(&self, rand: &mut impl Rng, attacker: &BattleState) -> bool {
        // Not implemented: dire hit/focus energy since they are bugged anyway.
        let base_speed = attacker.pokemon().base_stats[usize::from(Stat::Speed)];
        let mut t = base_speed / 2;
        if self.effect == MoveEffect::HighCrit {
            t *= 4;
        }
        let t = t.min(255) as u8;
        let r: u8 = rand.gen();

        r < t
    }

    pub fn get_recoil(&self, damage: u16) -> Option<u16> {
        match self.effect {
            MoveEffect::Recoil(c) => Some((damage / u16::from(c.get())).max(1)),
            _ => None
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::rngs::mock::StepRng;

    use crate::pokemon::Pokemon;

    use super::*;

    #[test]
    fn test_hit_calc() {
        let m = Move::fallback();
        let mut rng = StepRng::new(254, 1);
        // First one struggle should hit.
        assert_eq!(true, m.hits(&mut rng, Modifier::default(), Modifier::default()));
        // Second should be affected by the 1/256 glitch.
        assert_eq!(
            false,
            m.hits(&mut rng, Modifier::default(), Modifier::default())
        );

        // Test how many hits we get for all possible random numbers
        let hits = (0..=0xff)
            .filter(|_| m.hits(&mut rng, Modifier::from(-3), Modifier::from(1)))
            .count();

        // Should hit 255 * (2 / 5) / (3 / 2) = 68 times
        assert_eq!(68, hits);
    }

    #[test]
    fn is_crit() {
        let m = Move::fallback();
        let mut rng = StepRng::new(0, 1);
        let mew = Pokemon {
            base_stats: [100; 5],
            ..Default::default()
        };
        let state = BattleState::new(&mew);

        let hits = (0..=0xff)
            .filter(|_| m.is_critical(&mut rng, &state))
            .count();
        // Mew has a probability of 50/256 to hit a critical, so…
        assert_eq!(50, hits);
    }
}
