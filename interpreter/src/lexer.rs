//! Лексический анализатор (токенизатор).
//!
//! Преобразует исходный код JavaScript в последовательность токенов.
//! На данном этапе реализована базовая токенизация: числа, строки,
//! идентификаторы, операторы, ключевые слова и пунктуация.

use std::fmt;

/// Тип токена.
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    /// Числовой литерал.
    Number(f64),
    /// Строковый литерал.
    String(String),
    /// Идентификатор или ключевое слово.
    Identifier(String),
    /// Ключевое слово.
    Keyword(String),
    /// Оператор или пунктуация.
    Symbol(String),
    /// Конец файла.
    Eof,
}

/// Токен с позицией в исходном коде.
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    /// Тип токена.
    pub kind: TokenKind,
    /// Позиция (строка, колонка).
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            TokenKind::Number(n) => write!(f, "Number({})", n),
            TokenKind::String(s) => write!(f, "String({:?})", s),
            TokenKind::Identifier(id) => write!(f, "Identifier({})", id),
            TokenKind::Keyword(kw) => write!(f, "Keyword({})", kw),
            TokenKind::Symbol(op) => write!(f, "Symbol({})", op),
            TokenKind::Eof => write!(f, "Eof"),
        }
    }
}

/// Ошибка лексического анализа.
#[derive(Debug, Clone, PartialEq)]
pub struct LexError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (строка {}, колонка {})", self.message, self.line, self.column)
    }
}

/// Ключевые слова JavaScript (подмножество для Event Loop).
const KEYWORDS: &[&str] = &[
    "var", "let", "const", "function", "return", "if", "else", "for", "while",
    "do", "switch", "case", "default", "break", "continue", "new", "this",
    "true", "false", "null", "undefined", "typeof", "class", "extends",
    "async", "await", "try", "catch", "finally", "throw", "in", "of",
];

/// Лексический анализатор.
pub struct Lexer {
    source: Vec<char>,
    pos: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    /// Создаёт новый лексер из исходного кода.
    pub fn new(source: &str) -> Self {
        Lexer {
            source: source.chars().collect(),
            pos: 0,
            line: 1,
            column: 1,
        }
    }

    /// Токенизирует весь исходный код.
    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexError> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token()?;
            let is_eof = token.kind == TokenKind::Eof;
            tokens.push(token);
            if is_eof {
                break;
            }
        }
        Ok(tokens)
    }

    /// Возвращает следующий токен.
    fn next_token(&mut self) -> Result<Token, LexError> {
        self.skip_whitespace_and_comments();

        let line = self.line;
        let column = self.column;

        let Some(&ch) = self.peek() else {
            return Ok(Token { kind: TokenKind::Eof, line, column });
        };

        // Числа
        if ch.is_ascii_digit() {
            return self.read_number(line, column);
        }

        // Строки
        if ch == '"' || ch == '\'' || ch == '`' {
            return self.read_string(ch, line, column);
        }

        // Идентификаторы / ключевые слова
        if ch.is_alphabetic() || ch == '_' || ch == '$' {
            return self.read_identifier(line, column);
        }

        // Операторы и пунктуация
        self.read_operator(line, column)
    }

    fn skip_whitespace_and_comments(&mut self) {
        loop {
            match self.peek() {
                Some(&c) if c.is_whitespace() => {
                    self.advance();
                }
                // Однострочный комментарий //
                Some(&'/') if self.peek_n(1) == Some(&'/') => {
                    while let Some(&c) = self.peek() {
                        if c == '\n' {
                            break;
                        }
                        self.advance();
                    }
                }
                // Многострочный комментарий /* */
                Some(&'/') if self.peek_n(1) == Some(&'*') => {
                    self.advance();
                    self.advance();
                    while let Some(&c) = self.peek() {
                        if c == '*' && self.peek_n(1) == Some(&'/') {
                            self.advance();
                            self.advance();
                            break;
                        }
                        self.advance();
                    }
                }
                _ => break,
            }
        }
    }

    fn read_number(&mut self, line: usize, column: usize) -> Result<Token, LexError> {
        let mut num_str = String::new();
        while let Some(&c) = self.peek() {
            if c.is_ascii_digit() || c == '.' {
                num_str.push(c);
                self.advance();
            } else {
                break;
            }
        }
        let value: f64 = num_str.parse().map_err(|_| LexError {
            message: format!("Некорректное число: {}", num_str),
            line,
            column,
        })?;
        Ok(Token { kind: TokenKind::Number(value), line, column })
    }

    fn read_string(&mut self, quote: char, line: usize, column: usize) -> Result<Token, LexError> {
        self.advance(); // открывающая кавычка
        let mut s = String::new();
        loop {
            let Some(&c) = self.peek() else {
                return Err(LexError {
                    message: "Незакрытая строка".to_string(),
                    line,
                    column,
                });
            };
            if c == quote {
                self.advance();
                break;
            }
            if c == '\\' {
                self.advance();
                if let Some(&esc) = self.peek() {
                    match esc {
                        'n' => s.push('\n'),
                        't' => s.push('\t'),
                        '\\' => s.push('\\'),
                        '"' => s.push('"'),
                        '\'' => s.push('\''),
                        '`' => s.push('`'),
                        other => s.push(other),
                    }
                    self.advance();
                }
                continue;
            }
            s.push(c);
            self.advance();
        }
        Ok(Token { kind: TokenKind::String(s), line, column })
    }

    fn read_identifier(&mut self, line: usize, column: usize) -> Result<Token, LexError> {
        let mut ident = String::new();
        while let Some(&c) = self.peek() {
            if c.is_alphanumeric() || c == '_' || c == '$' {
                ident.push(c);
                self.advance();
            } else {
                break;
            }
        }
        let kind = if KEYWORDS.contains(&ident.as_str()) {
            TokenKind::Keyword(ident)
        } else {
            TokenKind::Identifier(ident)
        };
        Ok(Token { kind, line, column })
    }

    fn read_operator(&mut self, line: usize, column: usize) -> Result<Token, LexError> {
        // Многосимвольные операторы
        let two_char = [
            "===", "!==", "==", "!=", "<=", ">=", "&&", "||", "=>", "++", "--", "+=", "-=",
            "*=", "/=", "**", "??", "?.", "<<", ">>",
        ];
        for op in two_char {
            if self.starts_with(op) {
                for _ in op.chars() {
                    self.advance();
                }
                return Ok(Token { kind: TokenKind::Symbol(op.to_string()), line, column });
            }
        }

        // Односимвольные операторы
        let single = ['(', ')', '{', '}', '[', ']', ',', ';', ':', '.', '?', '+', '-', '*', '/', '%', '=', '<', '>', '!', '&', '|', '^', '~'];
        if let Some(&c) = self.peek() {
            if single.contains(&c) {
                self.advance();
                return Ok(Token { kind: TokenKind::Symbol(c.to_string()), line, column });
            }
        }

        Err(LexError {
            message: format!("Неизвестный символ: {:?}", self.peek()),
            line,
            column,
        })
    }

    fn peek(&self) -> Option<&char> {
        self.source.get(self.pos)
    }

    fn peek_n(&self, n: usize) -> Option<&char> {
        self.source.get(self.pos + n)
    }

    fn starts_with(&self, s: &str) -> bool {
        let chars: Vec<char> = s.chars().collect();
        for (i, c) in chars.iter().enumerate() {
            if self.source.get(self.pos + i) != Some(c) {
                return false;
            }
        }
        true
    }

    fn advance(&mut self) {
        if let Some(&c) = self.peek() {
            self.pos += 1;
            if c == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_basic() {
        let mut lexer = Lexer::new("let x = 42;");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 6);
        assert_eq!(tokens[0].kind, TokenKind::Keyword("let".to_string()));
        assert_eq!(tokens[1].kind, TokenKind::Identifier("x".to_string()));
        assert_eq!(tokens[2].kind, TokenKind::Symbol("=".to_string()));
        assert_eq!(tokens[3].kind, TokenKind::Number(42.0));
        assert_eq!(tokens[4].kind, TokenKind::Symbol(";".to_string()));
        assert_eq!(tokens[5].kind, TokenKind::Eof);
    }

    #[test]
    fn test_tokenize_string() {
        let mut lexer = Lexer::new("let s = \"hello\";");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[3].kind, TokenKind::String("hello".to_string()));
    }

    #[test]
    fn test_tokenize_comments() {
        let mut lexer = Lexer::new("// comment\nlet x = 1; /* block */");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].kind, TokenKind::Keyword("let".to_string()));
    }
}