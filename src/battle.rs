use std::ops::Index;

use crate::pokemon::Pokemon;
use crate::stats::{Modifier, Stat, StatSet};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Condition {
    Burned,
    Paralyzed,
    Frozen,
    /// Parameter is turns remaining
    Asleep(u8),
    /// Boolean indicates if this is a bad poisoning.
    Poisoned(bool),
}

pub struct NonVolatileState<'a> {
    pokemon: &'a Pokemon,
    hit_points: u16,
    condition: Option<Condition>,
}

impl<'a> NonVolatileState<'a> {
    pub fn new(pokemon: &'a Pokemon) -> Self {
        NonVolatileState {
            pokemon,
            hit_points: pokemon.get_stat(Stat::HP),
            condition: None,
        }
    }
}

pub struct BattleState<'a> {
    nv_state: NonVolatileState<'a>,
    stats: StatSet,
    modifiers: [Modifier; 7],
}

impl<'a> BattleState<'a> {
    pub fn new(pokemon: &'a Pokemon) -> Self {
        let nv_state = NonVolatileState::new(pokemon);
        BattleState::restore(nv_state)
    }

    pub fn restore(mut nv_state: NonVolatileState<'a>) -> Self {
        // Turn bad poison into poison.
        nv_state.condition = match nv_state.condition {
            Some(Condition::Poisoned(true)) => Some(Condition::Poisoned(false)),
            x => x,
        };

        // Recompute all stats.
        let stats = [
            nv_state.pokemon.get_stat(Stat::HP),
            nv_state.pokemon.get_stat(Stat::Attack),
            nv_state.pokemon.get_stat(Stat::Defense),
            nv_state.pokemon.get_stat(Stat::Special),
            nv_state.pokemon.get_stat(Stat::Speed),
        ];

        BattleState {
            nv_state,
            stats,
            modifiers: Default::default(),
        }
    }

    pub fn is_alive(&self) -> bool {
        self.nv_state.hit_points > 0
    }

    pub fn damage(&mut self, damage: u16) -> u16 {
        self.nv_state.hit_points = self.nv_state.hit_points.saturating_sub(damage);
        self.nv_state.hit_points
    }

    pub fn get_modifier(&self, stat: Stat) -> Modifier {
        self.modifiers[usize::from(stat)]
    }

    pub fn get_modifier_mut(&mut self, stat: Stat) -> &mut Modifier {
        &mut self.modifiers[usize::from(stat)]
    }

    pub fn pokemon(&self) -> &Pokemon {
        self.nv_state.pokemon
    }
}

impl<'a> Index<Stat> for BattleState<'a> {
    type Output = u16;

    fn index(&self, index: Stat) -> &Self::Output {
        match index {
            Stat::Accuracy | Stat::Evasion => unimplemented!(),
            x => &self.stats[usize::from(x)],
        }
    }
}
