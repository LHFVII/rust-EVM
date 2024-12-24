use super::evm::EVM;
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
    ResetNode,
    RunInstructions,
    AddInstruction{
        instruction: u8
    },
    SetInstructionGas{
        gas: u32
    },
    ResetInstructions,
}
pub struct CLI<'a,'b,'c> {
    pub node: Option<EVM<'a,'b, 'c>>
}

impl<'a, 'b, 'c> CLI<'a, 'b, 'c>{
    pub fn new() -> Self{
        return CLI{node: None};
        
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
                        Commands::AddInstruction { instruction } => self.add_instruction(instruction),
                        Commands::ResetNode => self.start_node(),
                        Commands::ResetInstructions => self.start_node(),
                        Commands::RunInstructions => self.start_node(),
                        Commands::SetInstructionGas { gas } => self.start_node(),
                    }
                }
                Err(e) => println!("That's not a valid command! Error: {}", e),
            };
        }
    }
    fn show_commands(&mut self) {
        println!(r#"COMMANDS:
    1) Start node - Start the EVM runtime
    2) Add instructions -ins add an opcode to the bytecode
    3) Reset instructions - resets added opcodes
    4) Set gas -gas - gas used by the bytecode
    5) Run instructions - 
    6) Reset node
    "#);
    }

    fn start_node(&mut self){
        self.node = Some(EVM::new(Vec::new(), 0, 0, vec![]));
    }
    fn add_instruction(&mut self, instruction: u8){
        if self.node.is_none(){
            eprintln!("Node is not started");
            return;
        }
        let evm = self.node.as_mut().unwrap();
        evm.add_op_code(instruction);
    }
}