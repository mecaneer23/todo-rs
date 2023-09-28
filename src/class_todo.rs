#[derive(Debug)]
pub struct Todo {
    pub text: String,
    pub display_text: String,
    pub box_char: Option<char>,
    pub color: usize,
    pub indent_level: usize,
}

impl Todo {
    fn _init_box_char(text: &str, pointer: usize) -> (Option<char>, usize) {
        if text.len() > pointer && "-+".contains(text.chars().nth(pointer).unwrap()) {
            return (Some(text.chars().nth(pointer).unwrap()), pointer + 1);
        }
        return (None, pointer);
    }

    fn _init_color(text: &str, pointer: usize) -> (usize, usize) {
        if text.len() > pointer + 1
            && text.chars().nth(pointer).unwrap().is_digit(10)
            && text.chars().nth(pointer + 1).unwrap() == ' '
        {
            return (
                text.chars()
                    .nth(pointer)
                    .unwrap()
                    .to_digit(10)
                    .unwrap()
                    .try_into()
                    .unwrap(),
                pointer + 2,
            );
        }
        return (7, pointer);
    }

    fn _init_attrs(text: &str, mut pointer: usize) -> (Option<char>, usize, String) {
        let (box_char, color);
        (box_char, pointer) = Self::_init_box_char(text, pointer);
        (color, pointer) = Self::_init_color(text, pointer);
        if text.len() > pointer && text.chars().nth(pointer).unwrap() == ' ' {
            pointer += 1;
        }
        return (box_char, color, text[pointer..].to_string());
    }

    pub fn new(text: &str) -> Todo {
        let (box_char, color, display_text);
        let indent_level = text.len() - text.trim_start().len();
        if text.len() == 0 {
            box_char = Some('-');
            color = 7;
            display_text = String::from("");
        } else {
            (box_char, color, display_text) = Self::_init_attrs(text, indent_level);
        }
        Self {
            text: text.to_string(),
            display_text: display_text.to_string(),
            box_char,
            color,
            indent_level,
        }
    }
}
