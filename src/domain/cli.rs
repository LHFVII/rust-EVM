use super::stack::Stack;
use clap::{command,Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    CreateStack,
}
pub struct CLI {
    pub stack: Option<Stack<i32>>,
}

impl CLI{
    pub fn new() -> Self{
        return CLI{stack: None};
        
    }

    pub fn run(&mut self) {
        loop {
            self.show_commands();
            let mut buf = String::new();
            std::io::stdin().read_line(&mut buf).expect("Couldn't parse stdin");
            let line = buf.trim();
            let mut args = vec!["program".to_string()]; // Add a dummy program name
            args.extend(shlex::split(line).ok_or("error: Invalid quoting").unwrap());
            match Args::try_parse_from(args) {
                Ok(cli) => {
                    match cli.cmd {
                        Commands::CreateStack => self.create_stack(),
                        
                    }
                }
                Err(e) => println!("That's not a valid command! Error: {}", e),
            };
        }
    }
    fn show_commands(&self) {
        println!(r#"COMMANDS:
    1) create-stack - It creates the EVM stack
    "#);
    }

    fn create_stack(&mut self){
        if !self.stack.is_none(){
            eprintln!("Stack already exists");
        }
        self.stack = Some(Stack::<i32>::new())
    }
}