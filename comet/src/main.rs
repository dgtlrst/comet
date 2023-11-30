mod serial;
mod ui;

fn main() {
    match serial::list_available_ports() {
        Ok(ports) => {
            if !ports.is_empty() {
                for p in ports {
                    println!("Port: {}", p.port_name);
                }
            } else {
                println!("No ports found.");
            }
        },

        Err(e) => {
            println!("{:?}", e);
        }
    }

    // Run the TUI from the ui module
    if let Err(e) = ui::run_tui() {
        eprintln!("Error: {}", e);
    }
}
