use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(

  name = "cli-calculator",
  about= "A command line calculator written in Rust"


)]

pub struct CommandLineArgs{

    pub op1: i32,
    pub operator : char,
    pub op2: i32,
}
