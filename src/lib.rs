pub enum Color {
    Blue,
    Red,
    Green,
    Yellow,
    Purple,
    Cyan,
    Gray,
    White
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
            Color::White => "37"
        }
    }

    /// Create color type from string
    pub fn from_str(s: &str) -> Option<Self> {
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
            "white" => Some(Brush { color: Color::White }),
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
            color: Color::from_str(color).unwrap_or(Color::White),
        };
        brush.paint(s)
    }
}
