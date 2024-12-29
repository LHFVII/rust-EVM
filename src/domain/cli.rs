use super::evm::EVM;
use clap::{command, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    StartNode,
    ResetNode,
    Run,
    AddInstruction { instruction: String },
    SetGas { gas: i32 },
    ResetInstructions,
    PrintStack,
    AddIxsBatch,
}
pub struct CLI<'a, 'b> {
    pub node: Option<EVM<'a, 'b>>,
}

impl<'a, 'b> CLI<'a, 'b> {
    pub fn new() -> Self {
        return CLI { node: None };
    }

    pub fn run(&mut self) {
        loop {
            self.show_commands();
            let mut buf = String::new();
            std::io::stdin()
                .read_line(&mut buf)
                .expect("Couldn't parse stdin");
            let line = buf.trim();
            let mut args = vec!["program".to_string()]; // Add a dummy program name
            args.extend(shlex::split(line).ok_or("error: Invalid quoting").unwrap());
            match Args::try_parse_from(args) {
                Ok(cli) => match cli.cmd {
                    Commands::StartNode => self.start_node(),
                    Commands::AddInstruction { instruction } => self.add_instruction(instruction),
                    Commands::ResetNode => self.start_node(),
                    Commands::ResetInstructions => self.reset_instructions(),
                    Commands::Run => self.run_instructions(),
                    Commands::SetGas { gas } => self.set_gas(gas),
                    Commands::PrintStack => self.print_stack(),
                    Commands::AddIxsBatch => self.print_stack(),
                },
                Err(e) => println!("That's not a valid command! Error: {}", e),
            };
        }
    }
    fn show_commands(&mut self) {
        println!(
            r#"COMMANDS:
    1) start-node -> Start the EVM runtime.
    2) add-instruction ins -> Add an opcode to the bytecode.
    3) add-ixs-batch ins -> Add an opcode to the bytecode.
    4) reset-instructions -> Resets added opcodes.
    5) set-gas -gas -> Set the gas used by the bytecode.
    6) run -> EVM runs the program bytecode.
    7) reset-node -> Restarts EVM.
    8) print-stack -> Prints EVM stack values.
    "#
        );
    }

    fn start_node(&mut self) {
        if self.node.is_some() {
            eprintln!("Node already started...");
            return;
        }
        self.node = Some(EVM::new(Vec::new(), 0, 0, vec![]));
        println!("Running...")
    }

    fn add_instruction(&mut self, instruction: String) {
        if self.node.is_none() {
            eprintln!("Node is not started");
            return;
        }
        let evm = self.node.as_mut().unwrap();
        let bytes = u8::from_str_radix(&instruction.trim_start_matches("0x"), 16)
            .expect("Failed to parse hex");
        println!("{:?}", bytes);
        evm.add_op_code(bytes);
    }

    fn set_gas(&mut self, gas: i32) {
        if self.node.is_none() {
            eprintln!("Node is not started");
            return;
        }
        let evm = self.node.as_mut().unwrap();
        evm.set_gas_for_instruction(gas);
    }

    fn reset_instructions(&mut self) {
        if self.node.is_none() {
            eprintln!("Node is not started");
            return;
        }
        let evm = self.node.as_mut().unwrap();
        evm.reset();
    }

    fn run_instructions(&mut self) {
        if self.node.is_none() {
            eprintln!("Node is not started");
            return;
        }
        let evm = self.node.as_mut().unwrap();
        evm.run();
    }

    fn print_stack(&mut self) {
        if self.node.is_none() {
            eprintln!("Node is not started");
            return;
        }
        self.node.as_ref().unwrap().println_stack();
    }
}
