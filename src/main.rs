// use versemind::cli::commands;

// fn err_main() {

// }

fn main() {
    // Handle command-line arguments and execute the appropriate command
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Please provide a command.");
        return;
    }

    match args[1].as_str() {

        // Server
        "server up"         => commands::server_up(),
        "server down"       => commands::server_down(),

        // Data
        // "import ontology"   => commands::import_data(),

        _ => eprintln!("Invalid command."),
    }
}