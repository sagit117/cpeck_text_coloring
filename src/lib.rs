#![crate_type = "lib"]
#![crate_name = "cpeck_text_coloring"]

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

/// Функция возвращает unicode escape последовательно цвета текста.
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

/// Функция возвращает unicode escape последовательно цвета фона текста.
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