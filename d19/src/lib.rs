extern crate btoi;
extern crate rustc_hash;
use btoi::btoi;
use rustc_hash::{FxHashMap, FxHashSet};
use std::io::BufRead;

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub len: usize,
}

impl Token {
    fn new(kind: TokenKind, len: usize) -> Token {
        Token { kind, len }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TokenKind {
    Colon,
    EndOfLine,
    Literal(LiteralKind),
    Pipe,
    Quote,
    Whitespace,
    Unknown,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LiteralKind {
    Char,
    Number,
}

struct Tokenizer {
    tokens: Vec<Token>,
    token_kind: Option<TokenKind>,
    token_len: usize,
}

impl Tokenizer {
    fn new() -> Self {
        Tokenizer {
            tokens: Vec::new(),
            token_kind: None,
            token_len: 1,
        }
    }

    fn tokenize(&mut self, iter: &mut std::slice::Iter<u8>) {
        for c in iter {
            let token_kind = match c {
                b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9' => {
                    TokenKind::Literal(LiteralKind::Number)
                }
                b'a' | b'b' => TokenKind::Literal(LiteralKind::Char),
                b':' => TokenKind::Colon,
                b'|' => TokenKind::Pipe,
                b' ' | b'"' => TokenKind::Whitespace,
                b'\n' => TokenKind::EndOfLine,
                _ => TokenKind::Unknown,
            };
            match self.token_kind {
                None => {
                    self.token_kind = Some(token_kind);
                }
                Some(prev_token_kind) => {
                    if token_kind == prev_token_kind {
                        self.token_len += 1;
                    } else {
                        self.tokens
                            .push(Token::new(self.token_kind.unwrap(), self.token_len));
                        self.token_kind = Some(token_kind);
                        self.token_len = 1;
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct RuleDef {
    id: usize,
    rule: RuleKind,
}

impl RuleDef {
    fn unary(id: usize, rules: Box<[usize]>) -> Self {
        Self {
            id,
            rule: RuleKind::Unary(UnaryRuleSequence { rules }),
        }
    }

    fn binary(id: usize, left: Box<[usize]>, right: Box<[usize]>) -> Self {
        Self {
            id,
            rule: RuleKind::Binary(BinaryRuleSequence { left, right }),
        }
    }

    fn concrete(id: usize, char: u8) -> Self {
        Self {
            id,
            rule: RuleKind::Concrete(char),
        }
    }
}

#[derive(Debug)]
pub enum RuleKind {
    Unary(UnaryRuleSequence),
    Binary(BinaryRuleSequence),
    Concrete(u8),
}

#[derive(Debug)]
pub struct UnaryRuleSequence {
    rules: Box<[usize]>,
}

#[derive(Debug)]
pub struct BinaryRuleSequence {
    left: Box<[usize]>,
    right: Box<[usize]>,
}

#[derive(Debug)]
pub struct ConcreteRule {
    value: u8,
}

pub struct RuleParser {
    state: RuleParserState,
    rules: Vec<RuleDef>,
    lhs: Vec<usize>,
    rhs: Vec<usize>,
    identifier: usize,
}

#[derive(Debug)]
pub enum RuleParserState {
    Label,
    Lhs,
    Rhs,
}

impl RuleParser {
    fn new() -> Self {
        Self {
            state: RuleParserState::Label,
            rules: Vec::new(),
            identifier: 0,
            lhs: Vec::new(),
            rhs: Vec::new(),
        }
    }

    fn parse(&mut self, input: &[u8], token: &Token) -> bool {
        match self.state {
            RuleParserState::Label => match token.kind {
                TokenKind::Literal(LiteralKind::Number) => {
                    self.identifier = btoi(input).unwrap();
                }
                TokenKind::Colon => {
                    self.state = RuleParserState::Lhs;
                }
                TokenKind::EndOfLine => {
                    return false;
                }
                TokenKind::Whitespace => (),
                _ => panic!("unexpected format"),
            },
            RuleParserState::Lhs => match token.kind {
                TokenKind::Literal(LiteralKind::Number) => {
                    self.lhs.push(btoi(input).unwrap());
                }
                TokenKind::Pipe => {
                    self.state = RuleParserState::Rhs;
                }
                TokenKind::Literal(LiteralKind::Char) => {
                    self.rules
                        .push(RuleDef::concrete(self.identifier, input[0]));
                    self.identifier = 0;
                }
                TokenKind::EndOfLine => {
                    if !self.lhs.is_empty() {
                        self.rules.push(RuleDef::unary(
                            self.identifier,
                            self.lhs.clone().into_boxed_slice(),
                        ));
                        self.lhs.clear();
                        self.identifier = 0;
                    }
                    if token.len == 1 {
                        self.state = RuleParserState::Label;
                    } else {
                        return false;
                    }
                }
                TokenKind::Whitespace => (),
                _ => panic!("unexpected format"),
            },
            RuleParserState::Rhs => match token.kind {
                TokenKind::Literal(LiteralKind::Number) => {
                    self.rhs.push(btoi(input).unwrap());
                }
                TokenKind::EndOfLine => {
                    self.rules.push(RuleDef::binary(
                        self.identifier,
                        self.lhs.clone().into_boxed_slice(),
                        self.rhs.clone().into_boxed_slice(),
                    ));
                    self.lhs.clear();
                    self.rhs.clear();
                    if token.len == 1 {
                        self.state = RuleParserState::Label;
                    } else {
                        return false;
                    }
                }
                TokenKind::Whitespace => (),
                _ => panic!("unexpected format"),
            },
        }
        true
    }
}

type ParseResult = (Box<[RuleDef]>, Box<[Box<[u8]>]>);

pub fn parse<R>(mut reader: R) -> ParseResult
where
    R: BufRead,
{
    let mut page: [u8; 4096] = [0; 4096];
    let mut input = Vec::new();
    let mut tokenizer = Tokenizer::new();
    while let Ok(page_len) = reader.read(&mut page) {
        if page_len == 0 {
            break;
        }
        tokenizer.tokenize(&mut page[..page_len].iter());
        input.extend_from_slice(&page[..page_len]);
    }
    tokenizer.tokenize(&mut b"\n".iter());

    let mut rule_parser = RuleParser::new();

    let mut pos = 0;
    let mut iter = tokenizer.tokens.iter();
    for token in &mut iter {
        if rule_parser.parse(&input[pos..pos + token.len], token) {
            pos += token.len;
        } else {
            pos += token.len;
            break;
        }
    }

    let mut messages = Vec::new();
    for token in &mut iter {
        if let TokenKind::Literal(LiteralKind::Char) = token.kind {
            messages.push(Box::<[u8]>::from(&input[pos..pos + token.len]));
        }
        pos += token.len;
    }

    (
        rule_parser.rules.into_boxed_slice(),
        messages.into_boxed_slice(),
    )
}

fn p1_validate(message: &[u8], rule_map: &FxHashMap<usize, &RuleKind>, rule: &RuleKind) -> usize {
    match rule {
        RuleKind::Concrete(value) => {
            if !message.is_empty() && message[0] == *value {
                1
            } else {
                0
            }
        }
        RuleKind::Unary(UnaryRuleSequence { rules }) => {
            let mut pos = 0;

            if rules.len() > message.len() {
                return pos;
            }

            for rule in rules.iter() {
                let valid = p1_validate(&message[pos..], rule_map, &rule_map[&rule]);
                if valid == 0 {
                    return 0;
                } else {
                    pos += valid;
                }
            }

            pos
        }
        RuleKind::Binary(BinaryRuleSequence { left, right }) => {
            let mut pos = 0;

            if message.len() >= left.len() {
                for rule in left.iter() {
                    let valid = p1_validate(&message[pos..], rule_map, &rule_map[&rule]);
                    if valid == 0 {
                        pos = 0;
                        break;
                    } else {
                        pos += valid;
                    }
                }
            }

            if pos != 0 || message.len() < right.len() {
                return pos;
            }

            for rule in right.iter() {
                let valid = p1_validate(&message[pos..], rule_map, &rule_map[&rule]);
                if valid == 0 {
                    return 0;
                } else {
                    pos += valid;
                }
            }

            pos
        }
    }
}

pub fn p1_solve((rule_defs, messages): &ParseResult) -> Option<usize> {
    let mut map = FxHashMap::default();
    for rule_def in rule_defs.iter() {
        map.insert(rule_def.id, &rule_def.rule);
    }
    Some(
        messages
            .iter()
            .filter(|message| p1_validate(message, &map, map[&0]) == message.len())
            .count(),
    )
}

fn p2_validate(
    message: &[u8],
    rule_map: &FxHashMap<usize, &RuleKind>,
    rule: &RuleKind,
) -> FxHashSet<usize> {
    match rule {
        RuleKind::Concrete(value) => {
            let mut result = FxHashSet::default();
            if !message.is_empty() && message[0] == *value {
                result.insert(1);
            }
            result
        }
        RuleKind::Unary(UnaryRuleSequence { rules }) => {
            let mut pos: FxHashSet<usize> = FxHashSet::default();
            pos.insert(0);
            for rule in rules.iter() {
                pos = pos
                    .iter()
                    .cloned()
                    .flat_map(|p| {
                        p2_validate(&message[p..], rule_map, &rule_map[&rule])
                            .iter()
                            .map(move |v| v + p)
                            .collect::<FxHashSet<usize>>()
                    })
                    .collect();
            }
            pos
        }
        RuleKind::Binary(BinaryRuleSequence { left, right }) => {
            let mut result = FxHashSet::default();
            let mut pos_left: FxHashSet<usize> = FxHashSet::default();
            pos_left.insert(0);
            for rule in left.iter() {
                pos_left = pos_left
                    .iter()
                    .cloned()
                    .flat_map(|p| {
                        p2_validate(&message[p..], rule_map, &rule_map[&rule])
                            .iter()
                            .map(move |v| v + p)
                            .collect::<FxHashSet<usize>>()
                    })
                    .collect();
            }

            result.extend(pos_left);

            let mut pos_right: FxHashSet<usize> = FxHashSet::default();
            pos_right.insert(0);
            for rule in right.iter() {
                pos_right = pos_right
                    .iter()
                    .cloned()
                    .flat_map(|p| {
                        p2_validate(&message[p..], rule_map, &rule_map[&rule])
                            .iter()
                            .map(move |v| v + p)
                            .collect::<FxHashSet<usize>>()
                    })
                    .collect();
            }
            result.extend(pos_right);
            result
        }
    }
}

pub fn p2_solve((rule_defs, messages): &ParseResult) -> Option<usize> {
    let mut map = FxHashMap::default();

    let swap_8 = RuleKind::Binary(BinaryRuleSequence {
        left: Box::from([42]),
        right: Box::from([42, 8]),
    });
    let swap_11 = RuleKind::Binary(BinaryRuleSequence {
        left: Box::from([42, 31]),
        right: Box::from([42, 11, 31]),
    });

    for rule_def in rule_defs.iter() {
        match rule_def.id {
            8 => map.insert(8, &swap_8),
            11 => map.insert(11, &swap_11),
            _ => map.insert(rule_def.id, &rule_def.rule),
        };
    }

    Some(
        messages
            .iter()
            .filter(|message| p2_validate(message, &map, map[&0]).contains(&message.len()))
            .count(),
    )
}
