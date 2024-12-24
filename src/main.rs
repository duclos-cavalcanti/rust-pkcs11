use std::error::Error;

mod manager;
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    let manager = manager::Manager::new()?;

    loop {
        let prompt  = "Enter command (l = list slots, s = session, c = create, e = exit): ";
        let command = utils::io::capture(prompt)?;

        match command.as_str() {
            "l" => {
                manager.list()?;
            }

            "s" => {
                if let Err(e) = manager.session() {
                    println!("{}", utils::io::red(&e.to_string()));
                }
                break;
            }

            "e" => {
                println!("{}", utils::io::green("Exiting..."));
                break;
            }

            _ => {
                println!("{}", utils::io::red("Unknown command..."));
                break;
            }  

        }
    }

    Ok(())
}
