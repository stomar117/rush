#[path = "utils/processor.rs"] mod processor;
use argparse::{ArgumentParser, StoreTrue};
fn main() {
    let mut interactive = false;
    {
        let mut parser = ArgumentParser::new();
        parser.refer(&mut interactive)
            .add_option(
                &["-i", "--interactive"], 
                StoreTrue, 
                "Launch an interactive session"
            );
        parser.parse_args()
            .expect("Failed to parse arguments");
    }
    if interactive {
        let shell_process_object = processor::Process::new();
        std::process::exit(shell_process_object.interactive());
    }
    else {
        println!("Working on non interactive sessions");
        std::process::exit(0);
    }
}
