pub mod mastermind;

pub use crate::mastermind::{code::Code, code_match::CodeMatch, Colors};

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
        use Colors::*;

        let yellow = Yellow;
        let red = Red;

        let code = code::Code([yellow, Red, White, Green]);

        let equal = code::Code([Yellow, red, White, Green]);
        let unequal = code::Code([Green, Red, Purple, Blue]);

        assert_eq!(code, equal);
        assert_ne!(code, unequal);
    }

    #[test]
    fn create_matches() {
        use Colors::*;

        let code = code::Code([Yellow, Red, White, Green]);
        let guess = code::Code([Red, Yellow, White, Purple]);

        assert_ne!(code, guess);

        let guess_match = guess.match_code(&code);

        assert_eq!(
            guess_match,
            code_match::CodeMatch([
                MatchLevels::ExactMatch,
                MatchLevels::ColorMatch,
                MatchLevels::ColorMatch,
                MatchLevels::NoMatch
            ])
        );
    }

    #[test]
    fn random_code() {
        for _ in 0..80 {
            let random_code = code::Code::random();

            assert!(random_code.is_unique());
        }
    }

    #[test]
    fn all_colors() {
        let all_colors = Colors::all();
        for _ in 0..80 {
            let random_color: Colors = rand::random();
            assert!(all_colors.contains(&random_color));
        }
    }

    #[test]
    fn all_codes() {
        let all_codes = code::Code::all();
        for _ in 0..80 {
            let random_code = code::Code::random();
            assert!(all_codes.contains(&random_code));
        }
        assert!(all_codes.iter().all(|code| code.is_unique()));
        assert_eq!(all_codes.len(), 6 * 5 * 4 * 3);
    }

    #[test]
    fn solver() {
        use Colors::*;
        use MatchLevels::*;

        let true_code = code::Code([Red, Green, Purple, Blue]);

        let guesses = vec![
            solver::Guess((
                code::Code([Yellow, Red, White, Green]),
                code_match::CodeMatch([ColorMatch, ColorMatch, NoMatch, NoMatch]),
            )),
            solver::Guess((
                code::Code([Green, Yellow, Purple, Blue]),
                code_match::CodeMatch([ExactMatch, ExactMatch, ColorMatch, NoMatch]),
            )),
            solver::Guess((
                code::Code([Green, White, Red, Blue]),
                code_match::CodeMatch([ExactMatch, ColorMatch, ColorMatch, NoMatch]),
            )),
        ];

        let possible_codes = solver::possible_codes(&guesses);
        assert!(possible_codes.contains(&true_code));
    }

    #[test]
    fn solve_known() {
        use Colors::*;
        use MatchLevels::*;

        let guesses = vec![solver::Guess((
            code::Code([Red, White, Green, Blue]),
            code_match::CodeMatch([ExactMatch; 4]),
        ))];

        assert_eq!(
            solver::possible_codes(&guesses),
            vec![code::Code([Red, White, Green, Blue])]
        );
    }

    #[test]
    fn solve_unknown() {
        use Colors::*;
        use MatchLevels::*;

        assert!(!code::Code([Blue, White, Purple, Blue]).is_unique());

        let guesses = vec![
            solver::Guess((
                code::Code([Red, White, Green, Blue]),
                code_match::CodeMatch([ExactMatch, ColorMatch, NoMatch, NoMatch]),
            )),
            solver::Guess((
                code::Code([Blue, Green, Purple, Red]),
                code_match::CodeMatch([ExactMatch, ExactMatch, NoMatch, NoMatch]),
            )),
        ];

        assert_eq!(
            solver::possible_codes(&guesses),
            vec![
                code::Code([Blue, White, Purple, Yellow]),
                code::Code([Yellow, White, Purple, Red])
            ]
        );
    }
}
