use rand::rngs::OsRng;

use genwunsym::battle::BattleState;
use genwunsym::moves::Move;
use genwunsym::stats::Stat::{Accuracy, Evasion};

use crate::common::{MEW, MEWTWO};

mod common;

/// Simulate a battle between Mewtwo and Mew where both only use struggle.
///
/// Unless you have critical hits, Mewtwo wins or it is a draw. Draws are more likely.
///
/// With critical hits, mew should win sometimes, if it gets lucky.
#[test]
fn struggle_battle() {
    let mut rng = OsRng::new().unwrap();
    let struggle = Move::fallback();

    let mut mew = BattleState::new(&MEW);
    let mut mewtwo = BattleState::new(&MEWTWO);

    // TODO: add crits.

    while mew.is_alive() && mewtwo.is_alive() {
        // Mewtwo is faster, so move it first.
        println!("Mewtwo used struggle.");
        if struggle.hits(&mut rng, mewtwo.get_modifier(Accuracy), mew.get_modifier(Evasion)) {
            let damage = struggle.damage(&mut rng, &mewtwo, &mew);

            mew.damage(damage);
            println!("Mew took {} damage", damage);

            if let Some(recoil) = struggle.get_recoil(damage) {
                println!("Mewtwo took {} damage in recoil", recoil);
                mewtwo.damage(recoil);
            }
        } else {
            println!("Mewtwo missed!");
        }

        if !mew.is_alive() || !mewtwo.is_alive() {
            break;
        }

        // Mewtwo is faster, so move it first.
        println!("Mew used struggle.");
        if struggle.hits(&mut rng, mew.get_modifier(Accuracy), mewtwo.get_modifier(Evasion)) {
            let damage = struggle.damage(&mut rng, &mew, &mewtwo);

            mewtwo.damage(damage);
            println!("Mewtwo took {} damage", damage);

            if let Some(recoil) = struggle.get_recoil(damage) {
                println!("Mew took {} damage in recoil", recoil);
                mew.damage(recoil);
            }
        } else {
            println!("Mew missed!");
        }
    }

    // Check who's won
    if mewtwo.is_alive() {
        println!("Mewtwo won!")
    } else if mew.is_alive() {
        println!("Mew won!")
    } else {
        println!("It's a draw!")
    }
}
