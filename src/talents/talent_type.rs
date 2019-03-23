/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/

// External includes.

// Internal includes.

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub enum TalentActivation {
    Passive(TalentActivationOp),
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub enum TalentActivationOp {
    EveryRound,
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub enum TalentType {
    ScanForSecrets(i32, TalentRange),
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub enum TalentRange {
    Radius(u32),
}
