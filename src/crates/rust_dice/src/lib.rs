/** Copyright (C) 2019 MouseProducedGames

See license in the LICENSE file

Documentation:

 **/
// External includes.
extern crate rand;

// Standard includes.

// Internal includes.
pub mod die;
pub mod roll;
pub mod roll_set;
#[allow(unused_imports)]
pub use die::Die;
pub use roll::Roll;
#[allow(unused_imports)]
pub use roll_set::RollSet;

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
    use super::die::Die;
    use super::roll::Roll;
    #[allow(unused_imports)]
    use super::roll_set::RollSet;

    #[test]
    fn it_works() {
        // assert_eq!(2 + 2, 4);
        for _ in 0..10000 {
            assert_eq!(RollSet::new(3, Die::new(6), 2).roll().total() >= 5, true);
        }
    }
}
