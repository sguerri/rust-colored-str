use std::borrow::Cow;

use regex::Regex;
use regex::Replacer;
use regex::Captures;
use regex::CaptureMatches;
use lazy_static::lazy_static;

pub use colored::*;

fn is_truecolor(text: &str) -> bool
{
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^#[ABCDEF\d]{6}$").unwrap();
    }
    RE.is_match(text)
}

fn is_on_truecolor(text: &str) -> bool
{
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^on_#[ABCDEF\d]{6}$").unwrap();
    }
    RE.is_match(text)
}

fn get_styles<R>(text: &str, rep: R) -> Cow<str>
    where R: Replacer
{
    lazy_static! {
        // static ref RE: Regex = Regex::new(r"\{(.+?)\}(.*?)\{/\}").unwrap();
        static ref RE: Regex = Regex::new(r"<(.+?)>(.*?)</>").unwrap();
    }
    RE.replace_all(text, rep)
}

fn iter_substyles(text: &str) -> CaptureMatches
{
    lazy_static! {
        // static ref RE: Regex = Regex::new(r"\{\+(.+?)\}(.*?)\{-\}").unwrap();
        static ref RE: Regex = Regex::new(r"<\+(.+?)>(.*?)<->").unwrap();
    }
    RE.captures_iter(text)
}



fn test_truecolor<'a>(style: &'a str, content: &ColoredString) -> Option<ColoredString>
{
    if !is_truecolor(style) {
        return None
    }
        
    let red = &style[1..3];
    let green = &style[3..5];
    let blue = &style[5..7];
    let r = u8::from_str_radix(&red, 16).expect("invalid color");
    let g = u8::from_str_radix(&green, 16).expect("invalid color");
    let b = u8::from_str_radix(&blue, 16).expect("invalid color");
    Some(content.clone().truecolor(r, g, b))
}

fn test_on_truecolor<'a>(style: &'a str, content: &ColoredString) -> Option<ColoredString>
{
    if !is_on_truecolor(style) {
        return None
    }
     
    let red = &style[4..6];
    let green = &style[6..8];
    let blue = &style[8..10];
    let r = u8::from_str_radix(&red, 16).expect("invalid color");
    let g = u8::from_str_radix(&green, 16).expect("invalid color");
    let b = u8::from_str_radix(&blue, 16).expect("invalid color");
    Some(content.clone().on_truecolor(r, g, b))
}

fn test_other<'a>(style: &'a str) -> impl Fn(ColoredString) -> ColoredString + 'a
{
    move |content: ColoredString| {
        if let Some(result) = test_truecolor(style, &content) {
            result
        } else if let Some(result) = test_on_truecolor(style, &content) {
            result
        } else {
            // let colored = format!("{{{}}}{}{{/}}", style, content);
            let colored = format!("<{}>{}</>", style, content);
            ColoredString::from(colored.as_ref())    
        }
    }
}



fn test_style<'a>(style: &'a str) -> Box<dyn Fn(ColoredString) -> ColoredString + 'a>
{
    match style.to_lowercase().as_str() {

        "black" => Box::new(&ColoredString::black),
        "red" => Box::new(&ColoredString::red),
        "green" => Box::new(&ColoredString::green),
        "yellow" => Box::new(&ColoredString::yellow),
        "blue" => Box::new(&ColoredString::blue),
        "magenta" => Box::new(&ColoredString::magenta),
        "cyan" => Box::new(&ColoredString::cyan),
        "white" => Box::new(&ColoredString::white),
        "lblack" => Box::new(&ColoredString::bright_black),
        "lred" => Box::new(&ColoredString::bright_red),
        "lgreen" => Box::new(&ColoredString::bright_green),
        "lyellow" => Box::new(&ColoredString::bright_yellow),
        "lblue" => Box::new(&ColoredString::bright_blue),
        "lmagenta" => Box::new(&ColoredString::bright_magenta),
        "lcyan" => Box::new(&ColoredString::bright_cyan),
        "lwhite" => Box::new(&ColoredString::bright_white),

        "on_black" => Box::new(&ColoredString::on_black),
        "on_red" => Box::new(&ColoredString::on_red),
        "on_green" => Box::new(&ColoredString::on_green),
        "on_yellow" => Box::new(&ColoredString::on_yellow),
        "on_blue" => Box::new(&ColoredString::on_blue),
        "on_magenta" => Box::new(&ColoredString::on_magenta),
        "on_cyan" => Box::new(&ColoredString::on_cyan),
        "on_white" => Box::new(&ColoredString::on_white),
        "on_lblack" => Box::new(&ColoredString::on_bright_black),
        "on_lred" => Box::new(&ColoredString::on_bright_red),
        "on_lgreen" => Box::new(&ColoredString::on_bright_green),
        "on_lyellow" => Box::new(&ColoredString::on_bright_yellow),
        "on_lblue" => Box::new(&ColoredString::on_bright_blue),
        "on_lmagenta" => Box::new(&ColoredString::on_bright_magenta),
        "on_lcyan" => Box::new(&ColoredString::on_bright_cyan),
        "on_lwhite" => Box::new(&ColoredString::on_bright_white),

        "bold" => Box::new(&ColoredString::bold),
        "underline" => Box::new(&ColoredString::underline),
        "italic" => Box::new(&ColoredString::italic),
        "dimmed" => Box::new(&ColoredString::dimmed),
        "reversed" => Box::new(&ColoredString::reversed),
        "blink" => Box::new(&ColoredString::blink),
        "hidden" => Box::new(&ColoredString::hidden),
        "strikethrough" => Box::new(&ColoredString::strikethrough),

        _ => Box::new(test_other(&style))
    }
}

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

fn set_style_from(text: &str, colored: &ColoredString) -> ColoredString
{
    let mut result = ColoredString::from(text);
    result = update_with_style(result, colored);
    result
}

fn add_style_from(colored_from: &ColoredString, colored_to: &ColoredString) -> ColoredString
{
    let mut result = set_style_from(&colored_to, colored_from);
    result = update_with_style(result, colored_to);
    result
}

pub fn colored(text: &str) -> ColoredString
{
    let updated = get_styles(text, |caps: &Captures| {
        let combined = caps[1].split("+");
        let mut item = ColoredString::from(&caps[2]);
        for style in combined {
            item = test_style(style.trim())(item);
        }

        // Check style variations
        let mut items: Vec<ColoredString> = vec![];
        // let mut is_first = true;
        let mut id_start = 0;
        let mut id_end = 0;

        for caps in iter_substyles(&item.clone()) {

            let range = caps.get(0).unwrap().range();
            id_end = range.end;

            // if range.start != 0 && is_first {
                // is_first = false;
                let text = &item[id_start..range.start];
                items.push(set_style_from(text, &item));
                id_start = id_end;
            // }
            
            let combined = caps[1].split("+");
            let mut subitem = ColoredString::from(&caps[2]);
            for style in combined {
                subitem = test_style(style.trim())(subitem);
            }
            subitem = add_style_from(&item, &subitem);
            items.push(subitem.clone());
        }
        if id_end != item.len() {
            let text = &item[id_end..item.len()];
            items.push(set_style_from(text, &item));
        }

        let mut res = "".to_owned();
        for i in items {
            res = format!("{}{}", res, &i);
        }
        res
    });
    ColoredString::from(updated.as_ref())
}

pub trait Colored
{
    fn colored(self) -> ColoredString;
}

impl<'a> Colored for &'a str
{
    fn colored(self) -> ColoredString
    {
        colored(self)
    }
}

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
