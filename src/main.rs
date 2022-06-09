mod cli;
use structopt::StructOpt;

use cli::CommandLineArgs;
fn main() {
    
   let CommandLineArgs{
      op1,
      operator,
      op2

    } = CommandLineArgs::from_args();
  
    match operator{

       '+' => println!("{}", op1 + op2),
       '-' => println!("{}", op1 - op2),     
       '*' => println!("{}", op1 * op2),
       '/' => println!("{}", op1 / op2),
       '%' => println!("{}", op1 % op2),
        _  => println!("Invalid Opeator. Try using a valid operator"),
    };

}
