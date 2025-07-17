use magician::prelude::*;
fn main() {
    init_magician();
    print_board(get_bishop_attacks(
        notation_to_index("a1"),
        blockers_from_squares(&["b2"]),
    ));
}

#[cfg(test)]
mod test {
    use crate::main;

    #[test]
    fn test_shit() {
        main();
    }
}
