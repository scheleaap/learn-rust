use std::fmt::Debug;

pub trait State {}

#[derive(Debug)]
pub struct Playing;

impl State for Playing {}

#[derive(Debug)]
struct Paused;

impl State for Paused {}

pub trait Applicable: Debug {
    fn apply(&self) -> Box<dyn State>;
}

impl Applicable for Playing {
    fn apply(&self) -> Box<dyn State> {
        if 0 == 0 {
            Box::new(Playing)
        } else {
            Box::new(Paused)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn playing_none() {
        // Given
        let state_before = Playing;
        let expected_state_after = Paused;

        // When
        let actual_state_after = state_before.apply();

        // Then
        let x: Playing = actual_state_after.into();
        assert_eq!(x, expected_state_after);
    }
}