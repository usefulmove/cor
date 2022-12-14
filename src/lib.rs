#![allow(unused)]

use colored::*;
use regex::Regex;
use std::fmt::Write as _;

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub bold: bool,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, bold: bool) -> Self {
        Self { r, g, b, bold }
    }
}

pub struct Theme {
    pub blue_coffee: Color,
    pub blue_coffee_bold: Color,
    pub blue_smurf: Color,
    pub blue_smurf_bold: Color,
    pub cream: Color,
    pub cream_bold: Color,
    pub charcoal_cream: Color,
    pub green_eggs: Color,
    pub green_eggs_bold: Color,
    pub grey_mouse: Color,
    pub orange_sherbet: Color,
    pub orange_sherbet_bold: Color,
    pub red: Color,
    pub red_bold: Color,
    pub yellow_canary: Color,
    pub yellow_canary_bold: Color,
    pub white: Color,
    pub white_bold: Color,
}

impl Theme {
    pub fn new() -> Self {
        Self {
            blue_coffee: Color {
                r: 0,
                g: 192,
                b: 255,
                bold: false,
            },
            blue_coffee_bold: Color {
                r: 0,
                g: 192,
                b: 255,
                bold: true,
            },
            blue_smurf: Color {
                r: 0,
                g: 128,
                b: 255,
                bold: false,
            },
            blue_smurf_bold: Color {
                r: 0,
                g: 128,
                b: 255,
                bold: true,
            },
            cream: Color {
                r: 250,
                g: 246,
                b: 228,
                bold: false,
            },
            cream_bold: Color {
                r: 250,
                g: 246,
                b: 228,
                bold: true,
            },
            charcoal_cream: Color {
                r: 102,
                g: 102,
                b: 102,
                bold: false,
            },
            green_eggs: Color {
                r: 135,
                g: 255,
                b: 175,
                bold: false,
            },
            green_eggs_bold: Color {
                r: 135,
                g: 255,
                b: 175,
                bold: true,
            },
            grey_mouse: Color {
                r: 115,
                g: 115,
                b: 115,
                bold: false,
            },
            orange_sherbet: Color {
                r: 239,
                g: 157,
                b: 110,
                bold: false,
            },
            orange_sherbet_bold: Color {
                r: 239,
                g: 157,
                b: 110,
                bold: true,
            },
            red: Color {
                r: 241,
                g: 95,
                b: 73,
                bold: false,
            },
            red_bold: Color {
                r: 241,
                g: 95,
                b: 73,
                bold: true,
            },
            yellow_canary: Color {
                r: 255,
                g: 252,
                b: 103,
                bold: false,
            },
            yellow_canary_bold: Color {
                r: 255,
                g: 252,
                b: 103,
                bold: true,
            },
            white: Color {
                r: 255,
                g: 255,
                b: 255,
                bold: false,
            },
            white_bold: Color {
                r: 255,
                g: 255,
                b: 255,
                bold: true,
            },
        }
    }

    pub fn color_rgb(&self, content: &str, color: &Color) -> ColoredString {
        if !color.bold {
            content.truecolor(color.r, color.g, color.b)
        } else {
            content.truecolor(color.r, color.g, color.b).bold()
        }
    }

    pub fn color_rgb_bg(&self, content: &str, color: &Color) -> ColoredString {
        let output: &str = &format!(" {} ", content); // pad output

        if !color.bold {
            output
                .truecolor(0, 0, 0)
                .on_truecolor(color.r, color.g, color.b)
        } else {
            output
                .truecolor(0, 0, 0)
                .on_truecolor(color.r, color.g, color.b)
                .bold()
        }
    }

    pub fn color_blank(&self, _message: &str) -> ColoredString {
        "".truecolor(0, 0, 0)
    }

    pub fn blue_coffee(&self, message: &str) -> ColoredString {
        self.color_rgb(message, &self.blue_coffee)
    }

    pub fn blue_coffee_bold(&self, message: &str) -> ColoredString {
        self.color_rgb(message, &self.blue_coffee_bold)
    }

    pub fn blue_smurf(&self, message: &str) -> ColoredString {
        self.color_rgb(message, &self.blue_smurf)
    }

    pub fn blue_smurf_bold(&self, message: &str) -> ColoredString {
        self.color_rgb(message, &self.blue_smurf_bold)
    }

    pub fn cream(&self, message: &str) -> ColoredString {
        self.color_rgb(message, &self.cream)
    }

    pub fn cream_bold(&self, message: &str) -> ColoredString {
        self.color_rgb(message, &self.cream_bold)
    }

    pub fn charcoal_cream(&self, message: &str) -> ColoredString {
        self.color_rgb(message, &self.charcoal_cream)
    }

    pub fn green_eggs(&self, message: &str) -> ColoredString {
        self.color_rgb(message, &self.green_eggs)
    }

    pub fn green_eggs_bold(&self, message: &str) -> ColoredString {
        self.color_rgb(message, &self.green_eggs_bold)
    }

    pub fn grey_mouse(&self, message: &str) -> ColoredString {
        self.color_rgb(message, &self.grey_mouse)
    }

    pub fn orange_sherbet(&self, message: &str) -> ColoredString {
        self.color_rgb(message, &self.orange_sherbet)
    }

    pub fn orange_sherbet_bold(&self, message: &str) -> ColoredString {
        self.color_rgb(message, &self.orange_sherbet_bold)
    }

    pub fn red(&self, message: &str) -> ColoredString {
        self.color_rgb(message, &self.red)
    }

    pub fn red_bold(&self, message: &str) -> ColoredString {
        self.color_rgb(message, &self.red_bold)
    }

    pub fn yellow_canary(&self, message: &str) -> ColoredString {
        self.color_rgb(message, &self.yellow_canary)
    }

    pub fn yellow_canary_bold(&self, message: &str) -> ColoredString {
        self.color_rgb(message, &self.yellow_canary_bold)
    }

    pub fn white(&self, message: &str) -> ColoredString {
        self.color_rgb(message, &self.white)
    }

    pub fn white_bold(&self, message: &str) -> ColoredString {
        self.color_rgb(message, &self.white_bold)
    }

}

pub fn highlight(output_str: &str, highlight_term: &str, color: &Color) -> String {
    /* find the highlight term in the output string and format the output
     * string to emphasize the highlight term in the output string
     */

    let tmp: String = output_str.to_string();
    let elements: Vec<&str> = tmp.split(&highlight_term).collect::<Vec<&str>>();

    // construct highlighted output
    let mut output: String = String::new();
    let theme = Theme::new();
    for i in 0..elements.len() {
        if i < (elements.len() - 1) {
            let _ = write!(
                output,
                "{}{}",
                theme.color_rgb(
                    elements[i],
                    &theme.grey_mouse
                ),
                theme.color_rgb(
                    highlight_term,
                    color
                ),
            );
        } else {
            let _ = write!(
                output,
                "{}",
                theme.color_rgb(
                    elements[i],
                    &theme.grey_mouse,
                ),
            );
        }
    }

    output
}

pub fn highlight_filename(output_str: &str, color: &Color) -> String {
    /* highlight everything following the last "/" */

    let re: Regex = Regex::new(r"/([^/]+)$").unwrap();

    let filename: String = match re.captures(output_str) {
        Some(n) => n[1].to_string(),
        None => String::from(""),
    };

    highlight(output_str, &filename, color)
}
