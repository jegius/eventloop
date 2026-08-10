//! Определения узлов AST (Abstract Syntax Tree).
//!
//! AST строится парсером и исполняется интерпретатором.
//! На данном этапе определены базовые узлы, необходимые для
//! демонстрации механизма Event Loop.

/// Программа — корневой узел AST.
#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    /// Список операторов программы.
    pub statements: Vec<Stmt>,
}

/// Оператор (statement).
#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    /// Объявление переменной.
    VarDecl {
        name: String,
        init: Option<Expr>,
        /// `const` / `let` / `var`
        kind: VarKind,
        /// Номер строки в исходном коде (1-based).
        line: usize,
    },
    /// Выражение-оператор.
    Expr {
        expr: Expr,
        /// Номер строки в исходном коде (1-based).
        line: usize,
    },
    /// Блок операторов.
    Block {
        statements: Vec<Stmt>,
        /// Номер строки в исходном коде (1-based).
        line: usize,
    },
    /// `if` / `else`
    If {
        cond: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
        /// Номер строки в исходном коде (1-based).
        line: usize,
    },
    /// `while (cond) body`
    While {
        cond: Expr,
        body: Box<Stmt>,
        /// Номер строки в исходном коде (1-based).
        line: usize,
    },
    /// `for (init; cond; update) body`
    For {
        init: Option<Box<Stmt>>,
        cond: Option<Expr>,
        update: Option<Expr>,
        body: Box<Stmt>,
        /// Номер строки в исходном коде (1-based).
        line: usize,
    },
    /// `return expr`
    Return {
        expr: Option<Expr>,
        /// Номер строки в исходном коде (1-based).
        line: usize,
    },
    /// `break`
    Break {
        /// Номер строки в исходном коде (1-based).
        line: usize,
    },
    /// `continue`
    Continue {
        /// Номер строки в исходном коде (1-based).
        line: usize,
    },
    /// Объявление функции.
    FunctionDecl {
        name: String,
        params: Vec<String>,
        body: Box<Stmt>,
        /// Является ли функция async-функцией.
        is_async: bool,
        /// Номер строки в исходном коде (1-based).
        line: usize,
    },
    /// `throw expr`
    Throw {
        expr: Expr,
        /// Номер строки в исходном коде (1-based).
        line: usize,
    },
    /// `try { } catch (e) { } finally { }`
    Try {
        body: Box<Stmt>,
        catch_param: Option<String>,
        catch_body: Option<Box<Stmt>>,
        finally_body: Option<Box<Stmt>>,
        /// Номер строки в исходном коде (1-based).
        line: usize,
    },
    /// `switch (expr) { case v: ... default: ... }`
    Switch {
        expr: Expr,
        /// Список `case`-веток: (значение, операторы).
        cases: Vec<(Expr, Vec<Stmt>)>,
        /// Операторы ветки `default`.
        default: Option<Vec<Stmt>>,
        /// Номер строки в исходном коде (1-based).
        line: usize,
    },
    /// `class Name { constructor(...) {...} method(...) {...} }`
    ClassDecl {
        name: String,
        /// Методы класса: (имя, параметры, тело).
        methods: Vec<(String, Vec<String>, Box<Stmt>)>,
        /// Номер строки в исходном коде (1-based).
        line: usize,
    },
}

/// Вид объявления переменной.
#[derive(Debug, Clone, PartialEq)]
pub enum VarKind {
    Var,
    Let,
    Const,
}

/// Выражение (expression).
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    /// Числовой литерал.
    Number(f64),
    /// Строковый литерал.
    String(String),
    /// Булев литерал.
    Boolean(bool),
    /// `null`
    Null,
    /// `undefined`
    Undefined,
    /// Идентификатор (переменная).
    Identifier(String),
    /// Присваивание.
    Assign {
        name: String,
        value: Box<Expr>,
    },
    /// Присваивание свойству объекта: `obj.prop = value`.
    SetMember {
        object: Box<Expr>,
        property: Box<Expr>,
        value: Box<Expr>,
    },
    /// Бинарная операция.
    Binary {
        left: Box<Expr>,
        op: String,
        right: Box<Expr>,
    },
    /// Унарная операция.
    Unary {
        op: String,
        expr: Box<Expr>,
    },
    /// Логическое И/ИЛИ.
    Logical {
        left: Box<Expr>,
        op: String,
        right: Box<Expr>,
    },
    /// Тернарный оператор.
    Ternary {
        cond: Box<Expr>,
        then_expr: Box<Expr>,
        else_expr: Box<Expr>,
    },
    /// Вызов функции.
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },
    /// Доступ к свойству.
    Member {
        object: Box<Expr>,
        property: Box<Expr>,
    },
    /// Объектный литерал.
    Object(Vec<(String, Expr)>),
    /// Массивный литерал.
    Array(Vec<Expr>),
    /// Функциональное выражение.
    Function {
        name: Option<String>,
        params: Vec<String>,
        body: Box<Stmt>,
        is_arrow: bool,
    },
    /// `this`
    This,
    /// `typeof expr`
    Typeof(Box<Expr>),
    /// `new Expr(args)`
    New {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },
    /// `await expr`
    Await(Box<Expr>),
    /// `async function`
    AsyncFunction {
        name: Option<String>,
        params: Vec<String>,
        body: Box<Stmt>,
    },
}

impl Stmt {
    /// Возвращает строковое представление оператора (для визуализации).
    pub fn display(&self) -> String {
        match self {
            Stmt::VarDecl { name, init, .. } => {
                match init {
                    Some(init) => format!("let {} = {}", name, init.display()),
                    None => format!("let {}", name),
                }
            }
            Stmt::Expr { expr, .. } => expr.display(),
            Stmt::Block { statements, .. } => {
                let stmts: Vec<String> = statements.iter().map(|s| s.display()).collect();
                format!("{{ {} }}", stmts.join("; "))
            }
            Stmt::If { cond, then_branch, else_branch, .. } => {
                let mut s = format!("if ({}) {}", cond.display(), then_branch.display());
                if let Some(else_branch) = else_branch {
                    s.push_str(&format!(" else {}", else_branch.display()));
                }
                s
            }
            Stmt::While { cond, body, .. } => format!("while ({}) {}", cond.display(), body.display()),
            Stmt::For { init, cond, update, body, .. } => {
                let init_s = init.as_ref().map(|i| i.display()).unwrap_or_default();
                let cond_s = cond.as_ref().map(|c| c.display()).unwrap_or_default();
                let update_s = update.as_ref().map(|u| u.display()).unwrap_or_default();
                format!("for ({}; {}; {}) {}", init_s, cond_s, update_s, body.display())
            }
            Stmt::Return { expr, .. } => match expr {
                Some(e) => format!("return {}", e.display()),
                None => "return".to_string(),
            },
            Stmt::Break { .. } => "break".to_string(),
            Stmt::Continue { .. } => "continue".to_string(),
            Stmt::FunctionDecl { name, params, is_async, .. } => {
                let prefix = if *is_async { "async " } else { "" };
                format!("{}function {}({})", prefix, name, params.join(", "))
            }
            Stmt::Throw { expr, .. } => format!("throw {}", expr.display()),
            Stmt::Try { body, .. } => format!("try {}", body.display()),
            Stmt::Switch { expr, .. } => format!("switch ({})", expr.display()),
            Stmt::ClassDecl { name, .. } => format!("class {}", name),
        }
    }

    /// Возвращает номер строки оператора в исходном коде (1-based, 0 — неизвестно).
    pub fn line(&self) -> usize {
        match self {
            Stmt::VarDecl { line, .. }
            | Stmt::Expr { line, .. }
            | Stmt::Block { line, .. }
            | Stmt::If { line, .. }
            | Stmt::While { line, .. }
            | Stmt::For { line, .. }
            | Stmt::Return { line, .. }
            | Stmt::Break { line }
            | Stmt::Continue { line }
            | Stmt::FunctionDecl { line, .. }
            | Stmt::Throw { line, .. }
            | Stmt::Try { line, .. }
            | Stmt::Switch { line, .. }
            | Stmt::ClassDecl { line, .. } => *line,
        }
    }

    /// Возвращает диапазон строк `(start, end)`, покрытых оператором
    /// (включая вложенные операторы). Используется для подсветки всего
    /// тела колбэка целиком, а не одной строки.
    pub fn line_range(&self) -> (usize, usize) {
        let line = self.line();
        let mut min = line;
        let mut max = line;
        self.collect_lines(&mut min, &mut max);
        (min, max)
    }

    /// Рекурсивно собирает минимальную и максимальную строку среди
    /// оператора и всех его вложенных операторов.
    fn collect_lines(&self, min: &mut usize, max: &mut usize) {
        let line = self.line();
        if line > 0 {
            if *min == 0 || line < *min {
                *min = line;
            }
            if line > *max {
                *max = line;
            }
        }
        match self {
            Stmt::Block { statements, .. } => {
                for s in statements {
                    s.collect_lines(min, max);
                }
            }
            Stmt::If { then_branch, else_branch, .. } => {
                then_branch.collect_lines(min, max);
                if let Some(e) = else_branch {
                    e.collect_lines(min, max);
                }
            }
            Stmt::While { body, .. } => body.collect_lines(min, max),
            Stmt::For { init, body, .. } => {
                if let Some(i) = init {
                    i.collect_lines(min, max);
                }
                body.collect_lines(min, max);
            }
            Stmt::FunctionDecl { body, .. } => body.collect_lines(min, max),
            Stmt::Try { body, catch_body, finally_body, .. } => {
                body.collect_lines(min, max);
                if let Some(c) = catch_body {
                    c.collect_lines(min, max);
                }
                if let Some(f) = finally_body {
                    f.collect_lines(min, max);
                }
            }
            Stmt::Switch { cases, default, .. } => {
                for (_, stmts) in cases {
                    for s in stmts {
                        s.collect_lines(min, max);
                    }
                }
                if let Some(d) = default {
                    for s in d {
                        s.collect_lines(min, max);
                    }
                }
            }
            Stmt::ClassDecl { methods, .. } => {
                for (_, _, body) in methods {
                    body.collect_lines(min, max);
                }
            }
            _ => {}
        }
    }
}

impl Expr {
    /// Возвращает строковое представление выражения (для визуализации).
    pub fn display(&self) -> String {
        match self {
            Expr::Number(n) => {
                if n.fract() == 0.0 && n.is_finite() {
                    format!("{}", *n as i64)
                } else {
                    n.to_string()
                }
            }
            Expr::String(s) => format!("'{}'", s),
            Expr::Boolean(b) => b.to_string(),
            Expr::Null => "null".to_string(),
            Expr::Undefined => "undefined".to_string(),
            Expr::Identifier(name) => name.clone(),
            Expr::Assign { name, value } => format!("{} = {}", name, value.display()),
            Expr::SetMember { object, property, value } => {
                format!("{}.{} = {}", object.display(), property.display(), value.display())
            }
            Expr::Binary { left, op, right } => format!("{} {} {}", left.display(), op, right.display()),
            Expr::Unary { op, expr } => format!("{}{}", op, expr.display()),
            Expr::Logical { left, op, right } => format!("{} {} {}", left.display(), op, right.display()),
            Expr::Ternary { cond, then_expr, else_expr } => {
                format!("{} ? {} : {}", cond.display(), then_expr.display(), else_expr.display())
            }
            Expr::Call { callee, args } => {
                let args_str: Vec<String> = args.iter().map(|a| a.display()).collect();
                format!("{}({})", callee.display(), args_str.join(", "))
            }
            Expr::Member { object, property } => format!("{}.{}", object.display(), property.display()),
            Expr::Object(properties) => {
                let props: Vec<String> = properties.iter().map(|(k, v)| format!("{}: {}", k, v.display())).collect();
                format!("{{{}}}", props.join(", "))
            }
            Expr::Array(items) => {
                let items_str: Vec<String> = items.iter().map(|i| i.display()).collect();
                format!("[{}]", items_str.join(", "))
            }
            Expr::Function { name, params, .. } => {
                let name_str = name.clone().unwrap_or_default();
                format!("function {}({})", name_str, params.join(", "))
            }
            Expr::This => "this".to_string(),
            Expr::Typeof(expr) => format!("typeof {}", expr.display()),
            Expr::New { callee, args } => {
                let args_str: Vec<String> = args.iter().map(|a| a.display()).collect();
                format!("new {}({})", callee.display(), args_str.join(", "))
            }
            Expr::Await(expr) => format!("await {}", expr.display()),
            Expr::AsyncFunction { name, params, .. } => {
                let name_str = name.as_deref().unwrap_or("");
                format!("async function {}({})", name_str, params.join(", "))
            }
        }
    }
}

impl Program {
    /// Создаёт пустую программу.
    pub fn new() -> Self {
        Program { statements: Vec::new() }
    }
}

impl Default for Program {
    fn default() -> Self {
        Self::new()
    }
}