use colored_str::*;

// cargo test -- --nocapture

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn incorrect_assertions()
    {
        assert_eq!(colored("<>").to_string(), "<>");
        assert_eq!(colored("<+>").to_string(), "<+>");
        assert_eq!(colored("<->").to_string(), "<->");
        assert_eq!(colored("</>").to_string(), "</>");
        assert_eq!(colored("<>toto").to_string(), "<>toto");
        assert_eq!(colored("toto<>").to_string(), "toto<>");
        assert_eq!(colored("<red>").to_string(), "<red>");

        assert_eq!(colored("<red>toto").to_string(), "<red>toto");
        assert_eq!(colored("toto<red>toto").to_string(), "toto<red>toto");
        assert_eq!(colored("toto</>").to_string(), "toto</>");
        assert_eq!(colored("toto</>toto").to_string(), "toto</>toto");
        assert_eq!(colored("<red>toto</red>").to_string(), "<red>toto</red>");
        assert_eq!(colored("<unkwown>toto</>").to_string(), "<unkwown>toto</>");
        assert_eq!(colored("<on_unkwown>toto</>").to_string(), "<on_unkwown>toto</>");

        assert_eq!(colored("<+red>toto").to_string(), "<+red>toto");
        assert_eq!(colored("<+red>toto<->").to_string(), "<+red>toto<->");
    }
    
    #[test]
    fn correct_assertions()
    {
        assert_eq!(colored("<red></>").to_string(), "");

        assert_eq!(colored("<red>toto</>").to_string(), "\x1B[31mtoto\x1B[0m");
        assert_eq!(colored("<on_red>toto</>").to_string(), "\x1B[41mtoto\x1B[0m");
        assert_eq!(colored("toto<red>toto</>").to_string(), "toto\x1B[31mtoto\x1B[0m");
        assert_eq!(colored("<red>toto</>toto").to_string(), "\x1B[31mtoto\x1B[0mtoto");
        assert_eq!(colored("toto<red>toto</>toto").to_string(), "toto\x1B[31mtoto\x1B[0mtoto");

        assert_eq!(colored("<red>toto</><blue>toto</>").to_string(), "\x1B[31mtoto\x1B[0m\x1B[34mtoto\x1B[0m");
        assert_eq!(colored("toto<red>toto</><blue>toto</>").to_string(), "toto\x1B[31mtoto\x1B[0m\x1B[34mtoto\x1B[0m");
        assert_eq!(colored("<red>toto</>toto<blue>toto</>").to_string(), "\x1B[31mtoto\x1B[0mtoto\x1B[34mtoto\x1B[0m");
        assert_eq!(colored("<red>toto</><blue>toto</>toto").to_string(), "\x1B[31mtoto\x1B[0m\x1B[34mtoto\x1B[0mtoto");
        assert_eq!(colored("toto<red>toto</>toto<blue>toto</>toto").to_string(), "toto\x1B[31mtoto\x1B[0mtoto\x1B[34mtoto\x1B[0mtoto");

        assert_eq!(colored("<red+bold>toto</>").to_string(), "\x1B[1;31mtoto\x1B[0m");
        assert_eq!(colored("<bold+red>toto</>").to_string(), "\x1B[1;31mtoto\x1B[0m");
        assert_eq!(colored("<red+bold+italic>toto</>").to_string(), "\x1B[1;3;31mtoto\x1B[0m");
        assert_eq!(colored("<red+on_blue>toto</>").to_string(), "\x1B[44;31mtoto\x1B[0m");
        assert_eq!(colored("<on_blue+red>toto</>").to_string(), "\x1B[44;31mtoto\x1B[0m");
        assert_eq!(colored("<red+on_blue+bold>toto</>").to_string(), "\x1B[1;44;31mtoto\x1B[0m");

        assert_eq!(colored("<red+blue>toto</>").to_string(), "\x1B[34mtoto\x1B[0m");
        assert_eq!(colored("<blue+red>toto</>").to_string(), "\x1B[31mtoto\x1B[0m");
        assert_eq!(colored("<on_red+on_blue>toto</>").to_string(), "\x1B[44mtoto\x1B[0m");
        assert_eq!(colored("<on_blue+on_red>toto</>").to_string(), "\x1B[41mtoto\x1B[0m");

        assert_eq!(colored("<bold+bold>toto</>").to_string(), "\x1B[1mtoto\x1B[0m");
        assert_eq!(colored("<bold+italic>toto</>").to_string(), "\x1B[1;3mtoto\x1B[0m");

        assert_eq!(colored("<red>toto\ntoto</>").to_string(), "\x1B[31mtoto\ntoto\x1B[0m");
    }

    #[test]
    fn correct_assertions_subtypes()
    {
        assert_eq!(colored("<red><+bold><-></>").to_string(), "");

        assert_eq!(colored("<red>toto<+bold>toto<-></>").to_string(), "\x1B[31mtoto\x1B[0m\x1B[1;31mtoto\x1B[0m");
    }

    #[test]
    fn str_trait()
    {

    }

    #[test]
    fn str_macro()
    {

    }


    #[test]
    fn it_works() {

        let blue = "\x1B[34m";
        let reset = "\x1B[0m";
        assert_eq!(cformat!("<blue>toto</>"), format!("{blue}toto{reset}"));




        // let toto = "toto";
        // println!("{}", toto.red());
        // assert!(false)
        // assert_eq!(cformat!(), "");
        // assert_eq!(cformat!("<red>toto</>"), "\u{1b}[31mtoto\u{1b}[0m");
        // dbg!(cformat!("<red>{}</>", "content"));
        // coloredln!("{}", "toto");
        // coloredln!("{}", "toto <red>red</>");
        // coloredln!("first <red>red <+on_blue>on_blue<-> red</> last");
        // coloredln!("first <red>red <+on_blue>on_blue<-> red</> middle <yellow>yellow</> last");
        // coloredln!("first <red>red <+on_blue>on_blue<-> red</> middle <yellow>yellow</> last");
        // coloredln!("first <red>red <+on_blue>on_blue<-> between <+on_yellow>on_yellow<-> red</> last");
        // coloredln!("first <red>red <+on_blue>on_blue<-> between <+on_yellow>on_yellow<-> between again <+on_green>on_green<-> red</> last");
        // println!("{}", " dsq <red>red <+on_blue>blue<-> red</> dsq".colored());
        // println!("{}", "{red}red {+bold}bold{-} none {+italic}italic{-} red{/}".colored());
        // let result = add(2, 2);
        // assert_eq!(result, 4);
    }
}
