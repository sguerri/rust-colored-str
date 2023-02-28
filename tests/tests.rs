use colored_str::*;

// cargo test -- --nocapture

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

        dbg!(cformat!());
        dbg!(cformat!("<red>toto</>"));
        dbg!(cformat!("<red>{}</>", "content"));

        coloredln!("{}", "toto");
        coloredln!("{}", "toto <red>red</>");
        coloredln!("first <red>red <+on_blue>on_blue<-> red</> last");
        coloredln!("first <red>red <+on_blue>on_blue<-> red</> middle <yellow>yellow</> last");
        coloredln!("first <red>red <+on_blue>on_blue<-> red</> middle <yellow>yellow</> last");
        coloredln!("first <red>red <+on_blue>on_blue<-> between <+on_yellow>on_yellow<-> red</> last");
        coloredln!("first <red>red <+on_blue>on_blue<-> between <+on_yellow>on_yellow<-> between again <+on_green>on_green<-> red</> last");
        // println!("{}", " dsq <red>red <+on_blue>blue<-> red</> dsq".colored());
        // println!("{}", "{red}red {+bold}bold{-} none {+italic}italic{-} red{/}".colored());
        // let result = add(2, 2);
        // assert_eq!(result, 4);
    }
}
