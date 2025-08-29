pub enum Color {
    Blue,
    Red,
    Green,
    Yellow,
    Purple,
    Cyan,
    Gray,
    White,
}

impl Color {
    pub fn get_code(&self) -> &str {
        match self {
            Color::Red => "31",
            Color::Green => "32",
            Color::Yellow => "33",
            Color::Blue => "34",
            Color::Purple => "35",
            Color::Cyan => "36",
            Color::Gray => "30",
            Color::White => "37",
        }
    }

    /// Create color type from string
    pub fn from_string(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "red" => Some(Color::Red),
            "green" => Some(Color::Green),
            "yellow" => Some(Color::Yellow),
            "blue" => Some(Color::Blue),
            "purple" => Some(Color::Purple),
            "cyan" => Some(Color::Cyan),
            "gray" | "grey" => Some(Color::Gray),
            _ => Some(Color::White),
        }
    }
}

pub struct Brush {
    pub color: Color,
}

impl Brush {
    /// Create a new brush with specified color
    ///
    /// # Arguments
    /// * `color` - A string slice representing the color name
    ///
    /// # Available colors
    /// red, green, yellow, blue, purple, cyan, gray/grey, white
    ///
    /// # Returns
    /// * `Some(Brush)` if the color is valid
    /// * `None` if the color is not recognized
    pub fn new(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "blue" => Some(Brush { color: Color::Blue }),
            "red" => Some(Brush { color: Color::Red }),
            "green" => Some(Brush {
                color: Color::Green,
            }),
            "yellow" => Some(Brush {
                color: Color::Yellow,
            }),
            "purple" => Some(Brush {
                color: Color::Purple,
            }),
            "cyan" => Some(Brush { color: Color::Cyan }),
            "gray" | "grey" => Some(Brush { color: Color::Gray }),
            "white" => Some(Brush {
                color: Color::White,
            }),
            _ => None,
        }
    }

    /// Format a given string with the color of the brush
    /// Arguments:
    /// s: string to be painted
    fn _paint(&self, s: &str) -> String {
        let code = &self.color.get_code();
        let return_string = format!("\x1B[{}m{}\x1B[0m", code, s);
        return_string
    }

    /// Format a given string with the color of the brush
    ///
    /// # Arguments
    /// * `s` - The string slice to be painted
    ///
    /// # Returns
    /// A `String` with ANSI color codes applied
    pub fn paint(&self, s: &str) -> String {
        self._paint(s)
    }

    /// Quickly format a string with a specified color
    ///
    /// # Arguments
    /// * `s` - The string slice to be painted
    /// * `color` - The color name as a string slice
    ///
    /// # Available colors
    /// red, green, yellow, blue, purple, cyan, gray/grey, white
    ///
    /// # Example
    /// ```
    /// use rustylook::Brush;
    /// let colored_text = Brush::q_paint("Hello", "red");
    /// println!("{}", colored_text);
    /// ```
    pub fn q_paint(s: &str, color: &str) -> String {
        let brush = Brush {
            color: Color::from_string(color).unwrap_or(Color::White),
        };
        brush.paint(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brush_new_valid_colors() {
        assert!(Brush::new("red").is_some());
        assert!(Brush::new("blue").is_some());
        assert!(Brush::new("green").is_some());
        assert!(Brush::new("invalid").is_none());
    }

    #[test]
    fn test_paint_contains_ansi_codes() {
        let brush = Brush::new("red").unwrap();
        let result = brush.paint("hello");
        assert!(result.contains("\x1B[31m"));
        assert!(result.contains("\x1B[0m"));
        assert!(result.contains("hello"));
    }

    #[test]
    fn test_q_paint() {
        let result = Brush::q_paint("test", "blue");
        assert!(result.contains("test"));
        assert!(result.contains("\x1B[34m"));
    }

    #[test]
    fn test_gray_spelling_variants() {
        assert!(Brush::new("gray").is_some());
        assert!(Brush::new("grey").is_some());
    }
}
