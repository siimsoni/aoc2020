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
    Whitespace,
    Unknown,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LiteralKind {
    Char,
    Integer,
}

pub struct Tokenizer {
    pub tokens: Vec<Token>,
    token_kind: Option<TokenKind>,
    token_len: usize,
}

impl Tokenizer {
    pub fn new() -> Self {
        Tokenizer {
            tokens: Vec::new(),
            token_kind: None,
            token_len: 1,
        }
    }

    pub fn tokenize(&mut self, iter: &mut std::slice::Iter<u8>) {
        for c in iter {
            let token_kind = match c {
                b':' => TokenKind::Colon,
                b'\n' => TokenKind::EndOfLine,
                b' ' => TokenKind::Whitespace,
                _ => {
                    if (c >= &b'a' && c <= &b'z') || (c >= &b'A' && c <= &b'Z') {
                        TokenKind::Literal(LiteralKind::Char)
                    } else if c >= &b'0' && c <= &b'9' {
                        TokenKind::Literal(LiteralKind::Integer)
                    } else {
                        TokenKind::Unknown
                    }
                }
            };
            match self.token_kind {
                None => {
                    self.token_kind = Some(token_kind);
                }
                Some(prev_token_kind) => {
                    if token_kind == prev_token_kind {
                        self.token_len += 1;
                    } else {
                        self.flush();
                        self.token_kind = Some(token_kind);
                        self.token_len = 1;
                    }
                }
            }
        }
    }

    pub fn flush(&mut self) {
        if let Some(token_kind) = self.token_kind {
            self.tokens.push(Token::new(token_kind, self.token_len));
        }
    }
}
