use crate::battle::Condition::Poisoned;
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
}
