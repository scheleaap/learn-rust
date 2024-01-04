use std::fmt::Debug;

pub trait State {}


// #[derive(Debug)]
// struct Uninitialized;
//Playing
// #[derive(Debug)]
// struct Stopped;

#[derive(Debug)]
pub struct Playing<'a> {
    pub currentUri: &'a str,
}

impl State for Playing<'_> {}

#[derive(Debug)]
struct Paused;

impl State for Paused {}

pub trait Applicable: Debug {
    fn apply(&self) -> Box<dyn State>;
}

impl Applicable for Playing<'_> {
    fn apply(&self) -> Box<dyn State> {
        todo!()
    }
}

fn playing_apply(i: u32) -> impl State {
    if i > 0 {
        Playing { currentUri: "new uri" }
    } else {
        Paused
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn playing_none() {
        // Given
        let state_before = Playing { currentUri: "uri" };
        let expected_state_after = Paused;

        // When
        let actual_state_after = state_before.apply();

        // Then
        assert_eq!(actual_state_after, expected_state_after);
    }
}