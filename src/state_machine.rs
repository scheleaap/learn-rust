use std::fmt::Debug;

#[derive(Debug)]
#[derive(PartialEq)]
struct Paused {
    foo: u32,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Playing {
    foo: u32,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum State {
    Playing(Playing),
    Paused(Paused),
}


fn apply(old: State) -> State {
    old
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn playing_none() {
        // Given
        let state_before = State::Paused(Paused { foo: 1 });
        let expected_state_after = State::Paused(Paused { foo: 1 });

        // When
        let actual_state_after: State = apply(state_before);

        // Then
        assert_eq!(actual_state_after, expected_state_after);
    }
}