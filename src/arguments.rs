use clap::{builder, Parser};

#[derive(Debug, Parser)]
#[command(name = "c-letter")]
#[command(author, version, about, long_about = None)]
pub struct Arguments {
    /// The person whom send it
    #[arg(short, long, default_value = "Dear Hiring Manager")]
    pub whom: String,

    /// Name of the Company
    #[arg(short, long, value_parser = builder::NonEmptyStringValueParser::new())]
    pub company: String,

    /// Name of the Position
    #[arg(short, long, value_parser = builder::NonEmptyStringValueParser::new())]
    pub position: String,

    /// The original File that will used for
    #[arg(short, long, value_parser = builder::NonEmptyStringValueParser::new())]
    pub file: String,
}
