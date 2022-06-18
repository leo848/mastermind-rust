use super::*;

pub fn print_info() {
    print_items();

    println!("You can input a guess in the following form:");
    println!("<4 colors> <matches>\n");
    println!("Examples:");
    for _ in 0..5 {
        let random_code = code::Code::random();
        let code_match = code::Code::random().match_code(&random_code);
        println!(
            "{} {}",
            random_code.to_guess_string(),
            code_match.to_guess_string()
        );
    }
}

pub fn print_items() {
    print_colors();

    println!("The following match levels are available:");
    for match_level in MatchLevels::all() {
        println!(
            "{}: {} ({})",
            match_level.prettify(),
            match_level,
            match_level.to_char()
        );
    }
    println!();
}

pub fn print_colors() {
    println!("The following colors are available:");
    for color in Colors::all() {
        println!("{}: {} ({})", color.prettify(), color, color.to_char());
    }
    println!();
}
