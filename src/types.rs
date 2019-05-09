#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Type {
    Normal,
    Fighting,
    Flying,
    Poison,
    Ground,
    Rock,
    Bug,
    Ghost,
    Fire,
    Water,
    Grass,
    Electric,
    Psychic,
    Ice,
    Dragon,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Effectiveness {
    Neutral,
    Immune,
    Weak,
    Resist,
}

impl Type {
    /// Get the effectiveness of this type onto another.
    ///
    /// This function is an implementation of the table shown here: https://bulbapedia.bulbagarden.net/wiki/Type/Type_chart#Generation_I
    pub fn effectiveness(self, other: Type) -> Effectiveness {
        use self::Type::*;
        use self::Effectiveness::*;

        match self {
            Normal => match other {
                Rock => Resist,
                Ghost => Immune,
                _ => Neutral,
            }
            Fighting => match other {
                Normal | Rock | Ice => Weak,
                Flying | Poison | Bug | Psychic => Resist,
                Ghost => Immune,
                _ => Neutral,
            }
            Flying => match other {
                Fighting | Bug | Grass => Weak,
                Rock | Electric => Resist,
                _ => Neutral,
            }
            Poison => match other {
                Bug | Grass => Weak,
                Poison | Ground | Rock | Ghost => Resist,
                _ => Neutral,
            }
            Ground => match other {
                Poison | Rock | Fire | Electric => Weak,
                Bug | Grass => Resist,
                Flying => Immune,
                _ => Neutral,
            }
            Rock => match other {
                Flying | Bug | Fire | Ice => Weak,
                Fighting | Ground => Resist,
                _ => Neutral
            }
            Bug => match other {
                Poison | Grass | Psychic => Weak,
                Fighting | Flying | Ghost | Fire => Resist,
                _ => Neutral,
            }
            Ghost => match other {
                Ghost => Weak,
                // The infamous "ghosts can't hit psychic" bug
                Normal | Psychic => Immune,
                _ => Neutral,
            }
            Fire => match other {
                Bug | Grass | Ice => Weak,
                Rock | Fire | Water | Dragon => Resist,
                _ => Neutral,
            }
            Water => match other {
                Ground | Rock | Fire => Weak,
                Water | Grass | Dragon => Resist,
                _ => Neutral,
            }
            Grass => match other {
                Ground | Rock | Water => Weak,
                Flying | Poison | Ghost | Fire | Dragon => Resist,
                _ => Neutral,
            }
            Electric => match other {
                Flying | Water => Weak,
                Grass | Electric | Dragon => Resist,
                Ground => Immune,
                _ => Neutral,
            }
            Psychic => match other {
                Fighting | Poison => Weak,
                Psychic => Resist,
                _ => Neutral,
            }
            Ice => match other {
                Flying | Ground | Grass | Dragon => Weak,
                // Fire did not resist ice until gen 2.
                Water | Ice => Resist,
                _ => Neutral,
            }
            Dragon => if other == Dragon { Weak } else { Neutral },
        }
    }
}
