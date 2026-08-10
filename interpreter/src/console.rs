//! Консоль вывода.
//!
//! Собирает вывод `console.log`, `console.error` и т.д. в список строк,
//! который затем передаётся во фронтенд для отображения.

/// Консоль вывода.
#[derive(Debug, Clone, Default)]
pub struct Console {
    /// Накопленный вывод.
    output: Vec<String>,
}

impl Console {
    /// Создаёт новую консоль.
    pub fn new() -> Self {
        Console { output: Vec::new() }
    }

    /// Выводит строку в консоль.
    pub fn log(&mut self, message: &str) {
        self.output.push(message.to_string());
    }

    /// Выводит сообщение об ошибке.
    pub fn error(&mut self, message: &str) {
        self.output.push(format!("[ERROR] {}", message));
    }

    /// Возвращает накопленный вывод.
    pub fn get_output(&self) -> Vec<String> {
        self.output.clone()
    }

    /// Очищает консоль.
    pub fn clear(&mut self) {
        self.output.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log() {
        let mut console = Console::new();
        console.log("hello");
        console.log("world");
        assert_eq!(console.get_output(), vec!["hello", "world"]);
    }

    #[test]
    fn test_error() {
        let mut console = Console::new();
        console.error("boom");
        assert_eq!(console.get_output(), vec!["[ERROR] boom"]);
    }
}