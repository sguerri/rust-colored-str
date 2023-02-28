// Copyright (C) 2023 Sebastien Guerri
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

#![warn(missing_docs)]

//! Coloring terminal by parsing string content
//! 
//! This crate is an extension to the [colored](https://crates.io/crates/colored) crate
//! that enables terminal coloring. It provides a trait and a macro to parse a given string
//! that incorporates style flags.
//! 
//! ## Usage
//! 
//! This crate is [on crates.io](https://crates.io/crates/colored-str) and can be used
//! by adding `colored-str` to your dependencies in your project's `Cargo.toml`.
//! 
//! ```toml
//! [dependencies]
//! colored-str = "0.0.1"
//! ```
//! 
//! ## How to use
//! 
//! Styles must be written within `<...>` opening flag and `</>` closing flag.
//! 
//! Style variations must be written within `<+...>` opening flag and `<->` closing flag.
//! 
//! See below examples.
//! 
//! ## Limitations
//! 
//! **Blocks cannot be overlapped**  
//! Such code `<red> ... <blue> ... </> ... </>` will not work properly.  
//! This is true as well for variations : `<red><+blue> ... <+bold> ... <-><-></>` will not work properly.  
//! 
//! **A style cannot be removed**  
//! With `<red+bold> ... here I want to keep red only => impossible </>`.  
//! The workaround is as follows: `<red> <+bold> ... <-> here I have red only </>`
//! 
//! ## Examples
//! 
//! ```
//! use colored_str::coloredln;
//! 
//! coloredln!("<red>this is red</>");
//! coloredln!("<#FF0000>this is also red</>");
//! coloredln!("<blue+red>this is red again</>");
//! coloredln!("<red+on_blue>this is red on blue</>");
//! coloredln!("<red+on_#0000FF>this is also red on blue</>");
//! coloredln!("<bold>this is bold</>");
//! coloredln!("<red>there is a first line\nthen a second</>");
//! ```
//! 
//! You can add variables as per [`println!`]
//! 
//! ```
//! use colored_str::coloredln;
//! 
//! let message = "this is red";
//! coloredln!("<red>{message}</>");
//! coloredln!("<red>{}</>", message);
//! ```
//! 
//! You can add styles adjustments in a block
//! 
//! ```
//! use colored_str::coloredln;
//! 
//! coloredln!("<red>this is red <+bold>this is red and bold<-> then red again </>");
//! coloredln!("<red>this is red <+bold+on_blue>this is red on blue and bold<-> then red again </>");
//! ```
//! 
//! You can also use it as a trait
//! 
//! ```
//! use colored_str::Colored;
//! let s: String = "<red>this is red</>".colored().to_string();
//! println!("{}", s);
//! ```
//! 
//! ## List of styles
//! 
//! ### Colors
//! 
//! - `black`
//! - `red`
//! - `green`
//! - `yellow`
//! - `blue`
//! - `magenta`
//! - `purple`
//! - `cyan`
//! - `white`
//! 
//! All can be used as backgound using `on_` prefix.
//! 
//! ### Light/Bright Colors
//! 
//! - `lblack`
//! - `lred`
//! - `lgreen`
//! - `lyellow`
//! - `lblue`
//! - `lmagenta`
//! - `lpurple`
//! - `lcyan`
//! - `lwhite`
//! 
//! All can be used as backgound using `on_` prefix.
//! 
//! ### Decorations
//! 
//! - `bold`
//! - `underline`
//! - `italic`
//! - `dimmed`
//! - `reverse`
//! - `reversed`
//! - `blink`
//! - `hidden`
//! - `strikethrough`
//! 
//! ### True colors
//! 
//! - `#RRGGBB`
//! - `on_#RRGGBB`
//! 

use std::borrow::Cow;

use regex::Regex;
use regex::Replacer;
use regex::Captures;
use regex::CaptureMatches;
use lazy_static::lazy_static;

use colored::*;

/// Regex to check truecolor foreground format
fn is_truecolor(text: &str) -> bool
{
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^#[ABCDEF\d]{6}$").unwrap();
    }
    RE.is_match(text)
}

/// Regex to check truecolor background format
fn is_on_truecolor(text: &str) -> bool
{
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^on_#[ABCDEF\d]{6}$").unwrap();
    }
    RE.is_match(text)
}

/// Regex to retrieve all styled blocks
fn get_styles<R>(text: &str, rep: R) -> Cow<str>
    where R: Replacer
{
    lazy_static! {
        static ref RE: Regex = Regex::new(r"<(.+?)>((.|\n)*?)</>").unwrap();
    }
    RE.replace_all(text, rep)
}

/// Regex to retrieve all style modification subblocks
fn iter_substyles(text: &str) -> CaptureMatches
{
    lazy_static! {
        static ref RE: Regex = Regex::new(r"<\+(.+?)>((.|\n)*?)<->").unwrap();
    }
    RE.captures_iter(text)
}

/// If style is truecolor foreground, apply style
fn test_truecolor(style: &'_ str, content: &ColoredString) -> Option<ColoredString>
{
    if !is_truecolor(style) {
        return None
    }
        
    let red = &style[1..3];
    let green = &style[3..5];
    let blue = &style[5..7];
    let r = u8::from_str_radix(red, 16).expect("invalid color");
    let g = u8::from_str_radix(green, 16).expect("invalid color");
    let b = u8::from_str_radix(blue, 16).expect("invalid color");
    Some(content.clone().truecolor(r, g, b))
}

/// If style is backcolor foreground, apply style
fn test_on_truecolor(style: &'_ str, content: &ColoredString) -> Option<ColoredString>
{
    if !is_on_truecolor(style) {
        return None
    }
     
    let red = &style[4..6];
    let green = &style[6..8];
    let blue = &style[8..10];
    let r = u8::from_str_radix(red, 16).expect("invalid color");
    let g = u8::from_str_radix(green, 16).expect("invalid color");
    let b = u8::from_str_radix(blue, 16).expect("invalid color");
    Some(content.clone().on_truecolor(r, g, b))
}

/// Returns the function to apply in case generic ColoredString function does not apply
fn test_other(style: &'_ str) -> impl Fn(ColoredString) -> ColoredString + '_
{
    move |content: ColoredString| {
        if let Some(result) = test_truecolor(style, &content) {
            result
        } else if let Some(result) = test_on_truecolor(style, &content) {
            result
        } else {
            let colored = format!("<{}>{}</>", style, content);
            ColoredString::from(colored.as_ref())    
        }
    }
}

/// Returns the function to apply in case generic ColoredString function exists
fn test_style<'a>(style: &'a str) -> Box<dyn Fn(ColoredString) -> ColoredString + 'a>
{
    match style.to_lowercase().as_str() {

        "black" => Box::new(&ColoredString::black),
        "red" => Box::new(&ColoredString::red),
        "green" => Box::new(&ColoredString::green),
        "yellow" => Box::new(&ColoredString::yellow),
        "blue" => Box::new(&ColoredString::blue),
        "magenta" => Box::new(&ColoredString::magenta),
        "purple" => Box::new(&ColoredString::purple),
        "cyan" => Box::new(&ColoredString::cyan),
        "white" => Box::new(&ColoredString::white),
        "lblack" => Box::new(&ColoredString::bright_black),
        "lred" => Box::new(&ColoredString::bright_red),
        "lgreen" => Box::new(&ColoredString::bright_green),
        "lyellow" => Box::new(&ColoredString::bright_yellow),
        "lblue" => Box::new(&ColoredString::bright_blue),
        "lmagenta" => Box::new(&ColoredString::bright_magenta),
        "lpurple" => Box::new(&ColoredString::bright_purple),
        "lcyan" => Box::new(&ColoredString::bright_cyan),
        "lwhite" => Box::new(&ColoredString::bright_white),

        "on_black" => Box::new(&ColoredString::on_black),
        "on_red" => Box::new(&ColoredString::on_red),
        "on_green" => Box::new(&ColoredString::on_green),
        "on_yellow" => Box::new(&ColoredString::on_yellow),
        "on_blue" => Box::new(&ColoredString::on_blue),
        "on_magenta" => Box::new(&ColoredString::on_magenta),
        "on_purple" => Box::new(&ColoredString::on_purple),
        "on_cyan" => Box::new(&ColoredString::on_cyan),
        "on_white" => Box::new(&ColoredString::on_white),
        "on_lblack" => Box::new(&ColoredString::on_bright_black),
        "on_lred" => Box::new(&ColoredString::on_bright_red),
        "on_lgreen" => Box::new(&ColoredString::on_bright_green),
        "on_lyellow" => Box::new(&ColoredString::on_bright_yellow),
        "on_lblue" => Box::new(&ColoredString::on_bright_blue),
        "on_lmagenta" => Box::new(&ColoredString::on_bright_magenta),
        "on_lpurple" => Box::new(&ColoredString::on_bright_purple),
        "on_lcyan" => Box::new(&ColoredString::on_bright_cyan),
        "on_lwhite" => Box::new(&ColoredString::on_bright_white),

        "bold" => Box::new(&ColoredString::bold),
        "underline" => Box::new(&ColoredString::underline),
        "italic" => Box::new(&ColoredString::italic),
        "dimmed" => Box::new(&ColoredString::dimmed),
        "reverse" => Box::new(&ColoredString::reverse),
        "reversed" => Box::new(&ColoredString::reversed),
        "blink" => Box::new(&ColoredString::blink),
        "hidden" => Box::new(&ColoredString::hidden),
        "strikethrough" => Box::new(&ColoredString::strikethrough),

        _ => Box::new(test_other(style))
    }
}

/// Overwrite the style of first entry with the style of second entry
fn update_with_style(text: ColoredString, colored: &ColoredString) -> ColoredString
{
    let mut result = text;
    if let Some(fgcolor) = colored.fgcolor() {
        result = result.color(fgcolor);
    }
    if let Some(bgcolor) = colored.bgcolor() {
        result = result.on_color(bgcolor);
    }
    if colored.style().contains(Styles::Bold) { result = result.bold(); }
    if colored.style().contains(Styles::Underline) { result = result.underline(); }
    if colored.style().contains(Styles::Italic) { result = result.italic(); }
    if colored.style().contains(Styles::Dimmed) { result = result.dimmed(); }
    if colored.style().contains(Styles::Reversed) { result = result.reverse(); }
    if colored.style().contains(Styles::Blink) { result = result.blink(); }
    if colored.style().contains(Styles::Hidden) { result = result.hidden(); }
    if colored.style().contains(Styles::Strikethrough) { result = result.strikethrough(); }
    result
}

/// Add a given ColoredString style to an unstyled text
fn set_style_from(text: &str, colored: &ColoredString) -> ColoredString
{
    let mut result = ColoredString::from(text);
    result = update_with_style(result, colored);
    result
}

/// Add a given ColoredString style to an already styled text (overwrite content)
fn add_style_from(colored_from: &ColoredString, colored_to: &ColoredString) -> ColoredString
{
    let mut result = set_style_from(colored_to, colored_from);
    result = update_with_style(result, colored_to);
    result
}












/// Creates a new [`ColoredString`][1] by parsing given text.
///
/// It will parse the given text, searching for `<...> * </>` blocks and `<+...> * <->`
/// subblocks, to add corresponding styles to the text. It then returns a new
/// instance of [`ColoredString`][1].
///
/// [1]: <https://docs.rs/colored/latest/colored/struct.ColoredString.html>
/// 
/// # Examples
///
/// Basic usage:
///
/// ```
/// use colored_str::colored;
/// 
/// println!("{}", colored("<red>this is red text</red>"));
/// ```
/// 
/// See [crate] for other examples
pub fn colored(text: &str) -> ColoredString
{
    let updated = get_styles(text, |caps: &Captures| {

        // Create main styled item based on capture
        let combined = caps[1].split('+');
        let mut item = ColoredString::from(&caps[2]);
        for style in combined {
            item = test_style(style.trim())(item);
        }

        let mut items: Vec<ColoredString> = vec![];
        let mut id_start = 0;
        let mut id_end = 0;

        // Add substyles
        for caps in iter_substyles(&item) {

            let range = caps.get(0).unwrap().range();
            id_end = range.end;

            // Add styled item before the capture if text is not empty
            if id_start != range.start {
                let text = &item[id_start..range.start];
                items.push(set_style_from(text, &item));
                id_start = id_end;
            }

            // Add substyled item if not empty
            if !&caps[2].is_empty() {
                let combined = caps[1].split('+');
                let mut subitem = ColoredString::from(&caps[2]);
                for style in combined {
                    subitem = test_style(style.trim())(subitem);
                }
                subitem = add_style_from(&item, &subitem);
                items.push(subitem.clone());
            }
        }

        // Add styled item after all captures (end of the string)
        if id_end != item.len() {
            let text = &item[id_end..item.len()];
            items.push(set_style_from(text, &item));
        }

        // Join all items
        let mut res = "".to_owned();
        for i in items {
            res = format!("{}{}", res, &i);
        }
        res
    });

    ColoredString::from(updated.as_ref())
}

/// The trait that enables a string to be colorized
pub trait Colored
{
    /// Creates a new [`ColoredString`][1] by parsing given text.
    /// 
    /// [1]: <https://docs.rs/colored/latest/colored/struct.ColoredString.html>
    fn colored(self) -> ColoredString;
}

impl<'a> Colored for &'a str
{
    fn colored(self) -> ColoredString
    {
        colored(self)
    }
}

/// Creates a new [`String`] by parsing given text.
///
/// With nothing given returns an empty [`String`].  
/// Otherwise format given parameters using `format!` macro then apply [`colored()`].
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use colored_str::cformat;
/// 
/// println!("{}", cformat!("<red>this is red text</red>"));
/// println!("{}", cformat!("<red>this is {} text</red>", "red"));
/// ```
/// 
/// See [crate] for other examples
#[macro_export]
macro_rules! cformat {
    () => {
        String::from("")
    };
    ($top:tt) => ({
        let msg = format!($top);
        $crate::colored(&msg).to_string()
    });
    ($top:tt, $($arg:tt)*) => ({
        let msg = format!($top, $($arg)*);
        $crate::colored(&msg).to_string()
    });
}

/// Print colored text to standard output.
///
/// With nothing given does nothing.  
/// Otherwise format given parameters using `format!` macro, apply [`colored()`], then [`print!`] to standard output.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use colored_str::colored;
/// 
/// colored!("<red>this is red text</red>");
/// colored!("<red>this is {} text</red>", "red");
/// ```
/// 
/// See [crate] for other examples
#[macro_export]
macro_rules! colored {
    () => {
        print!()
    };
    ($top:tt) => {
        let msg = format!($top);
        print!("{}", $crate::colored(&msg));
    };
    ($top:tt, $($arg:tt)*) => {
        let msg = format!($top, $($arg)*);
        print!("{}", $crate::colored(&msg));
    };
}


/// Print colored text to standard output with newline at the end.
///
/// With nothing given does nothing.  
/// Otherwise format given parameters using `format!` macro, apply [`colored()`], then [`println!`] to standard output.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use colored_str::coloredln;
/// 
/// coloredln!("<red>this is red text</red>");
/// coloredln!("<red>this is {} text</red>", "red");
/// ```
/// 
/// See [crate] for other examples
#[macro_export]
macro_rules! coloredln {
    () => {
        println!()
    };
    ($top:tt) => {
        let msg = format!($top);
        println!("{}", $crate::colored(&msg));
    };
    ($top:tt, $($arg:tt)*) => {
        let msg = format!($top, $($arg)*);
        println!("{}", $crate::colored(&msg));
    };
}
