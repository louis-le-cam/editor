use editor_terminal::Color;

pub struct Theme {
    pub cursor: Color,
    pub code_background: Color,
    pub code_text: Color,
    pub code_info_background: Color,
    pub code_info_text: Color,
    pub gutter_background: Color,
    pub gutter_line: Color,
    pub gutter_current_line: Color,
    pub command_bar_background: Color,
    pub command_bar_text: Color,
    pub command_suggestion_background: Color,
    pub command_suggestion_text: Color,
}
impl Default for Theme {
    fn default() -> Self {
        let dark_gray = Color::Rgb {
            r: 28,
            g: 33,
            b: 38,
        };
        let gray = Color::Rgb {
            r: 50,
            g: 55,
            b: 60,
        };
        let light_gray = Color::Rgb {
            r: 109,
            g: 120,
            b: 133,
        };
        let white = Color::Rgb {
            r: 255,
            g: 255,
            b: 255,
        };

        Self {
            cursor: white,
            code_background: dark_gray,
            code_text: white,
            code_info_background: gray,
            code_info_text: white,
            gutter_background: dark_gray,
            gutter_line: light_gray,
            gutter_current_line: white,
            command_bar_background: dark_gray,
            command_bar_text: white,
            command_suggestion_background: gray,
            command_suggestion_text: white,
        }
    }
}
