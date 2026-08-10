//! Синтаксический анализатор (recursive descent).
//!
//! Преобразует токены (из [`lexer`]) в AST (из [`ast`]).
//! На данном этапе реализован базовый парсер, покрывающий
//! конструкции, необходимые для демонстрации Event Loop.

use crate::ast::{Expr, Program, Stmt, VarKind};
use crate::lexer::{LexError, Lexer, Token, TokenKind};

/// Ошибка синтаксического анализа.
#[derive(Debug, Clone, PartialEq)]
pub struct ParseError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (строка {}, колонка {})", self.message, self.line, self.column)
    }
}

impl From<LexError> for ParseError {
    fn from(e: LexError) -> Self {
        ParseError { message: e.message, line: e.line, column: e.column }
    }
}

/// Парсер.
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    /// Создаёт парсер из исходного кода.
    pub fn new(source: &str) -> Result<Self, ParseError> {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize()?;
        Ok(Parser { tokens, pos: 0 })
    }

    /// Разбирает программу.
    pub fn parse_program(&mut self) -> Result<Program, ParseError> {
        let mut statements = Vec::new();
        while !self.at_end() {
            let stmt = self.parse_statement()?;
            statements.push(stmt);
        }
        Ok(Program { statements })
    }

    // --- Вспомогательные методы ---

    fn current(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.pos.saturating_sub(1)]
    }

    fn at_end(&self) -> bool {
        self.current().kind == TokenKind::Eof
    }

    fn advance(&mut self) -> Token {
        let token = self.tokens[self.pos].clone();
        if !self.at_end() {
            self.pos += 1;
        }
        token
    }

    fn check(&self, kind: &TokenKind) -> bool {
        &self.current().kind == kind
    }

    fn match_symbol(&mut self, symbol: &str) -> bool {
        if let TokenKind::Symbol(s) = &self.current().kind {
            if s == symbol {
                self.advance();
                return true;
            }
        }
        false
    }

    fn match_keyword(&mut self, keyword: &str) -> bool {
        if let TokenKind::Keyword(k) = &self.current().kind {
            if k == keyword {
                self.advance();
                return true;
            }
        }
        false
    }

    fn expect_symbol(&mut self, symbol: &str) -> Result<Token, ParseError> {
        if self.match_symbol(symbol) {
            Ok(self.previous().clone())
        } else {
            Err(self.error(&format!("Ожидался символ '{}'", symbol)))
        }
    }

    fn expect_identifier(&mut self) -> Result<String, ParseError> {
        if let TokenKind::Identifier(name) = &self.current().kind {
            let name = name.clone();
            self.advance();
            Ok(name)
        } else {
            Err(self.error("Ожидался идентификатор"))
        }
    }

    fn expect_keyword(&mut self, keyword: &str) -> Result<Token, ParseError> {
        if self.match_keyword(keyword) {
            Ok(self.previous().clone())
        } else {
            Err(self.error(&format!("Ожидалось ключевое слово '{}'", keyword)))
        }
    }

    fn error(&self, message: &str) -> ParseError {
        let token = self.current();
        ParseError {
            message: message.to_string(),
            line: token.line,
            column: token.column,
        }
    }

    // --- Операторы (statements) ---

    fn parse_statement(&mut self) -> Result<Stmt, ParseError> {
        // Объявление переменной
        if self.check(&TokenKind::Keyword("let".to_string()))
            || self.check(&TokenKind::Keyword("const".to_string()))
            || self.check(&TokenKind::Keyword("var".to_string()))
        {
            return self.parse_var_decl();
        }

        // Объявление функции
        if self.check(&TokenKind::Keyword("function".to_string())) {
            return self.parse_function_decl();
        }
        // async function
        if self.check(&TokenKind::Keyword("async".to_string())) {
            return self.parse_async_function_decl();
        }

        // Управляющие конструкции
        if self.check(&TokenKind::Keyword("if".to_string())) {
            return self.parse_if();
        }
        if self.check(&TokenKind::Keyword("while".to_string())) {
            return self.parse_while();
        }
        if self.check(&TokenKind::Keyword("for".to_string())) {
            return self.parse_for();
        }
        if self.check(&TokenKind::Keyword("return".to_string())) {
            return self.parse_return();
        }
        if self.check(&TokenKind::Keyword("break".to_string())) {
            self.advance();
            let line = self.previous().line;
            self.expect_symbol(";")?;
            return Ok(Stmt::Break { line });
        }
        if self.check(&TokenKind::Keyword("continue".to_string())) {
            self.advance();
            let line = self.previous().line;
            self.expect_symbol(";")?;
            return Ok(Stmt::Continue { line });
        }
        if self.check(&TokenKind::Keyword("switch".to_string())) {
            return self.parse_switch();
        }
        if self.check(&TokenKind::Keyword("class".to_string())) {
            return self.parse_class();
        }
        if self.check(&TokenKind::Keyword("try".to_string())) {
            return self.parse_try();
        }
        if self.check(&TokenKind::Keyword("throw".to_string())) {
            return self.parse_throw();
        }

        // Блок
        if self.match_symbol("{") {
            return self.parse_block();
        }

        // Выражение-оператор
        let expr = self.parse_expression()?;
        let line = self.previous().line;
        self.match_symbol(";");
        Ok(Stmt::Expr { expr, line })
    }

    fn parse_var_decl(&mut self) -> Result<Stmt, ParseError> {
        let kind = match &self.current().kind {
            TokenKind::Keyword(k) if k == "let" => VarKind::Let,
            TokenKind::Keyword(k) if k == "const" => VarKind::Const,
            TokenKind::Keyword(k) if k == "var" => VarKind::Var,
            _ => return Err(self.error("Ожидалось объявление переменной")),
        };
        self.advance();
        let line = self.previous().line;

        let name = self.expect_identifier()?;
        let init = if self.match_symbol("=") {
            Some(self.parse_expression()?)
        } else {
            None
        };
        self.match_symbol(";");
        Ok(Stmt::VarDecl { name, init, kind, line })
    }

    fn parse_function_decl(&mut self) -> Result<Stmt, ParseError> {
        self.advance(); // function
        let line = self.previous().line;
        let name = self.expect_identifier()?;
        let (params, body) = self.parse_function_body()?;
        Ok(Stmt::FunctionDecl { name, params, body: Box::new(body), is_async: false, line })
    }

    fn parse_async_function_decl(&mut self) -> Result<Stmt, ParseError> {
        self.advance(); // async
        let line = self.previous().line;
        self.expect_keyword("function")?;
        let name = self.expect_identifier()?;
        let (params, body) = self.parse_function_body()?;
        // Имя сохраняется без префикса, чтобы вызов `name()` находил функцию.
        // Async-статус хранится в поле `is_async`.
        Ok(Stmt::FunctionDecl { name, params, body: Box::new(body), is_async: true, line })
    }

    fn parse_function_body(&mut self) -> Result<(Vec<String>, Stmt), ParseError> {
        self.expect_symbol("(")?;
        let mut params = Vec::new();
        if !self.check(&TokenKind::Symbol(")".to_string())) {
            loop {
                params.push(self.expect_identifier()?);
                if !self.match_symbol(",") {
                    break;
                }
            }
        }
        self.expect_symbol(")")?;
        self.expect_symbol("{")?;
        let body = self.parse_block()?;
        Ok((params, body))
    }

    fn parse_block(&mut self) -> Result<Stmt, ParseError> {
        let mut statements = Vec::new();
        while !self.check(&TokenKind::Symbol("}".to_string())) && !self.at_end() {
            statements.push(self.parse_statement()?);
        }
        // Номер строки блока — строка закрывающей `}`. Это позволяет
        // `line_range()` покрывать весь блок целиком (от первого оператора
        // до закрывающей скобки).
        self.expect_symbol("}")?;
        let line = self.previous().line;
        Ok(Stmt::Block { statements, line })
    }

    fn parse_if(&mut self) -> Result<Stmt, ParseError> {
        self.advance(); // if
        let line = self.previous().line;
        self.expect_symbol("(")?;
        let cond = self.parse_expression()?;
        self.expect_symbol(")")?;
        let then_branch = Box::new(self.parse_statement()?);
        let else_branch = if self.match_keyword("else") {
            Some(Box::new(self.parse_statement()?))
        } else {
            None
        };
        Ok(Stmt::If { cond, then_branch, else_branch, line })
    }

    fn parse_while(&mut self) -> Result<Stmt, ParseError> {
        self.advance(); // while
        let line = self.previous().line;
        self.expect_symbol("(")?;
        let cond = self.parse_expression()?;
        self.expect_symbol(")")?;
        let body = Box::new(self.parse_statement()?);
        Ok(Stmt::While { cond, body, line })
    }

    fn parse_for(&mut self) -> Result<Stmt, ParseError> {
        self.advance(); // for
        let line = self.previous().line;
        self.expect_symbol("(")?;

        let init = if self.match_symbol(";") {
            None
        } else {
            let stmt = self.parse_statement()?;
            Some(Box::new(stmt))
        };

        let cond = if self.match_symbol(";") {
            None
        } else {
            let expr = self.parse_expression()?;
            self.expect_symbol(";")?;
            Some(expr)
        };

        let update = if self.match_symbol(")") {
            None
        } else {
            let expr = self.parse_expression()?;
            self.expect_symbol(")")?;
            Some(expr)
        };

        let body = Box::new(self.parse_statement()?);
        Ok(Stmt::For { init, cond, update, body, line })
    }

    fn parse_return(&mut self) -> Result<Stmt, ParseError> {
        self.advance(); // return
        let line = self.previous().line;
        let expr = if self.check(&TokenKind::Symbol(";".to_string())) || self.at_end() {
            None
        } else {
            Some(self.parse_expression()?)
        };
        self.match_symbol(";");
        Ok(Stmt::Return { expr, line })
    }

    fn parse_switch(&mut self) -> Result<Stmt, ParseError> {
        self.advance(); // switch
        let line = self.previous().line;
        self.expect_symbol("(")?;
        let expr = self.parse_expression()?;
        self.expect_symbol(")")?;
        self.expect_symbol("{")?;

        let mut cases: Vec<(Expr, Vec<Stmt>)> = Vec::new();
        let mut default: Option<Vec<Stmt>> = None;

        while !self.check(&TokenKind::Symbol("}".to_string())) && !self.at_end() {
            if self.match_keyword("case") {
                let value = self.parse_expression()?;
                self.expect_symbol(":")?;
                let mut body = Vec::new();
                while !self.check(&TokenKind::Keyword("case".to_string()))
                    && !self.check(&TokenKind::Keyword("default".to_string()))
                    && !self.check(&TokenKind::Symbol("}".to_string()))
                    && !self.at_end()
                {
                    body.push(self.parse_statement()?);
                }
                cases.push((value, body));
            } else if self.match_keyword("default") {
                self.expect_symbol(":")?;
                let mut body = Vec::new();
                while !self.check(&TokenKind::Symbol("}".to_string())) && !self.at_end() {
                    body.push(self.parse_statement()?);
                }
                default = Some(body);
            } else {
                return Err(self.error("Ожидался 'case' или 'default' в switch"));
            }
        }
        self.expect_symbol("}")?;
        Ok(Stmt::Switch { expr, cases, default, line })
    }

    fn parse_class(&mut self) -> Result<Stmt, ParseError> {
        self.advance(); // class
        let line = self.previous().line;
        let name = self.expect_identifier()?;
        // Пропускаем необязательное `extends` (наследование пока не реализовано)
        if self.match_keyword("extends") {
            let _ = self.expect_identifier()?;
        }
        self.expect_symbol("{")?;

        let mut methods: Vec<(String, Vec<String>, Box<Stmt>)> = Vec::new();
        while !self.check(&TokenKind::Symbol("}".to_string())) && !self.at_end() {
            let method_name = self.expect_identifier()?;
            let (params, body) = self.parse_function_body()?;
            methods.push((method_name, params, Box::new(body)));
        }
        self.expect_symbol("}")?;
        Ok(Stmt::ClassDecl { name, methods, line })
    }

    /// Разбирает `try { body } catch (e) { catch_body } finally { finally_body }`.
    fn parse_try(&mut self) -> Result<Stmt, ParseError> {
        self.advance(); // try
        let line = self.previous().line;
        self.expect_symbol("{")?;
        let body = self.parse_block()?;

        // catch — необязательный
        let mut catch_param = None;
        let mut catch_body = None;
        if self.match_keyword("catch") {
            self.expect_symbol("(")?;
            catch_param = Some(self.expect_identifier()?);
            self.expect_symbol(")")?;
            self.expect_symbol("{")?;
            catch_body = Some(Box::new(self.parse_block()?));
        }

        // finally — необязательный
        let finally_body = if self.match_keyword("finally") {
            self.expect_symbol("{")?;
            Some(Box::new(self.parse_block()?))
        } else {
            None
        };

        Ok(Stmt::Try {
            body: Box::new(body),
            catch_param,
            catch_body,
            finally_body,
            line,
        })
    }

    /// Разбирает `throw expr;`.
    fn parse_throw(&mut self) -> Result<Stmt, ParseError> {
        self.advance(); // throw
        let line = self.previous().line;
        let expr = self.parse_expression()?;
        self.match_symbol(";");
        Ok(Stmt::Throw { expr, line })
    }

    // --- Выражения (expressions) ---

    fn parse_expression(&mut self) -> Result<Expr, ParseError> {
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> Result<Expr, ParseError> {
        let expr = self.parse_ternary()?;
        if self.match_symbol("=") {
            let value = self.parse_assignment()?;
            match expr {
                Expr::Identifier(name) => {
                    return Ok(Expr::Assign { name, value: Box::new(value) });
                }
                Expr::Member { object, property } => {
                    return Ok(Expr::SetMember {
                        object,
                        property,
                        value: Box::new(value),
                    });
                }
                _ => {
                    return Err(self.error("Левая часть присваивания должна быть идентификатором или свойством"));
                }
            }
        }
        Ok(expr)
    }

    fn parse_ternary(&mut self) -> Result<Expr, ParseError> {
        let cond = self.parse_logical_or()?;
        if self.match_symbol("?") {
            let then_expr = self.parse_expression()?;
            self.expect_symbol(":")?;
            let else_expr = self.parse_expression()?;
            return Ok(Expr::Ternary {
                cond: Box::new(cond),
                then_expr: Box::new(then_expr),
                else_expr: Box::new(else_expr),
            });
        }
        Ok(cond)
    }

    fn parse_logical_or(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_logical_and()?;
        while self.match_symbol("||") {
            let right = self.parse_logical_and()?;
            expr = Expr::Logical {
                left: Box::new(expr),
                op: "||".to_string(),
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn parse_logical_and(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_equality()?;
        while self.match_symbol("&&") {
            let right = self.parse_equality()?;
            expr = Expr::Logical {
                left: Box::new(expr),
                op: "&&".to_string(),
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn parse_equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_comparison()?;
        // Важно: проверяем трёхсимвольные операторы (===, !==) ПЕРЕД двухсимвольными (==, !=),
        // иначе `===` будет распознан как `==` + `=`.
        while self.match_symbol("===") || self.match_symbol("!==") || self.match_symbol("==") || self.match_symbol("!=") {
            let op = match &self.previous().kind {
                TokenKind::Symbol(s) => s.clone(),
                _ => unreachable!(),
            };
            let right = self.parse_comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn parse_comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_term()?;
        while self.match_symbol("<") || self.match_symbol(">") || self.match_symbol("<=") || self.match_symbol(">=") {
            let op = match &self.previous().kind {
                TokenKind::Symbol(s) => s.clone(),
                _ => unreachable!(),
            };
            let right = self.parse_term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn parse_term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_factor()?;
        while self.match_symbol("+") || self.match_symbol("-") {
            let op = match &self.previous().kind {
                TokenKind::Symbol(s) => s.clone(),
                _ => unreachable!(),
            };
            let right = self.parse_factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn parse_factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_unary()?;
        while self.match_symbol("*") || self.match_symbol("/") || self.match_symbol("%") {
            let op = match &self.previous().kind {
                TokenKind::Symbol(s) => s.clone(),
                _ => unreachable!(),
            };
            let right = self.parse_unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn parse_unary(&mut self) -> Result<Expr, ParseError> {
        if self.match_symbol("!") || self.match_symbol("-") || self.match_symbol("+") {
            let op = match &self.previous().kind {
                TokenKind::Symbol(s) => s.clone(),
                _ => unreachable!(),
            };
            let expr = self.parse_unary()?;
            return Ok(Expr::Unary { op, expr: Box::new(expr) });
        }
        if self.match_keyword("typeof") {
            let expr = self.parse_unary()?;
            return Ok(Expr::Typeof(Box::new(expr)));
        }
        if self.match_keyword("await") {
            let expr = self.parse_unary()?;
            return Ok(Expr::Await(Box::new(expr)));
        }
        if self.match_keyword("new") {
            // callee — первичное выражение (класс/конструктор), args — аргументы.
            let callee = self.parse_primary()?;
            let args = if self.match_symbol("(") {
                let mut args = Vec::new();
                if !self.check(&TokenKind::Symbol(")".to_string())) {
                    loop {
                        args.push(self.parse_expression()?);
                        if !self.match_symbol(",") {
                            break;
                        }
                    }
                }
                self.expect_symbol(")")?;
                args
            } else {
                Vec::new()
            };
            let mut expr = Expr::New { callee: Box::new(callee), args };
            // Обрабатываем постфиксные операции: obj.method(), obj.prop
            loop {
                if self.match_symbol("(") {
                    let mut call_args = Vec::new();
                    if !self.check(&TokenKind::Symbol(")".to_string())) {
                        loop {
                            call_args.push(self.parse_expression()?);
                            if !self.match_symbol(",") {
                                break;
                            }
                        }
                    }
                    self.expect_symbol(")")?;
                    expr = Expr::Call { callee: Box::new(expr), args: call_args };
                } else if self.match_symbol(".") {
                    let property = self.parse_property_name()?;
                    expr = Expr::Member { object: Box::new(expr), property: Box::new(property) };
                } else if self.match_symbol("[") {
                    let index = self.parse_expression()?;
                    self.expect_symbol("]")?;
                    expr = Expr::Member { object: Box::new(expr), property: Box::new(index) };
                } else {
                    break;
                }
            }
            return Ok(expr);
        }
        self.parse_postfix()
    }

    fn parse_postfix(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_primary()?;
        loop {
            if self.match_symbol("(") {
                // Вызов функции
                let mut args = Vec::new();
                if !self.check(&TokenKind::Symbol(")".to_string())) {
                    loop {
                        args.push(self.parse_expression()?);
                        if !self.match_symbol(",") {
                            break;
                        }
                    }
                }
                self.expect_symbol(")")?;
                expr = Expr::Call { callee: Box::new(expr), args };
            } else if self.match_symbol(".") {
                let property = self.parse_property_name()?;
                expr = Expr::Member { object: Box::new(expr), property: Box::new(property) };
            } else if self.match_symbol("[") {
                let index = self.parse_expression()?;
                self.expect_symbol("]")?;
                expr = Expr::Member { object: Box::new(expr), property: Box::new(index) };
            } else if self.match_symbol("++") {
                // Постфиксный инкремент: i++ → i = i + 1
                expr = self.build_inc_dec(expr, true)?;
            } else if self.match_symbol("--") {
                // Постфиксный декремент: i-- → i = i - 1
                expr = self.build_inc_dec(expr, false)?;
            } else {
                break;
            }
        }
        Ok(expr)
    }

    /// Строит выражение инкремента/декремента: `i++` → `i = i + 1`.
    fn build_inc_dec(&mut self, target: Expr, is_increment: bool) -> Result<Expr, ParseError> {
        let op = if is_increment { "+" } else { "-" };
        match target {
            Expr::Identifier(name) => {
                let name_clone = name.clone();
                Ok(Expr::Assign {
                    name,
                    value: Box::new(Expr::Binary {
                        left: Box::new(Expr::Identifier(name_clone)),
                        op: op.to_string(),
                        right: Box::new(Expr::Number(1.0)),
                    }),
                })
            }
            Expr::Member { object, property } => {
                let obj_clone = object.clone();
                let prop_clone = property.clone();
                Ok(Expr::SetMember {
                    object,
                    property,
                    value: Box::new(Expr::Binary {
                        left: Box::new(Expr::Member {
                            object: obj_clone,
                            property: prop_clone,
                        }),
                        op: op.to_string(),
                        right: Box::new(Expr::Number(1.0)),
                    }),
                })
            }
            _ => Err(self.error("Инкремент/декремент применим только к переменным или свойствам")),
        }
    }

    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        let token = self.current().clone();
        match token.kind {
            TokenKind::Number(n) => {
                self.advance();
                Ok(Expr::Number(n))
            }
            TokenKind::String(s) => {
                self.advance();
                Ok(Expr::String(s))
            }
            TokenKind::Identifier(name) => {
                // Проверяем, является ли это стрелочной функцией с одним параметром
                // без скобок: `ident => body`.
                if self.is_single_param_arrow_function() {
                    return self.parse_single_param_arrow_function();
                }
                self.advance();
                Ok(Expr::Identifier(name))
            }
            TokenKind::Keyword(kw) => match kw.as_str() {
                "true" => {
                    self.advance();
                    Ok(Expr::Boolean(true))
                }
                "false" => {
                    self.advance();
                    Ok(Expr::Boolean(false))
                }
                "null" => {
                    self.advance();
                    Ok(Expr::Null)
                }
                "undefined" => {
                    self.advance();
                    Ok(Expr::Undefined)
                }
                "this" => {
                    self.advance();
                    Ok(Expr::This)
                }
                "function" => {
                    self.advance();
                    let name = if let TokenKind::Identifier(n) = &self.current().kind {
                        let n = n.clone();
                        self.advance();
                        Some(n)
                    } else {
                        None
                    };
                    let (params, body) = self.parse_function_body()?;
                    Ok(Expr::Function { name, params, body: Box::new(body), is_arrow: false })
                }
                _ => Err(self.error("Неожиданное ключевое слово")),
            },
            TokenKind::Symbol(s) => match s.as_str() {
                "(" => {
                    // Проверяем, является ли это стрелочной функцией: (params) => body
                    if self.is_arrow_function() {
                        return self.parse_arrow_function();
                    }
                    self.advance();
                    let expr = self.parse_expression()?;
                    self.expect_symbol(")")?;
                    Ok(expr)
                }
                "{" => {
                    self.advance();
                    let mut properties = Vec::new();
                    while !self.check(&TokenKind::Symbol("}".to_string())) && !self.at_end() {
                        let key = self.expect_identifier()?;
                        self.expect_symbol(":")?;
                        let value = self.parse_expression()?;
                        properties.push((key, value));
                        if !self.match_symbol(",") {
                            break;
                        }
                    }
                    self.expect_symbol("}")?;
                    Ok(Expr::Object(properties))
                }
                "[" => {
                    self.advance();
                    let mut items = Vec::new();
                    while !self.check(&TokenKind::Symbol("]".to_string())) && !self.at_end() {
                        items.push(self.parse_expression()?);
                        if !self.match_symbol(",") {
                            break;
                        }
                    }
                    self.expect_symbol("]")?;
                    Ok(Expr::Array(items))
                }
                _ => Err(self.error("Unexpected token")),
            },
            TokenKind::Eof => Err(self.error("Неожиданный конец файла")),
        }
    }

    /// Разбирает имя свойства после точки: `obj.prop`.
    ///
    /// В отличие от `parse_primary`, позволяет использовать ключевые слова
    /// как имена свойств (например, `obj.catch`, `obj.then`, `obj.default`).
    fn parse_property_name(&mut self) -> Result<Expr, ParseError> {
        match &self.current().kind {
            TokenKind::Identifier(name) => {
                let name = name.clone();
                self.advance();
                Ok(Expr::Identifier(name))
            }
            TokenKind::Keyword(kw) => {
                // Ключевые слова могут быть именами свойств: obj.catch, obj.then
                let kw = kw.clone();
                self.advance();
                Ok(Expr::Identifier(kw))
            }
            _ => self.parse_primary(),
        }
    }

    /// Проверяет, является ли текущая конструкция стрелочной функцией.
    /// Ожидается, что текущий токен — `(`.
    fn is_arrow_function(&self) -> bool {
        // Ищем закрывающую скобку и проверяем, что после неё идёт `=>`
        let mut depth = 0;
        let mut i = self.pos;
        while i < self.tokens.len() {
            match &self.tokens[i].kind {
                TokenKind::Symbol(s) if s == "(" => depth += 1,
                TokenKind::Symbol(s) if s == ")" => {
                    depth -= 1;
                    if depth == 0 {
                        // Проверяем следующий токен
                        if let Some(next) = self.tokens.get(i + 1) {
                            return matches!(&next.kind, TokenKind::Symbol(s) if s == "=>");
                        }
                        return false;
                    }
                }
                TokenKind::Eof => return false,
                _ => {}
            }
            i += 1;
        }
        false
    }

    /// Разбирает стрелочную функцию: (params) => body
    fn parse_arrow_function(&mut self) -> Result<Expr, ParseError> {
        self.expect_symbol("(")?;
        let mut params = Vec::new();
        if !self.check(&TokenKind::Symbol(")".to_string())) {
            loop {
                params.push(self.expect_identifier()?);
                if !self.match_symbol(",") {
                    break;
                }
            }
        }
        self.expect_symbol(")")?;
        self.expect_symbol("=>")?;

        // Тело стрелочной функции
        let body = if self.match_symbol("{") {
            self.parse_block()?
        } else {
            // Выражение-тело: (params) => expr
            let expr = self.parse_expression()?;
            let line = self.previous().line;
            Stmt::Return { expr: Some(expr), line }
        };

        Ok(Expr::Function {
            name: None,
            params,
            body: Box::new(body),
            is_arrow: true,
        })
    }

    /// Проверяет, является ли текущий идентификатор параметром стрелочной функции
    /// с одним параметром без скобок: `ident => body`.
    fn is_single_param_arrow_function(&self) -> bool {
        // Текущий токен — идентификатор. Проверяем, что следующий токен — `=>`.
        matches!(
            self.tokens.get(self.pos + 1).map(|t| &t.kind),
            Some(TokenKind::Symbol(s)) if s == "=>"
        )
    }

    /// Разбирает стрелочную функцию с одним параметром без скобок: `ident => body`.
    fn parse_single_param_arrow_function(&mut self) -> Result<Expr, ParseError> {
        let param = self.expect_identifier()?;
        self.expect_symbol("=>")?;

        // Тело стрелочной функции
        let body = if self.match_symbol("{") {
            self.parse_block()?
        } else {
            // Выражение-тело: ident => expr
            let expr = self.parse_expression()?;
            let line = self.previous().line;
            Stmt::Return { expr: Some(expr), line }
        };

        Ok(Expr::Function {
            name: None,
            params: vec![param],
            body: Box::new(body),
            is_arrow: true,
        })
    }

    // --- Статический анализ: детекция потенциального зацикливания очередей ---

    /// Детектирует код, который может привести к бесконечному зацикливанию
    /// микрозадач или макрозадач, и возвращает список предупреждений.
    ///
    /// Анализ выполняется на уровне AST (статически, до выполнения кода):
    /// - **микрозадачи**: функция вызывает саму себя рекурсивно через
    ///   `Promise.resolve().then(fn)` или `Promise.resolve().then(() => fn())`;
    /// - **макрозадачи**: функция вызывает саму себя рекурсивно через
    ///   `setTimeout(fn, 0)`.
    ///
    /// Такой код не обязательно является ошибкой (например, рекурсия может
    /// быть ограничена условием), поэтому вместо прерывания выполнения
    /// генерируется предупреждение с рекомендацией прервать исполнение
    /// кнопкой сброса, если очереди начнут бесконечно расти.
    pub fn detect_queue_loops(&self, program: &Program) -> Vec<String> {
        let mut warnings = Vec::new();

        // Собираем все именованные функции верхнего уровня: как объявленные через
        // `function name() {}`, так и присвоенные переменным `const name = () => {}`.
        for stmt in &program.statements {
            let (name, body) = match stmt {
                Stmt::FunctionDecl { name, body, .. } => (name.clone(), body),
                Stmt::VarDecl { name, init, .. } => {
                    // Инициализатор — функция (стрелочная или обычная).
                    match init {
                        Some(Expr::Function { body, .. }) => (name.clone(), body),
                        Some(Expr::AsyncFunction { body, .. }) => (name.clone(), body),
                        _ => continue,
                    }
                }
                _ => continue,
            };

            // Проверяем тело функции на рекурсивный вызов через очереди.
            let mut micro_loop = false;
            let mut macro_loop = false;
            Self::scan_stmt_for_loop(body, &name, &mut micro_loop, &mut macro_loop);

            if micro_loop {
                warnings.push(format!(
                    "⚠️ Функция `{}` вызывает саму себя через Promise.resolve().then(). \
                     Это может привести к бесконечному зацикливанию микрозадач и заблокировать \
                     рендеринг. Если очередь микрозадач начнёт бесконечно расти, прервите \
                     исполнение кнопкой «Сброс».",
                    name
                ));
            }
            if macro_loop {
                warnings.push(format!(
                    "⚠️ Функция `{}` вызывает саму себя через setTimeout(). \
                     Это может привести к бесконечному зацикливанию макрозадач. \
                     Если очередь макрозадач начнёт бесконечно расти, прервите \
                     исполнение кнопкой «Сброс».",
                    name
                ));
            }
        }

        warnings
    }

    /// Рекурсивно сканирует оператор на предмет рекурсивного вызова функции
    /// `target` через микрозадачу (`Promise.resolve().then`) или макрозадачу
    /// (`setTimeout`).
    fn scan_stmt_for_loop(
        stmt: &Stmt,
        target: &str,
        micro_loop: &mut bool,
        macro_loop: &mut bool,
    ) {
        match stmt {
            Stmt::VarDecl { init, .. } => {
                if let Some(init) = init {
                    Self::scan_expr_for_loop(init, target, micro_loop, macro_loop);
                }
            }
            Stmt::Expr { expr, .. } => {
                Self::scan_expr_for_loop(expr, target, micro_loop, macro_loop);
            }
            Stmt::Block { statements, .. } => {
                for s in statements {
                    Self::scan_stmt_for_loop(s, target, micro_loop, macro_loop);
                }
            }
            Stmt::If { cond, then_branch, else_branch, .. } => {
                Self::scan_expr_for_loop(cond, target, micro_loop, macro_loop);
                Self::scan_stmt_for_loop(then_branch, target, micro_loop, macro_loop);
                if let Some(else_branch) = else_branch {
                    Self::scan_stmt_for_loop(else_branch, target, micro_loop, macro_loop);
                }
            }
            Stmt::While { cond, body, .. } => {
                Self::scan_expr_for_loop(cond, target, micro_loop, macro_loop);
                Self::scan_stmt_for_loop(body, target, micro_loop, macro_loop);
            }
            Stmt::For { init, cond, update, body, .. } => {
                if let Some(init) = init {
                    Self::scan_stmt_for_loop(init, target, micro_loop, macro_loop);
                }
                if let Some(cond) = cond {
                    Self::scan_expr_for_loop(cond, target, micro_loop, macro_loop);
                }
                if let Some(update) = update {
                    Self::scan_expr_for_loop(update, target, micro_loop, macro_loop);
                }
                Self::scan_stmt_for_loop(body, target, micro_loop, macro_loop);
            }
            Stmt::Return { expr, .. } => {
                if let Some(expr) = expr {
                    Self::scan_expr_for_loop(expr, target, micro_loop, macro_loop);
                }
            }
            Stmt::FunctionDecl { body, .. } => {
                Self::scan_stmt_for_loop(body, target, micro_loop, macro_loop);
            }
            Stmt::Throw { expr, .. } => {
                Self::scan_expr_for_loop(expr, target, micro_loop, macro_loop);
            }
            Stmt::Try { body, catch_body, finally_body, .. } => {
                Self::scan_stmt_for_loop(body, target, micro_loop, macro_loop);
                if let Some(catch_body) = catch_body {
                    Self::scan_stmt_for_loop(catch_body, target, micro_loop, macro_loop);
                }
                if let Some(finally_body) = finally_body {
                    Self::scan_stmt_for_loop(finally_body, target, micro_loop, macro_loop);
                }
            }
            Stmt::Switch { expr, cases, default, .. } => {
                Self::scan_expr_for_loop(expr, target, micro_loop, macro_loop);
                for (case_expr, case_stmts) in cases {
                    Self::scan_expr_for_loop(case_expr, target, micro_loop, macro_loop);
                    for s in case_stmts {
                        Self::scan_stmt_for_loop(s, target, micro_loop, macro_loop);
                    }
                }
                if let Some(default) = default {
                    for s in default {
                        Self::scan_stmt_for_loop(s, target, micro_loop, macro_loop);
                    }
                }
            }
            Stmt::ClassDecl { methods, .. } => {
                for (_, _, body) in methods {
                    Self::scan_stmt_for_loop(body, target, micro_loop, macro_loop);
                }
            }
            Stmt::Break { .. } | Stmt::Continue { .. } => {}
        }
    }

    /// Рекурсивно сканирует выражение на предмет рекурсивного вызова функции
    /// `target` через микрозадачу (`Promise.resolve().then`) или макрозадачу
    /// (`setTimeout`).
    fn scan_expr_for_loop(
        expr: &Expr,
        target: &str,
        micro_loop: &mut bool,
        macro_loop: &mut bool,
    ) {
        match expr {
            Expr::Assign { value, .. } => {
                Self::scan_expr_for_loop(value, target, micro_loop, macro_loop);
            }
            Expr::SetMember { object, property, value } => {
                Self::scan_expr_for_loop(object, target, micro_loop, macro_loop);
                Self::scan_expr_for_loop(property, target, micro_loop, macro_loop);
                Self::scan_expr_for_loop(value, target, micro_loop, macro_loop);
            }
            Expr::Binary { left, right, .. } => {
                Self::scan_expr_for_loop(left, target, micro_loop, macro_loop);
                Self::scan_expr_for_loop(right, target, micro_loop, macro_loop);
            }
            Expr::Unary { expr, .. } => {
                Self::scan_expr_for_loop(expr, target, micro_loop, macro_loop);
            }
            Expr::Logical { left, right, .. } => {
                Self::scan_expr_for_loop(left, target, micro_loop, macro_loop);
                Self::scan_expr_for_loop(right, target, micro_loop, macro_loop);
            }
            Expr::Ternary { cond, then_expr, else_expr } => {
                Self::scan_expr_for_loop(cond, target, micro_loop, macro_loop);
                Self::scan_expr_for_loop(then_expr, target, micro_loop, macro_loop);
                Self::scan_expr_for_loop(else_expr, target, micro_loop, macro_loop);
            }
            Expr::Call { callee, args } => {
                // Проверяем, является ли вызов рекурсивным через очередь.
                Self::check_call_for_loop(callee, args, target, micro_loop, macro_loop);
                // Продолжаем сканирование вложенных выражений.
                Self::scan_expr_for_loop(callee, target, micro_loop, macro_loop);
                for arg in args {
                    Self::scan_expr_for_loop(arg, target, micro_loop, macro_loop);
                }
            }
            Expr::Member { object, property } => {
                Self::scan_expr_for_loop(object, target, micro_loop, macro_loop);
                Self::scan_expr_for_loop(property, target, micro_loop, macro_loop);
            }
            Expr::Object(properties) => {
                for (_, value) in properties {
                    Self::scan_expr_for_loop(value, target, micro_loop, macro_loop);
                }
            }
            Expr::Array(items) => {
                for item in items {
                    Self::scan_expr_for_loop(item, target, micro_loop, macro_loop);
                }
            }
            Expr::Function { body, .. } => {
                Self::scan_stmt_for_loop(body, target, micro_loop, macro_loop);
            }
            Expr::AsyncFunction { body, .. } => {
                Self::scan_stmt_for_loop(body, target, micro_loop, macro_loop);
            }
            Expr::Typeof(expr) => {
                Self::scan_expr_for_loop(expr, target, micro_loop, macro_loop);
            }
            Expr::New { callee, args } => {
                Self::scan_expr_for_loop(callee, target, micro_loop, macro_loop);
                for arg in args {
                    Self::scan_expr_for_loop(arg, target, micro_loop, macro_loop);
                }
            }
            Expr::Await(expr) => {
                Self::scan_expr_for_loop(expr, target, micro_loop, macro_loop);
            }
            Expr::Number(_)
            | Expr::String(_)
            | Expr::Boolean(_)
            | Expr::Null
            | Expr::Undefined
            | Expr::Identifier(_)
            | Expr::This => {}
        }
    }

    /// Проверяет, является ли вызов `callee(args)` рекурсивным вызовом функции
    /// `target` через микрозадачу (`Promise.resolve().then`) или макрозадачу
    /// (`setTimeout`).
    fn check_call_for_loop(
        callee: &Expr,
        args: &[Expr],
        target: &str,
        micro_loop: &mut bool,
        macro_loop: &mut bool,
    ) {
        // Макрозадача: setTimeout(fn, ...) — callee — идентификатор `setTimeout`.
        if let Expr::Identifier(name) = callee {
            if name == "setTimeout" && !*macro_loop {
                // Проверяем, что первый аргумент — это вызов/ссылка на `target`.
                if let Some(first) = args.first() {
                    if Self::expr_refers_to(first, target) {
                        *macro_loop = true;
                    }
                }
            }
            return;
        }

        // Микрозадача: Promise.resolve().then(fn) — callee — Member `.then`.
        if let Expr::Member { object, property } = callee {
            if let Expr::Identifier(prop_name) = &**property {
                if prop_name == "then" && !*micro_loop {
                    // Проверяем, что первый аргумент `.then` — это вызов/ссылка на `target`.
                    if let Some(first) = args.first() {
                        if Self::expr_refers_to(first, target) {
                            *micro_loop = true;
                        }
                    }
                }
            }
        }
    }

    /// Проверяет, ссылается ли выражение на функцию `target` (прямо или через
    /// вызов `() => target()`).
    fn expr_refers_to(expr: &Expr, target: &str) -> bool {
        match expr {
            // Прямая ссылка: setTimeout(someFunc, 0) или .then(someFunc).
            Expr::Identifier(name) => name == target,
            // Стрелочная функция-обёртка: setTimeout(() => someFunc(), 0)
            // или .then(() => someFunc()).
            Expr::Function { body, .. } => {
                // Ищем вызов `target()` в теле функции-обёртки.
                let mut found = false;
                Self::scan_stmt_for_direct_call(body, target, &mut found);
                found
            }
            _ => false,
        }
    }

    /// Ищет прямой вызов `target()` внутри оператора.
    fn scan_stmt_for_direct_call(stmt: &Stmt, target: &str, found: &mut bool) {
        if *found {
            return;
        }
        match stmt {
            Stmt::Expr { expr, .. } => Self::scan_expr_for_direct_call(expr, target, found),
            Stmt::Block { statements, .. } => {
                for s in statements {
                    Self::scan_stmt_for_direct_call(s, target, found);
                }
            }
            Stmt::VarDecl { init, .. } => {
                if let Some(init) = init {
                    Self::scan_expr_for_direct_call(init, target, found);
                }
            }
            Stmt::If { cond, then_branch, else_branch, .. } => {
                Self::scan_expr_for_direct_call(cond, target, found);
                Self::scan_stmt_for_direct_call(then_branch, target, found);
                if let Some(else_branch) = else_branch {
                    Self::scan_stmt_for_direct_call(else_branch, target, found);
                }
            }
            Stmt::While { cond, body, .. } => {
                Self::scan_expr_for_direct_call(cond, target, found);
                Self::scan_stmt_for_direct_call(body, target, found);
            }
            Stmt::For { init, cond, update, body, .. } => {
                if let Some(init) = init {
                    Self::scan_stmt_for_direct_call(init, target, found);
                }
                if let Some(cond) = cond {
                    Self::scan_expr_for_direct_call(cond, target, found);
                }
                if let Some(update) = update {
                    Self::scan_expr_for_direct_call(update, target, found);
                }
                Self::scan_stmt_for_direct_call(body, target, found);
            }
            Stmt::Return { expr, .. } => {
                if let Some(expr) = expr {
                    Self::scan_expr_for_direct_call(expr, target, found);
                }
            }
            Stmt::FunctionDecl { body, .. } => {
                Self::scan_stmt_for_direct_call(body, target, found);
            }
            Stmt::Throw { expr, .. } => Self::scan_expr_for_direct_call(expr, target, found),
            Stmt::Try { body, catch_body, finally_body, .. } => {
                Self::scan_stmt_for_direct_call(body, target, found);
                if let Some(catch_body) = catch_body {
                    Self::scan_stmt_for_direct_call(catch_body, target, found);
                }
                if let Some(finally_body) = finally_body {
                    Self::scan_stmt_for_direct_call(finally_body, target, found);
                }
            }
            Stmt::Switch { expr, cases, default, .. } => {
                Self::scan_expr_for_direct_call(expr, target, found);
                for (case_expr, case_stmts) in cases {
                    Self::scan_expr_for_direct_call(case_expr, target, found);
                    for s in case_stmts {
                        Self::scan_stmt_for_direct_call(s, target, found);
                    }
                }
                if let Some(default) = default {
                    for s in default {
                        Self::scan_stmt_for_direct_call(s, target, found);
                    }
                }
            }
            Stmt::ClassDecl { methods, .. } => {
                for (_, _, body) in methods {
                    Self::scan_stmt_for_direct_call(body, target, found);
                }
            }
            Stmt::Break { .. } | Stmt::Continue { .. } => {}
        }
    }

    /// Ищет прямой вызов `target()` внутри выражения.
    fn scan_expr_for_direct_call(expr: &Expr, target: &str, found: &mut bool) {
        if *found {
            return;
        }
        match expr {
            Expr::Call { callee, args } => {
                // Прямой вызов: target(...).
                if let Expr::Identifier(name) = &**callee {
                    if name == target {
                        *found = true;
                        return;
                    }
                }
                Self::scan_expr_for_direct_call(callee, target, found);
                for arg in args {
                    Self::scan_expr_for_direct_call(arg, target, found);
                }
            }
            Expr::Assign { value, .. } => Self::scan_expr_for_direct_call(value, target, found),
            Expr::SetMember { object, property, value } => {
                Self::scan_expr_for_direct_call(object, target, found);
                Self::scan_expr_for_direct_call(property, target, found);
                Self::scan_expr_for_direct_call(value, target, found);
            }
            Expr::Binary { left, right, .. } => {
                Self::scan_expr_for_direct_call(left, target, found);
                Self::scan_expr_for_direct_call(right, target, found);
            }
            Expr::Unary { expr, .. } => Self::scan_expr_for_direct_call(expr, target, found),
            Expr::Logical { left, right, .. } => {
                Self::scan_expr_for_direct_call(left, target, found);
                Self::scan_expr_for_direct_call(right, target, found);
            }
            Expr::Ternary { cond, then_expr, else_expr } => {
                Self::scan_expr_for_direct_call(cond, target, found);
                Self::scan_expr_for_direct_call(then_expr, target, found);
                Self::scan_expr_for_direct_call(else_expr, target, found);
            }
            Expr::Member { object, property } => {
                Self::scan_expr_for_direct_call(object, target, found);
                Self::scan_expr_for_direct_call(property, target, found);
            }
            Expr::Object(properties) => {
                for (_, value) in properties {
                    Self::scan_expr_for_direct_call(value, target, found);
                }
            }
            Expr::Array(items) => {
                for item in items {
                    Self::scan_expr_for_direct_call(item, target, found);
                }
            }
            Expr::Function { body, .. } => Self::scan_stmt_for_direct_call(body, target, found),
            Expr::AsyncFunction { body, .. } => Self::scan_stmt_for_direct_call(body, target, found),
            Expr::Typeof(expr) => Self::scan_expr_for_direct_call(expr, target, found),
            Expr::New { callee, args } => {
                Self::scan_expr_for_direct_call(callee, target, found);
                for arg in args {
                    Self::scan_expr_for_direct_call(arg, target, found);
                }
            }
            Expr::Await(expr) => Self::scan_expr_for_direct_call(expr, target, found),
            Expr::Number(_)
            | Expr::String(_)
            | Expr::Boolean(_)
            | Expr::Null
            | Expr::Undefined
            | Expr::Identifier(_)
            | Expr::This => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_var_decl() {
        let mut parser = Parser::new("let x = 42;").unwrap();
        let program = parser.parse_program().unwrap();
        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Stmt::VarDecl { name, init, kind, .. } => {
                assert_eq!(name, "x");
                assert_eq!(*kind, VarKind::Let);
                assert!(matches!(init, Some(Expr::Number(42.0))));
            }
            _ => panic!("Ожидался VarDecl"),
        }
    }

    #[test]
    fn test_parse_function() {
        let mut parser = Parser::new("function add(a, b) { return a + b; }").unwrap();
        let program = parser.parse_program().unwrap();
        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Stmt::FunctionDecl { name, params, .. } => {
                assert_eq!(name, "add");
                assert_eq!(params.len(), 2);
            }
            _ => panic!("Ожидался FunctionDecl"),
        }
    }

    #[test]
    fn test_parse_console_log() {
        let mut parser = Parser::new("console.log('hello');").unwrap();
        let program = parser.parse_program().unwrap();
        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Stmt::Expr { expr: Expr::Call { callee, .. }, .. } => {
                assert!(matches!(**callee, Expr::Member { .. }));
            }
            _ => panic!("Ожидался Call"),
        }
    }

    #[test]
    fn test_parse_new_promise() {
        let mut parser = Parser::new("new Promise((resolve) => { resolve(3); });").unwrap();
        let program = parser.parse_program().unwrap();
        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Stmt::Expr { expr: Expr::New { callee, args }, .. } => {
                assert!(matches!(**callee, Expr::Identifier(ref n) if n == "Promise"));
                assert_eq!(args.len(), 1);
            }
            _ => panic!("Ожидался New"),
        }
    }

    #[test]
    fn test_parse_single_param_arrow() {
        // Стрелочная функция с одним параметром без скобок: result => { ... }
        let mut parser = Parser::new("Promise.resolve(4).then(result => { return result; });").unwrap();
        let program = parser.parse_program().unwrap();
        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Stmt::Expr { expr: Expr::Call { callee, args }, .. } => {
                assert!(matches!(**callee, Expr::Member { .. }));
                assert_eq!(args.len(), 1);
                assert!(matches!(args[0], Expr::Function { is_arrow: true, .. }));
            }
            _ => panic!("Ожидался Call"),
        }
    }

    #[test]
    fn test_parse_multiline_promise_chain() {
        // Многострочная цепочка промисов со стрелочными функциями без скобок.
        let source = "\
            new Promise((resolve) => { \
                console.log(2); \
                resolve(3); \
            }).then(result => { \
                console.log(result); \
                return 8; \
            }).then(console.log);";
        let mut parser = Parser::new(source).unwrap();
        let program = parser.parse_program().unwrap();
        assert_eq!(program.statements.len(), 1);
    }
}