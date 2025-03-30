#![crate_type = "lib"]
#![crate_name = "cpeck_text_coloring"]

/// Перечисление цветов.
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Navy,
    Purple,
    Blue,
    White,
    Clear
}

/// Функция возвращает строку с unicode escape последовательно цвета текста.
///
/// # Examples
/// ```
/// let red = cpeck_text_coloring::unicode_escape_sequences_text(cpeck_text_coloring::Color::Red);
/// assert_eq!(red, "\u{001b}[31m");
/// ```
pub fn unicode_escape_sequences_text(color: Color) -> String {
    match color {
        Color::Black => "\u{001b}[30m".to_owned(),
        Color::Red => "\u{001b}[31m".to_owned(),
        Color::Green => "\u{001b}[32m".to_owned(),
        Color::Yellow => "\u{001b}[33m".to_owned(),
        Color::Navy => "\u{001b}[34m".to_owned(),
        Color::Purple => "\u{001b}[35m".to_owned(),
        Color::Blue => "\u{001b}[36m".to_owned(),
        Color::White => "\u{001b}[37m".to_owned(),
        Color::Clear => "\u{001b}[0m".to_owned(),
    }
}

/// Функция возвращает строку с unicode escape последовательно цвета фона текста.
///
/// # Examples
/// ```
/// let red = cpeck_text_coloring::unicode_escape_sequences_backgraund(cpeck_text_coloring::Color::Red);
/// assert_eq!(red, "\u{001b}[41m");
/// ```
pub fn unicode_escape_sequences_backgraund(color: Color) -> String {
    match color {
        Color::Black => "\u{001b}[40m".to_owned(),
        Color::Red => "\u{001b}[41m".to_owned(),
        Color::Green => "\u{001b}[42m".to_owned(),
        Color::Yellow => "\u{001b}[43m".to_owned(),
        Color::Navy => "\u{001b}[44m".to_owned(),
        Color::Purple => "\u{001b}[45m".to_owned(),
        Color::Blue => "\u{001b}[46m".to_owned(),
        Color::White => "\u{001b}[47m".to_owned(),
        Color::Clear => "\u{001b}[0m".to_owned(),
    }
}

/// Функция возвращает строку переданного текста обрамленного unicode escape последовательно цвета.
fn format_color_text(str: &str, color: Color) -> String {
    format!("{}{}{}", unicode_escape_sequences_text(color), str, unicode_escape_sequences_text(Color::Clear))
}

/// Функция возвращает строку переданного текста обрамленного unicode escape последовательно черного цвета.
///
/// # Examples
/// ```
/// let black = cpeck_text_coloring::black("black");
/// assert_eq!(black, "\u{001b}[30mblack\u{001b}[0m");
/// ```
pub fn black(str: &str) -> String {
    format_color_text(str, Color::Black)
}

/// Функция возвращает строку переданного текста обрамленного unicode escape последовательно красного цвета.
///
/// # Examples
/// ```
/// let red = cpeck_text_coloring::red("red");
/// assert_eq!(red, "\u{001b}[31mred\u{001b}[0m");
/// ```
pub fn red(str: &str) -> String {
    format_color_text(str, Color::Red)
}

/// Функция возвращает строку переданного текста обрамленного unicode escape последовательно зеленого цвета.
///
/// # Examples
/// ```
/// let green = cpeck_text_coloring::green("green");
/// assert_eq!(green, "\u{001b}[32mgreen\u{001b}[0m");
/// ```
pub fn green(str: &str) -> String {
    format_color_text(str, Color::Green)
}

/// Функция возвращает строку переданного текста обрамленного unicode escape последовательно желтого цвета.
///
/// # Examples
/// ```
/// let yellow = cpeck_text_coloring::yellow("yellow");
/// assert_eq!(yellow, "\u{001b}[33myellow\u{001b}[0m");
/// ```
pub fn yellow(str: &str) -> String {
    format_color_text(str, Color::Yellow)
}

/// Функция возвращает строку переданного текста обрамленного unicode escape последовательно синего цвета.
///
/// # Examples
/// ```
/// let navy = cpeck_text_coloring::navy("navy");
/// assert_eq!(navy, "\u{001b}[34mnavy\u{001b}[0m");
/// ```
pub fn navy(str: &str) -> String {
    format_color_text(str, Color::Navy)
}

/// Функция возвращает строку переданного текста обрамленного unicode escape последовательно пурпурного цвета.
///
/// # Examples
/// ```
/// let purple = cpeck_text_coloring::purple("purple");
/// assert_eq!(purple, "\u{001b}[35mpurple\u{001b}[0m");
/// ```
pub fn purple(str: &str) -> String {
    format_color_text(str, Color::Purple)
}

/// Функция возвращает строку переданного текста обрамленного unicode escape последовательно голубого цвета.
///
/// # Examples
/// ```
/// let blue = cpeck_text_coloring::blue("blue");
/// assert_eq!(blue, "\u{001b}[36mblue\u{001b}[0m");
/// ```
pub fn blue(str: &str) -> String {
    format_color_text(str, Color::Blue)
}

/// Функция возвращает строку переданного текста обрамленного unicode escape последовательно белого цвета.
///
/// # Examples
/// ```
/// let white = cpeck_text_coloring::white("white");
/// assert_eq!(white, "\u{001b}[37mwhite\u{001b}[0m");
/// ```
pub fn white(str: &str) -> String {
    format_color_text(str, Color::White)
}

#[cfg(test)]
mod tests {
    // Обратите внимание на эту полезную идиому: импортирование имён из внешней (для mod - тестов) области видимости.
    use super::*;

    #[test]
    fn test_format_color_text() {
        let red = format_color_text("red", Color::Red);
        assert_eq!(red, "\u{001b}[31mred\u{001b}[0m");
    }
}