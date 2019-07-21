use gumdrop::Options;
use lexical_bool::LexicalBool;

#[derive(Clone, Options, Debug)]
pub struct Args {
    #[options(help = "print help message")]
    pub help: bool,

    #[options(help = "display current git branch")]
    pub git_branch: bool,

    #[options(help = "attempt to make the components unique")]
    pub unique: bool,

    #[options(help = "output as json", meta = "bool", default = "true")]
    pub json: LexicalBool,

    #[options(free, required)]
    pub path: String,
}
