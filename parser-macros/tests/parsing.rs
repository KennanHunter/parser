use parser_macros::{Expression, Grammar, NonTerminal, Parser, Terminal};
use std::collections::HashMap;

#[test]
fn tests() {
    let mut rules = HashMap::new();

    // Sum rules
    rules.insert(
        NonTerminal::Sum,
        vec![vec![
            Expression::NonTerminal(NonTerminal::Sum),
            Expression::Terminal(Terminal::Plus),
            Expression::NonTerminal(NonTerminal::Sub),
        ]],
    );

    // Sub rules
    rules.insert(
        NonTerminal::Sub,
        vec![vec![
            Expression::NonTerminal(NonTerminal::Sub),
            Expression::Terminal(Terminal::Minus),
            Expression::NonTerminal(NonTerminal::Mult),
        ]],
    );

    // Mult rules
    rules.insert(
        NonTerminal::Mult,
        vec![vec![
            Expression::NonTerminal(NonTerminal::Mult),
            Expression::Terminal(Terminal::Star),
            Expression::NonTerminal(NonTerminal::Atom),
        ]],
    );

    // Atom rules
    rules.insert(
        NonTerminal::Atom,
        vec![
            vec![
                Expression::Terminal(Terminal::LeftParen),
                Expression::NonTerminal(NonTerminal::Sum),
                Expression::Terminal(Terminal::RightParen),
            ],
            vec![Expression::NonTerminal(NonTerminal::Number)],
        ],
    );

    // Number rules
    rules.insert(
        NonTerminal::Number,
        vec![vec![Expression::Terminal(Terminal::Zero)]],
    );

    let g = Grammar {
        starting_symbol: NonTerminal::Sum,
        rules,
    };

    println!("{}", g);

    let parser = Parser::new(g);

    parser.parse("0 + 0 * 0").expect("Should be able to parse");

    parser.parse("0 * 0 + 0").expect("Should be able to parse");

    parser.parse("( 0 * 0 )").expect("Should be able to parse");
}
