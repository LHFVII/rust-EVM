use super::{memory::Memory, stack::Stack};
use clap::{command,Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    StartNode,
}
pub struct CLI {
    pub stack: Option<Stack<i32>>,
    pub memory: Option<Memory>
}

impl CLI{
    pub fn new() -> Self{
        return CLI{stack: None, memory: None};
        
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
                        Commands::StartNode => self.start_node(),
                        
                    }
                }
                Err(e) => println!("That's not a valid command! Error: {}", e),
            };
        }
    }
    fn show_commands(&self) {
        println!(r#"COMMANDS:
    1) start-node - It creates the EVM stack
    "#);
    }

    fn start_node(&mut self){
        if !self.stack.is_none(){
            eprintln!("Stack already exists");
            return;
        }
        self.stack = Some(Stack::<i32>::new());
        if !self.memory.is_none(){
            eprintln!("Memory already exists");
            return;
        }
        self.memory = Some(Memory::new());
    }
}