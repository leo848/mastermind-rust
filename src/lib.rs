pub mod mastermind;

#[cfg(test)]
mod tests {
    use super::mastermind::*;

    #[test]
    fn color_equality() {
        let blue = Colors::Blue;
        let green = Colors::Green;

        assert_ne!(blue, green);
        assert_eq!(green, Colors::Green);
    }

    #[test]
    fn code_as_arr() {
        let code = code::Code([Colors::Yellow, Colors::Red, Colors::White, Colors::Green]);

        assert_eq!(code.len(), 4);
        assert_eq!(code[2], Colors::White);
    }

    #[test]
    fn code_equality() {
        let yellow = Colors::Yellow;
        let red = Colors::Red;

        let code = code::Code([yellow, Colors::Red, Colors::White, Colors::Green]);

        let equal = code::Code([Colors::Yellow, red, Colors::White, Colors::Green]);
        let unequal = code::Code([Colors::Green, Colors::Red, Colors::Violet, Colors::Blue]);

        assert_eq!(code, equal);
        assert_ne!(code, unequal);
    }

    #[test]
    fn create_matches() {
        let code = code::Code([Colors::Yellow, Colors::Red, Colors::White, Colors::Green]);
        let guess = code::Code([Colors::Red, Colors::Yellow, Colors::White, Colors::Violet]);

        assert_ne!(code, guess);

        let guess_match = guess.match_code(&code);

        assert_eq!(guess_match, code_match::CodeMatch([MatchLevels::ExactMatch, MatchLevels::ColorMatch, MatchLevels::ColorMatch, MatchLevels::NoMatch]));
    }
}
