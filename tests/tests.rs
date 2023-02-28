use colored_str::*;

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

        assert_eq!(colored("<#FF000>toto</>").to_string(), "<#FF000>toto</>");
        assert_eq!(colored("<on_#FF000>toto</>").to_string(), "<on_#FF000>toto</>");

        assert_eq!(colored("<+red>toto").to_string(), "<+red>toto");
        assert_eq!(colored("<+red>toto<->").to_string(), "<+red>toto<->");
        assert_eq!(colored("<+bold><->").to_string(), "<+bold><->");
        assert_eq!(colored("<red><+blue>toto</>").to_string(), "\x1B[31m<+blue>toto\x1B[0m");
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

        assert_eq!(colored("<#FF0000>toto</>").to_string(), "\x1B[38;2;255;0;0mtoto\x1B[0m");
        assert_eq!(colored("<on_#FF0000>toto</>").to_string(), "\x1B[48;2;255;0;0mtoto\x1B[0m");

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

        assert_eq!(colored("<red><+bold>toto<-></>").to_string(), "\x1B[1;31mtoto\x1B[0m");
        assert_eq!(colored("<red><+blue>toto<-></>").to_string(), "\x1B[34mtoto\x1B[0m");
        assert_eq!(colored("<red><+bold+on_blue>toto<-></>").to_string(), "\x1B[1;44;31mtoto\x1B[0m");

        assert_eq!(colored("<red>toto<+bold>toto<-></>").to_string(), "\x1B[31mtoto\x1B[0m\x1B[1;31mtoto\x1B[0m");
        assert_eq!(colored("<red><+bold>toto<->toto</>").to_string(), "\x1B[1;31mtoto\x1B[0m\x1B[31mtoto\x1B[0m");
        assert_eq!(colored("<red>toto<+bold>toto<->toto</>").to_string(), "\x1B[31mtoto\x1B[0m\x1B[1;31mtoto\x1B[0m\x1B[31mtoto\x1B[0m");
        assert_eq!(colored("toto<red>toto<+bold>toto<->toto</>").to_string(), "toto\x1B[31mtoto\x1B[0m\x1B[1;31mtoto\x1B[0m\x1B[31mtoto\x1B[0m");
        assert_eq!(colored("<red>toto<+bold>toto<->toto</>toto").to_string(), "\x1B[31mtoto\x1B[0m\x1B[1;31mtoto\x1B[0m\x1B[31mtoto\x1B[0mtoto");
        assert_eq!(colored("toto<red>toto<+bold>toto<->toto</>toto").to_string(), "toto\x1B[31mtoto\x1B[0m\x1B[1;31mtoto\x1B[0m\x1B[31mtoto\x1B[0mtoto");

        assert_eq!(colored("<red>toto<+bold>toto<-><+italic>toto<->toto</>").to_string(), "\x1B[31mtoto\x1B[0m\x1B[1;31mtoto\x1B[0m\x1B[3;31mtoto\x1B[0m\x1B[31mtoto\x1B[0m");
        assert_eq!(colored("<red>toto<+bold>toto<->toto<+italic>toto<->toto</>").to_string(), "\x1B[31mtoto\x1B[0m\x1B[1;31mtoto\x1B[0m\x1B[31mtoto\x1B[0m\x1B[3;31mtoto\x1B[0m\x1B[31mtoto\x1B[0m");

        assert_eq!(colored("<red><+bold>toto\ntoto<-></>").to_string(), "\x1B[1;31mtoto\ntoto\x1B[0m");
    }

    #[test]
    fn str_trait()
    {
        assert_eq!("<red></>".colored().to_string(), "");
        assert_eq!("<red>toto</>".colored().to_string(), "\x1B[31mtoto\x1B[0m");
        assert_eq!("<#FF0000>toto</>".colored().to_string(), "\x1B[38;2;255;0;0mtoto\x1B[0m");
        assert_eq!("<on_#FF0000>toto</>".colored().to_string(), "\x1B[48;2;255;0;0mtoto\x1B[0m");
        assert_eq!("<red>toto</><blue>toto</>".colored().to_string(), "\x1B[31mtoto\x1B[0m\x1B[34mtoto\x1B[0m");
        assert_eq!("<red+bold>toto</>".colored().to_string(), "\x1B[1;31mtoto\x1B[0m");
        assert_eq!("<red+blue>toto</>".colored().to_string(), "\x1B[34mtoto\x1B[0m");
        assert_eq!("<bold+bold>toto</>".colored().to_string(), "\x1B[1mtoto\x1B[0m");
        assert_eq!("<red>toto\ntoto</>".colored().to_string(), "\x1B[31mtoto\ntoto\x1B[0m");

        assert_eq!("<red><+bold><-></>".colored().to_string(), "");
        assert_eq!("<red><+bold>toto<-></>".colored().to_string(), "\x1B[1;31mtoto\x1B[0m");
        assert_eq!("<red>toto<+bold>toto<-></>".colored().to_string(), "\x1B[31mtoto\x1B[0m\x1B[1;31mtoto\x1B[0m");
        assert_eq!("<red>toto<+bold>toto<-><+italic>toto<->toto</>".colored().to_string(), "\x1B[31mtoto\x1B[0m\x1B[1;31mtoto\x1B[0m\x1B[3;31mtoto\x1B[0m\x1B[31mtoto\x1B[0m");
        assert_eq!("<red><+bold>toto\ntoto<-></>".colored().to_string(), "\x1B[1;31mtoto\ntoto\x1B[0m");
    }

    #[test]
    fn str_macro()
    {
        let toto = "toto";

        assert_eq!(cformat!(), "");
        assert_eq!(cformat!("<red>{toto}</>"), "\x1B[31mtoto\x1B[0m");
        assert_eq!(cformat!("<red>{}</>", toto), "\x1B[31mtoto\x1B[0m");

        assert_eq!(cformat!("<#FF0000>{toto}</>"), "\x1B[38;2;255;0;0mtoto\x1B[0m");
        assert_eq!(cformat!("<on_#FF0000>{toto}</>"), "\x1B[48;2;255;0;0mtoto\x1B[0m");
        assert_eq!(cformat!("<red>{toto}</><blue>{toto}</>"), "\x1B[31mtoto\x1B[0m\x1B[34mtoto\x1B[0m");
        assert_eq!(cformat!("<red+bold>{toto}</>"), "\x1B[1;31mtoto\x1B[0m");
        assert_eq!(cformat!("<red+blue>{toto}</>"), "\x1B[34mtoto\x1B[0m");
        assert_eq!(cformat!("<bold+bold>{toto}</>"), "\x1B[1mtoto\x1B[0m");
        assert_eq!(cformat!("<red>{toto}\n{toto}</>"), "\x1B[31mtoto\ntoto\x1B[0m");

        assert_eq!(cformat!("<red><+bold>{toto}<-></>"), "\x1B[1;31mtoto\x1B[0m");
        assert_eq!(cformat!("<red>{toto}<+bold>{toto}<-></>"), "\x1B[31mtoto\x1B[0m\x1B[1;31mtoto\x1B[0m");
        assert_eq!(cformat!("<red>{toto}<+bold>{toto}<-><+italic>{toto}<->{toto}</>"), "\x1B[31mtoto\x1B[0m\x1B[1;31mtoto\x1B[0m\x1B[3;31mtoto\x1B[0m\x1B[31mtoto\x1B[0m");
        assert_eq!(cformat!("<red><+bold>{toto}\n{toto}<-></>"), "\x1B[1;31mtoto\ntoto\x1B[0m");
    }

}
