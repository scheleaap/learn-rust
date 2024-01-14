use std::fmt::Debug;

// --------------- TODO: move to different file
#[derive(Debug, PartialEq)]
pub struct MopidyUri<'a>(&'a str);

#[derive(Debug, PartialEq)]
pub struct Album<'a> {
    spotify_uri: MopidyUri<'a>,
}

#[derive(Debug, PartialEq)]
pub enum Card<'a> {
    None,
    Album(Album<'a>),
}

// ---------------

#[derive(Debug, PartialEq)]
pub struct Paused<'a> {
    last_uri: MopidyUri<'a>,
    // TODO since
}

#[derive(Debug, PartialEq)]
pub struct Playing<'a> {
    last_uri: MopidyUri<'a>,
}

#[derive(Debug, PartialEq)]
pub enum State<'a> {
    Playing(Playing<'a>),
    Paused(Paused<'a>),
}

impl State<'_> {
    fn apply(self: Self, input) -> Self {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paused_none() {
        // Given
        let state_before = State::Paused(Paused { last_uri: MopidyUri("foo") });
        let input = Card::None;
        let expected_state_after = state_before;

        // When
        let actual_state_after: State = state_before.apply(input);

        // Then
        assert_eq!(actual_state_after, expected_state_after);
    }

    #[test]
    fn paused_album_() {
        // Given
        let state_before = State::Paused(Paused { last_uri: MopidyUri("foo") });
        let input = Card::Album(Album {spotify_uri: MopidyUri("foo")});
        let expected_state_after = state_before;

        // When
        let actual_state_after: State = state_before.apply(input);

        // Then
        assert_eq!(actual_state_after, expected_state_after);
    }
}