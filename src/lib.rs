use std::borrow::Cow;

use regex::Regex;
use regex::Replacer;
use regex::Captures;
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
        static ref RE: Regex = Regex::new(r"\{(.+?)\}(.*?)\{/\}").unwrap();
    }
    RE.replace_all(text, rep)
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
            let colored = format!("{{{}}}{}{{/}}", style, content);
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



pub fn colored(text: &str) -> ColoredString
{
    let updated = get_styles(text, |caps: &Captures| {
        let combined = caps[1].split("+");
        let mut item = ColoredString::from(&caps[2]);
        for style in combined {
            item = test_style(style.trim())(item);
        }
        format!("{}", item)
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





#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {
        // let result = add(2, 2);
        // assert_eq!(result, 4);
    }
}
