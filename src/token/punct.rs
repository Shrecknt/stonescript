use super::{stream::Stream, ParseError, ParseResult, Span, Token};
use lazy_static::lazy_static;
use std::{collections::BTreeMap, iter::FusedIterator};

type PunctCharTree = BTreeMap<char, PunctCharNode>;

#[derive(Debug)]
enum PunctCharNode {
    HeadedTree(PunctToken, PunctCharTree),
    HeadlessTree(BTreeMap<char, PunctCharNode>),
    Token(PunctToken),
}

macro_rules! punct_tree {
    ($map:expr, $variant:ident $char:literal) => {
        if let Some(prev_value) = $map.remove(&$char) {
            match prev_value {
                PunctCharNode::HeadedTree(token, _tree) => {
                    panic!("Token conflict between {:?} and {:?}", token, PunctToken::$variant);
                }
                PunctCharNode::HeadlessTree(tree) => {
                    $map.insert(
                        $char,
                        PunctCharNode::HeadedTree(PunctToken::$variant, tree)
                    );
                }
                PunctCharNode::Token(token) => {
                   panic!("Token conflict between {:?} and {:?}", token, PunctToken::$variant);
                }
            }
        } else {
            $map.insert($char, PunctCharNode::Token(PunctToken::$variant));
        }
    };
    ($map:expr, $variant:ident $char:literal $($tail:literal)+) => {
        if let Some(prev_value) = $map.remove(&$char) {
            match prev_value {
                PunctCharNode::HeadedTree(token, mut tree) => {
                    punct_tree!(tree, $variant $($tail)+);
                    $map.insert($char, PunctCharNode::HeadedTree(token, tree));
                }
                PunctCharNode::HeadlessTree(mut tree) => {
                    punct_tree!(tree, $variant $($tail)+);
                    $map.insert($char, PunctCharNode::HeadlessTree(tree));
                }
                PunctCharNode::Token(token) => {
                    $map.insert($char, PunctCharNode::HeadedTree(
                        token, {
                            let mut map = BTreeMap::new();
                            punct_tree!(map, $variant $($tail)+);
                            map
                        })
                    );
                }
            }
        } else {
            $map.insert($char, PunctCharNode::HeadlessTree({
                let mut map = BTreeMap::new();
                punct_tree!(map, $variant $($tail)+);
                map
            }));
        }
    };
}

macro_rules! define_punct {
    ($($variant:ident => $($char:literal)+),+) => {
        #[derive(Debug, Clone, Copy)]
        pub enum PunctToken {
            $($variant),+
        }

        lazy_static! {
            static ref PUNCT_TREE: PunctCharTree = {
                let mut map = BTreeMap::new();
                $(punct_tree!(map, $variant $($char)+);)+
                map
            };
        }
    }
}

define_punct!(
    Add => '+',
    Subtract => '-',
    Multiply => '*',
    Slash => '/',
    Modulo => '%',
    LessThan => '<',
    LessThanEquals => '<' '=',
    GreaterThan => '>',
    GreaterThanEquals => '>' '=',
    Assignment => '=',
    Not => '!',
    NotEquals => '!' '=',
    Property => '.',
    Ternary => '?',
    Colon => ':',
    Equals => '=' '=',
    And => '&' '&',
    Or => '|' '|',
    NullishCoalescing => '?' '?',
    Semicolon => ';',
    Comma => ',',
    Lambda => '-' '>',
    Selector => '@',
    Relative => '~',
    Comment => '#'
);

#[derive(Debug, Clone, Copy)]
pub struct Punct {
    pub span: Span,
    pub token: PunctToken,
}

struct PunctResolver<'a, 'b, T: FusedIterator<Item = char>> {
    stream: &'a mut Stream<'b, T>,
    start_pos: usize,
    depth: usize,
}

impl<'a, 'b, T: FusedIterator<Item = char>> PunctResolver<'a, 'b, T> {
    fn follow_tree(&mut self, tree: &PunctCharTree) -> ParseResult<Punct> {
        self.depth += 1;

        let char = self.stream.expect_next()?;
        if let Some(node) = tree.get(&char) {
            match node {
                PunctCharNode::HeadedTree(token, tree) => {
                    let peeked_char = self.stream.expect_peek()?;
                    if tree.contains_key(&peeked_char) {
                        self.follow_tree(tree)
                    } else {
                        Ok(Punct {
                            span: Span::new(self.start_pos, self.depth),
                            token: *token,
                        })
                    }
                }
                PunctCharNode::HeadlessTree(tree) => self.follow_tree(tree),
                PunctCharNode::Token(token) => Ok(Punct {
                    span: Span::new(self.start_pos, self.depth),
                    token: *token,
                }),
            }
        } else {
            Err(ParseError::UnexpectedToken(
                char.to_string(),
                "punct",
                Span::new(self.start_pos, self.depth),
            ))
        }
    }
}

impl<T: FusedIterator<Item = char>> Token<T> for Punct {
    fn parse(stream: &mut Stream<T>) -> ParseResult<Self> {
        PunctResolver {
            start_pos: stream.position,
            stream,
            depth: 0,
        }
        .follow_tree(&PUNCT_TREE)
    }

    fn valid_start(start: char) -> bool {
        PUNCT_TREE.contains_key(&start)
    }
}
