pub mod tokenizer;

use std::{collections::HashMap, fmt};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NonTerminal {
    Sum,
    Sub,
    Mult,
    Atom,
    Number,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Terminal {
    Plus,
    Minus,
    Star,
    LeftParen,
    RightParen,
    Zero,
}

pub struct Grammar {
    pub starting_symbol: NonTerminal,
    pub rules: HashMap<NonTerminal, Vec<Vec<Expression>>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Terminal(Terminal),
    NonTerminal(NonTerminal),
}

#[derive(Debug, Clone)]
pub enum StackValue {
    Tree {
        head: NonTerminal,
        values: Vec<StackValue>,
    },
    Terminal(Terminal),
}

impl fmt::Display for NonTerminal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NonTerminal::Sum => write!(f, "sum"),
            NonTerminal::Sub => write!(f, "sub"),
            NonTerminal::Mult => write!(f, "mult"),
            NonTerminal::Atom => write!(f, "atom"),
            NonTerminal::Number => write!(f, "number"),
        }
    }
}

impl fmt::Display for Terminal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Terminal::Plus => write!(f, "+"),
            Terminal::Minus => write!(f, "-"),
            Terminal::Star => write!(f, "*"),
            Terminal::LeftParen => write!(f, "("),
            Terminal::RightParen => write!(f, ")"),
            Terminal::Zero => write!(f, "0"),
        }
    }
}

impl fmt::Display for Grammar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (non_terminal, productions) in &self.rules {
            for production in productions {
                write!(f, "{} -> ", non_terminal)?;
                for expr in production {
                    match expr {
                        Expression::Terminal(t) => write!(f, "'{}' ", t)?,
                        Expression::NonTerminal(nt) => write!(f, "{} ", nt)?,
                    }
                }
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

pub struct Parser {
    grammar: Grammar,
}

impl Parser {
    pub fn new(grammar: Grammar) -> Self {
        Parser { grammar }
    }

    pub fn parse(&self, input: &str) -> Result<(), String> {
        println!("\n==============\nParsing {input}");

        let mut tokens = input.split_whitespace().map(|val| match val {
            "+" => Terminal::Plus,
            "-" => Terminal::Minus,
            "*" => Terminal::Star,
            "(" => Terminal::LeftParen,
            ")" => Terminal::RightParen,
            "0" => Terminal::Zero,
            invalid => panic!("Invalid character: {invalid}"),
        });

        self.parse_expression(&mut tokens)
    }

    fn parse_expression<I>(&self, tokens: &mut I) -> Result<(), String>
    where
        I: Iterator<Item = Terminal> + Clone,
    {
        let items: HashMap<NonTerminal, Vec<Expression>> = self
            .grammar
            .rules
            .clone()
            .into_iter()
            .flat_map(|(rule_non_terminal, val)| {
                val.into_iter()
                    .map(move |rule| (rule_non_terminal.clone(), rule.clone()))
            })
            .collect();

        let mut stack: Vec<StackValue> = vec![];

        while let Some(token) = tokens.next() {
            let terminal = loop {
                let matching_non_terminals: Vec<(usize, NonTerminal)> = items
                    .iter()
                    .filter_map(|(nt, rhs)| {
                        if stack.len() < rhs.len() {
                            return None;
                        }

                        let comp: Vec<(&StackValue, &Expression)> =
                            Iterator::zip(stack.iter(), rhs.iter()).collect();

                        // println!(
                        //     "==\nChecking if the following comparison:\n {comp:#?} can be replaced with {nt}\n"
                        // );

                        if comp.iter().all(|(left, right)| match (left, right) {
                            (
                                StackValue::Tree { head, values: _ },
                                Expression::NonTerminal(non_terminal),
                            ) => head == non_terminal,
                            (StackValue::Terminal(left), Expression::Terminal(right)) => {
                                left == right
                            }
                            _ => false,
                        }) {
                            Some((rhs.len(), nt.clone()))
                        } else {
                            None
                        }
                    })
                    .collect();

                if matching_non_terminals.len() > 1 {
                    panic!(
                        "Ambiguous grammar, multiple applicable rewrites: {}",
                        matching_non_terminals
                            .into_iter()
                            .map(|(len, nt)| {
                                format!(
                                    "{nt} => {:?}",
                                    stack.get(stack.len().saturating_sub(len)..).expect(
                                        "Stack will at least have length \
                                        of right hand side of rewrite rule"
                                    )
                                )
                            })
                            .collect::<Vec<String>>()
                            .join(", ")
                    )
                }

                let Some((len, nt)) = matching_non_terminals.first() else {
                    break token;
                };

                let old = stack
                    .drain(stack.len().saturating_sub(*len)..)
                    .as_slice()
                    .to_owned();

                println!("Replacing stack values {old:?} with nonterminal {nt}");

                stack.push(StackValue::Tree {
                    head: nt.clone(),
                    values: old,
                });

                println!("Stack state: {:?}", stack)
            };

            println!("Adding terminal: {terminal}");

            stack.push(StackValue::Terminal(terminal));

            println!("Stack state: {:?}", stack);
        }

        if stack.len() == 1
            && let Some(StackValue::Terminal(_)) = stack.first()
        {
            Ok(())
        } else {
            Err(format!("Bad stack: {stack:#?}"))
        }
    }
}
