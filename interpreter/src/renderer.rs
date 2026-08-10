//! Минимальный рендер-движок.
//!
//! Реализует настоящие (минимальные) фазы рендера браузера:
//! 1. Parsing HTML → DOM.
//! 2. Parsing CSS → CSSOM.
//! 3. DOM (построение дерева DOM).
//! 4. CSSOM (построение дерева CSSOM).
//! 5. Render Tree (объединение DOM + CSSOM).
//! 6. Layout (вычисление геометрии).
//! 7. Paint (отрисовка).
//! 8. Compose (композиция слоёв).
//!
//! Каждая фаза генерирует событие для визуализации.

use std::collections::HashMap;

/// Фаза рендера.
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum RenderPhase {
    /// Выполнение колбэков requestAnimationFrame (часть шага рендеринга).
    AnimationFrame,
    /// Разбор HTML.
    ParsingHtml,
    /// Разбор CSS.
    ParsingCss,
    /// Построение DOM.
    Dom,
    /// Построение CSSOM.
    Cssom,
    /// Построение Render Tree.
    RenderTree,
    /// Вычисление геометрии (Layout).
    Layout,
    /// Отрисовка (Paint).
    Paint,
    /// Композиция слоёв (Compose).
    Compose,
}

impl RenderPhase {
    /// Возвращает название фазы (для визуализации).
    pub fn name(&self) -> &'static str {
        match self {
            RenderPhase::AnimationFrame => "requestAnimationFrame",
            RenderPhase::ParsingHtml => "Parsing HTML",
            RenderPhase::ParsingCss => "Parsing CSS",
            RenderPhase::Dom => "DOM",
            RenderPhase::Cssom => "CSSOM",
            RenderPhase::RenderTree => "Render Tree",
            RenderPhase::Layout => "Layout",
            RenderPhase::Paint => "Paint",
            RenderPhase::Compose => "Compose",
        }
    }
}

/// Узел DOM.
#[derive(Debug, Clone, PartialEq)]
pub struct DomNode {
    /// Тег или текст.
    pub tag: String,
    /// Атрибуты.
    pub attributes: HashMap<String, String>,
    /// Дочерние узлы.
    pub children: Vec<DomNode>,
}

impl DomNode {
    /// Создаёт элемент.
    pub fn element(tag: &str, attributes: HashMap<String, String>) -> Self {
        DomNode {
            tag: tag.to_string(),
            attributes,
            children: Vec::new(),
        }
    }

    /// Создаёт текстовый узел.
    pub fn text(content: &str) -> Self {
        DomNode {
            tag: "#text".to_string(),
            attributes: HashMap::new(),
            children: vec![DomNode {
                tag: content.to_string(),
                attributes: HashMap::new(),
                children: Vec::new(),
            }],
        }
    }
}

/// Узел CSSOM.
#[derive(Debug, Clone, PartialEq)]
pub struct CssNode {
    /// Селектор.
    pub selector: String,
    /// Стили (свойство → значение).
    pub styles: HashMap<String, String>,
}

/// Узел Render Tree.
#[derive(Debug, Clone, PartialEq)]
pub struct RenderNode {
    /// Соответствующий DOM-узел.
    pub dom_tag: String,
    /// Вычисленные стили.
    pub computed_styles: HashMap<String, String>,
    /// Дочерние узлы.
    pub children: Vec<RenderNode>,
}

/// Геометрия узла (результат Layout).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LayoutBox {
    /// Тег узла.
    pub tag: String,
    /// X-координата.
    pub x: f64,
    /// Y-координата.
    pub y: f64,
    /// Ширина.
    pub width: f64,
    /// Высота.
    pub height: f64,
    /// Дочерние боксы.
    pub children: Vec<LayoutBox>,
}

/// Минимальный рендер-движок.
#[derive(Debug, Clone, Default)]
pub struct Renderer {
    /// DOM-дерево.
    pub dom: Vec<DomNode>,
    /// CSSOM-дерево.
    pub cssom: Vec<CssNode>,
    /// Render Tree.
    pub render_tree: Vec<RenderNode>,
    /// Результат Layout.
    pub layout: Vec<LayoutBox>,
    /// События фаз рендера.
    pub phase_events: Vec<RenderPhase>,
}

impl Renderer {
    /// Создаёт новый рендер-движок.
    pub fn new() -> Self {
        Renderer::default()
    }

    /// Выполняет все фазы рендера и возвращает список фаз.
    pub fn render(&mut self, html: &str, css: &str) -> Vec<RenderPhase> {
        self.phase_events.clear();

        // 1. Parsing HTML
        self.phase_events.push(RenderPhase::ParsingHtml);
        self.dom = self.parse_html(html);

        // 2. Parsing CSS
        self.phase_events.push(RenderPhase::ParsingCss);
        self.cssom = self.parse_css(css);

        // 3. DOM
        self.phase_events.push(RenderPhase::Dom);

        // 4. CSSOM
        self.phase_events.push(RenderPhase::Cssom);

        // 5. Render Tree
        self.phase_events.push(RenderPhase::RenderTree);
        self.render_tree = self.build_render_tree(&self.dom, &self.cssom);

        // 6. Layout
        self.phase_events.push(RenderPhase::Layout);
        self.layout = self.compute_layout(&self.render_tree);

        // 7. Paint
        self.phase_events.push(RenderPhase::Paint);

        // 8. Compose
        self.phase_events.push(RenderPhase::Compose);

        self.phase_events.clone()
    }

    // ------------------------------------------------------------------
    // HTML → DOM парсер
    // ------------------------------------------------------------------

    /// Разбирает HTML-строку в дерево DOM.
    ///
    /// Поддерживает:
    /// - открывающие/закрывающие теги;
    /// - самозакрывающиеся теги (`<br/>`, `<img/>`);
    /// - атрибуты (включая беззнаковые, например `disabled`);
    /// - текстовые узлы;
    /// - комментарии `<!-- ... -->`.
    pub fn parse_html(&self, html: &str) -> Vec<DomNode> {
        let mut nodes = Vec::new();
        let mut stack: Vec<DomNode> = Vec::new();
        let chars: Vec<char> = html.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            // Комментарий
            if chars[i] == '<' && chars[i..].starts_with(&['<', '!', '-', '-'][..]) {
                if let Some(end) = find_subsequence(&chars[i..], &['-', '-', '>']) {
                    i += end + 3;
                    continue;
                }
            }

            // Открывающий тег
            if chars[i] == '<' && i + 1 < chars.len() && chars[i + 1] != '/' {
                let (tag, attrs, self_closing, consumed) = parse_open_tag(&chars[i..]);
                i += consumed;

                let node = DomNode::element(&tag, attrs);

                if self_closing || is_void_element(&tag) {
                    // Самозакрывающийся или void-элемент — добавляем в текущий родитель.
                    if let Some(parent) = stack.last_mut() {
                        parent.children.push(node);
                    } else {
                        nodes.push(node);
                    }
                } else {
                    // Открывающий тег — кладём на стек.
                    stack.push(node);
                }
                continue;
            }

            // Закрывающий тег
            if chars[i] == '<' && i + 1 < chars.len() && chars[i + 1] == '/' {
                if let Some(end) = find_subsequence(&chars[i..], &['>']) {
                    // Извлекаем имя тега между </ и >
                    let name: String = chars[i + 2..i + end].iter().collect::<String>().trim().to_string();
                    i += end + 1;

                    // Закрываем соответствующий элемент на стеке.
                    if let Some(popped) = stack.pop() {
                        if let Some(parent) = stack.last_mut() {
                            parent.children.push(popped);
                        } else {
                            nodes.push(popped);
                        }
                    }
                    let _ = name;
                }
                continue;
            }

            // Текстовый узел
            let text_start = i;
            while i < chars.len() && chars[i] != '<' {
                i += 1;
            }
            let text: String = chars[text_start..i].iter().collect();
            let trimmed = text.trim();
            if !trimmed.is_empty() {
                let text_node = DomNode::text(trimmed);
                if let Some(parent) = stack.last_mut() {
                    parent.children.push(text_node);
                } else {
                    nodes.push(text_node);
                }
            }
        }

        // Сбрасываем оставшиеся элементы стека в корень.
        while let Some(popped) = stack.pop() {
            if let Some(parent) = stack.last_mut() {
                parent.children.push(popped);
            } else {
                nodes.push(popped);
            }
        }

        nodes
    }

    // ------------------------------------------------------------------
    // CSS → CSSOM парсер
    // ------------------------------------------------------------------

    /// Разбирает CSS-строку в CSSOM.
    ///
    /// Поддерживает:
    /// - правила вида `selector { property: value; ... }`;
    /// - комментарии `/* ... */`;
    /// - несколько селекторов через запятую.
    pub fn parse_css(&self, css: &str) -> Vec<CssNode> {
        let mut rules = Vec::new();
        let mut i = 0;
        let chars: Vec<char> = css.chars().collect();

        while i < chars.len() {
            // Пропускаем комментарии
            if chars[i] == '/' && i + 1 < chars.len() && chars[i + 1] == '*' {
                if let Some(end) = find_subsequence(&chars[i..], &['*', '/']) {
                    i += end + 2;
                    continue;
                }
            }

            // Пропускаем пробелы
            if chars[i].is_whitespace() {
                i += 1;
                continue;
            }

            // Ищем открывающую скобку блока
            if let Some(open) = find_subsequence(&chars[i..], &['{']) {
                let selector: String = chars[i..i + open]
                    .iter()
                    .collect::<String>()
                    .trim()
                    .to_string();

                // Ищем закрывающую скобку
                if let Some(close) = find_subsequence(&chars[i + open..], &['}']) {
                    let body: String = chars[i + open + 1..i + open + close]
                        .iter()
                        .collect::<String>();

                    let styles = parse_css_declarations(&body);

                    // Разбиваем селекторы по запятой.
                    for sel in selector.split(',') {
                        let sel = sel.trim();
                        if !sel.is_empty() {
                            rules.push(CssNode {
                                selector: sel.to_string(),
                                styles: styles.clone(),
                            });
                        }
                    }

                    i += open + close + 1;
                    continue;
                }
            }

            // Не удалось распознать — двигаемся дальше.
            i += 1;
        }

        rules
    }

    // ------------------------------------------------------------------
    // Render Tree
    // ------------------------------------------------------------------

    /// Строит Render Tree из DOM и CSSOM.
    ///
    /// Для каждого DOM-узла применяются стили из CSSOM по селектору.
    /// Текстовые узлы (`#text`) не попадают в Render Tree.
    pub fn build_render_tree(&self, dom: &[DomNode], cssom: &[CssNode]) -> Vec<RenderNode> {
        dom.iter()
            .filter(|node| node.tag != "#text")
            .map(|node| {
                let computed = self.compute_styles(node, cssom);
                RenderNode {
                    dom_tag: node.tag.clone(),
                    computed_styles: computed,
                    children: self.build_render_tree(&node.children, cssom),
                }
            })
            .collect()
    }

    /// Вычисляет стили узла, применяя CSS-правила по селектору.
    fn compute_styles(&self, node: &DomNode, cssom: &[CssNode]) -> HashMap<String, String> {
        let mut result = HashMap::new();

        for rule in cssom {
            if selector_matches(rule.selector.as_str(), node) {
                for (k, v) in &rule.styles {
                    result.insert(k.clone(), v.clone());
                }
            }
        }

        result
    }

    // ------------------------------------------------------------------
    // Layout
    // ------------------------------------------------------------------

    /// Вычисляет геометрию (Layout) для Render Tree.
    ///
    /// Минимальная реализация: каждый узел получает размеры из стилей
    /// (width/height) или значения по умолчанию. Раскладка — блочная,
    /// узлы располагаются вертикально.
    pub fn compute_layout(&self, render_tree: &[RenderNode]) -> Vec<LayoutBox> {
        let mut y = 0.0;
        render_tree
            .iter()
            .map(|node| {
                let box_ = self.layout_node(node, 0.0, y);
                y += box_.height + 4.0; // небольшой отступ между узлами
                box_
            })
            .collect()
    }

    /// Раскладывает один узел и его детей.
    fn layout_node(&self, node: &RenderNode, x: f64, y: f64) -> LayoutBox {
        let width = parse_dimension(node.computed_styles.get("width")).unwrap_or(100.0);
        let height = parse_dimension(node.computed_styles.get("height")).unwrap_or(20.0);

        let mut child_y = y + height + 2.0;
        let mut children = Vec::new();
        for child in &node.children {
            let child_box = self.layout_node(child, x + 10.0, child_y);
            child_y += child_box.height + 2.0;
            children.push(child_box);
        }

        LayoutBox {
            tag: node.dom_tag.clone(),
            x,
            y,
            width,
            height,
            children,
        }
    }
}

// ----------------------------------------------------------------------
// Вспомогательные функции
// ----------------------------------------------------------------------

/// Ищет подпоследовательность `needle` в `haystack`, возвращает индекс начала.
fn find_subsequence(haystack: &[char], needle: &[char]) -> Option<usize> {
    if needle.is_empty() || haystack.len() < needle.len() {
        return None;
    }
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

/// Разбирает открывающий тег. Возвращает (имя, атрибуты, самозакрывающийся, потреблено символов).
fn parse_open_tag(s: &[char]) -> (String, HashMap<String, String>, bool, usize) {
    // s[0] == '<'
    let mut i = 1;
    let mut tag = String::new();

    // Имя тега
    while i < s.len() && (s[i].is_alphanumeric() || s[i] == '-' || s[i] == '_') {
        tag.push(s[i]);
        i += 1;
    }

    let mut attrs = HashMap::new();
    let mut self_closing = false;

    // Атрибуты
    while i < s.len() && s[i] != '>' {
        // Пропускаем пробелы
        while i < s.len() && s[i].is_whitespace() {
            i += 1;
        }
        if i >= s.len() || s[i] == '>' {
            break;
        }

        // Самозакрывающийся тег
        if s[i] == '/' {
            self_closing = true;
            i += 1;
            continue;
        }

        // Имя атрибута
        let mut attr_name = String::new();
        while i < s.len() && (s[i].is_alphanumeric() || s[i] == '-' || s[i] == '_' || s[i] == ':') {
            attr_name.push(s[i]);
            i += 1;
        }

        // Пропускаем пробелы перед '='
        while i < s.len() && s[i].is_whitespace() {
            i += 1;
        }

        // Значение атрибута
        if i < s.len() && s[i] == '=' {
            i += 1;
            while i < s.len() && s[i].is_whitespace() {
                i += 1;
            }
            if i < s.len() && (s[i] == '"' || s[i] == '\'') {
                let quote = s[i];
                i += 1;
                let mut value = String::new();
                while i < s.len() && s[i] != quote {
                    value.push(s[i]);
                    i += 1;
                }
                i += 1; // закрывающая кавычка
                attrs.insert(attr_name, value);
            } else {
                // Без кавычек
                let mut value = String::new();
                while i < s.len() && !s[i].is_whitespace() && s[i] != '>' {
                    value.push(s[i]);
                    i += 1;
                }
                attrs.insert(attr_name, value);
            }
        } else if !attr_name.is_empty() {
            // Атрибут без значения (например, `disabled`)
            attrs.insert(attr_name, String::new());
        }
    }

    // Пропускаем '>'
    if i < s.len() && s[i] == '>' {
        i += 1;
    }

    (tag, attrs, self_closing, i)
}

/// Возвращает true, если тег является void-элементом (не требует закрывающего тега).
fn is_void_element(tag: &str) -> bool {
    matches!(
        tag,
        "area" | "base" | "br" | "col" | "embed" | "hr" | "img" | "input"
            | "link" | "meta" | "param" | "source" | "track" | "wbr"
    )
}

/// Разбирает объявления CSS внутри блока `{ ... }`.
fn parse_css_declarations(body: &str) -> HashMap<String, String> {
    let mut styles = HashMap::new();
    for decl in body.split(';') {
        let decl = decl.trim();
        if decl.is_empty() {
            continue;
        }
        if let Some(colon) = decl.find(':') {
            let property = decl[..colon].trim().to_string();
            let value = decl[colon + 1..].trim().to_string();
            if !property.is_empty() && !value.is_empty() {
                styles.insert(property, value);
            }
        }
    }
    styles
}

/// Проверяет, соответствует ли селектор узлу DOM.
///
/// Поддерживает:
/// - тег: `div`
/// - класс: `.foo`
/// - id: `#bar`
/// - комбинации: `div.foo`, `div#bar`
fn selector_matches(selector: &str, node: &DomNode) -> bool {
    // Прямолинейный разбор селектора.
    let mut tag: Option<String> = None;
    let mut id: Option<String> = None;
    let mut class_list: Vec<String> = Vec::new();

    let mut buf = String::new();
    let mut kind = 't'; // 't' = tag, '.' = class, '#' = id
    for ch in selector.chars() {
        match ch {
            '.' | '#' => {
                if !buf.is_empty() {
                    match kind {
                        't' => tag = Some(buf.clone()),
                        '.' => class_list.push(buf.clone()),
                        '#' => id = Some(buf.clone()),
                        _ => {}
                    }
                    buf.clear();
                }
                kind = ch;
            }
            _ => buf.push(ch),
        }
    }
    if !buf.is_empty() {
        match kind {
            't' => tag = Some(buf.clone()),
            '.' => class_list.push(buf.clone()),
            '#' => id = Some(buf.clone()),
            _ => {}
        }
    }

    // Проверяем тег
    if let Some(t) = tag {
        if node.tag != t {
            return false;
        }
    }

    // Проверяем id
    if let Some(i) = id {
        if node.attributes.get("id").map(String::as_str) != Some(i.as_str()) {
            return false;
        }
    }

    // Проверяем классы
    if !class_list.is_empty() {
        let node_classes: Vec<&str> = node
            .attributes
            .get("class")
            .map(|c| c.split_whitespace().collect())
            .unwrap_or_default();
        for cls in &class_list {
            if !node_classes.contains(&cls.as_str()) {
                return false;
            }
        }
    }

    true
}

/// Разбирает CSS-размер (например, "100px", "50%") в число.
fn parse_dimension(value: Option<&String>) -> Option<f64> {
    let value = value?;
    let trimmed = value.trim();
    let num: String = trimmed
        .chars()
        .take_while(|c| c.is_ascii_digit() || *c == '.')
        .collect();
    num.parse::<f64>().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_phases() {
        let mut renderer = Renderer::new();
        let phases = renderer.render("<html><body></body></html>", "body { color: red; }");
        assert_eq!(phases.len(), 8);
        assert_eq!(phases[0], RenderPhase::ParsingHtml);
        assert_eq!(phases[7], RenderPhase::Compose);
    }

    #[test]
    fn test_phase_names() {
        assert_eq!(RenderPhase::Layout.name(), "Layout");
        assert_eq!(RenderPhase::Paint.name(), "Paint");
    }

    #[test]
    fn test_parse_html_basic() {
        let renderer = Renderer::new();
        let dom = renderer.parse_html("<html><body><div class='a'>Hello</div></body></html>");
        assert_eq!(dom.len(), 1);
        assert_eq!(dom[0].tag, "html");
        assert_eq!(dom[0].children.len(), 1);
        assert_eq!(dom[0].children[0].tag, "body");
        assert_eq!(dom[0].children[0].children.len(), 1);
        assert_eq!(dom[0].children[0].children[0].tag, "div");
        assert_eq!(
            dom[0].children[0].children[0].attributes.get("class").map(String::as_str),
            Some("a")
        );
        // Текстовый узел
        assert_eq!(dom[0].children[0].children[0].children.len(), 1);
        assert_eq!(dom[0].children[0].children[0].children[0].tag, "#text");
    }

    #[test]
    fn test_parse_html_void_elements() {
        let renderer = Renderer::new();
        let dom = renderer.parse_html("<div><br/><img src='x.png'/></div>");
        assert_eq!(dom[0].children.len(), 2);
        assert_eq!(dom[0].children[0].tag, "br");
        assert_eq!(dom[0].children[1].tag, "img");
        assert_eq!(
            dom[0].children[1].attributes.get("src").map(String::as_str),
            Some("x.png")
        );
    }

    #[test]
    fn test_parse_css_basic() {
        let renderer = Renderer::new();
        let cssom = renderer.parse_css("body { color: red; font-size: 14px; }");
        assert_eq!(cssom.len(), 1);
        assert_eq!(cssom[0].selector, "body");
        assert_eq!(cssom[0].styles.get("color").map(String::as_str), Some("red"));
        assert_eq!(
            cssom[0].styles.get("font-size").map(String::as_str),
            Some("14px")
        );
    }

    #[test]
    fn test_parse_css_multiple_selectors() {
        let renderer = Renderer::new();
        let cssom = renderer.parse_css("h1, h2 { color: blue; }");
        assert_eq!(cssom.len(), 2);
        assert_eq!(cssom[0].selector, "h1");
        assert_eq!(cssom[1].selector, "h2");
    }

    #[test]
    fn test_build_render_tree() {
        let renderer = Renderer::new();
        let dom = renderer.parse_html("<div class='box'>text</div>");
        let cssom = renderer.parse_css(".box { width: 100px; }");
        let tree = renderer.build_render_tree(&dom, &cssom);
        assert_eq!(tree.len(), 1);
        assert_eq!(tree[0].dom_tag, "div");
        assert_eq!(
            tree[0].computed_styles.get("width").map(String::as_str),
            Some("100px")
        );
        // Текстовый узел не попадает в Render Tree
        assert!(tree[0].children.is_empty());
    }

    #[test]
    fn test_compute_layout() {
        let renderer = Renderer::new();
        let dom = renderer.parse_html("<div style='width:50px;height:30px'></div>");
        let cssom = renderer.parse_css("div { width: 50px; height: 30px; }");
        let tree = renderer.build_render_tree(&dom, &cssom);
        let layout = renderer.compute_layout(&tree);
        assert_eq!(layout.len(), 1);
        assert_eq!(layout[0].width, 50.0);
        assert_eq!(layout[0].height, 30.0);
    }

    #[test]
    fn test_selector_matches() {
        let renderer = Renderer::new();
        let dom = renderer.parse_html("<div id='main' class='a b'>x</div>");
        let node = &dom[0];

        assert!(selector_matches("div", node));
        assert!(selector_matches("div.a", node));
        assert!(selector_matches("div#main", node));
        assert!(selector_matches("#main", node));
        assert!(selector_matches(".a", node));
        assert!(selector_matches(".a.b", node));
        assert!(!selector_matches("span", node));
        assert!(!selector_matches(".c", node));
        assert!(!selector_matches("#other", node));
    }
}