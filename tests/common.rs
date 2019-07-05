use genwunsym::pokemon::Pokemon;
use genwunsym::stats::{PERFECT_EVS, PERFECT_IVS};
use genwunsym::types::Type;

pub const MEW: Pokemon = Pokemon {
    level: 100,
    base_stats: [100; 5],
    evs: PERFECT_EVS,
    ivs: PERFECT_IVS,
    types: [Some(Type::Psychic), None],
};

pub const MEWTWO: Pokemon = Pokemon {
    level: 100,
    base_stats: [106, 110, 90, 154, 130],
    evs: PERFECT_EVS,
    ivs: PERFECT_IVS,
    types: [Some(Type::Psychic), None],
};
