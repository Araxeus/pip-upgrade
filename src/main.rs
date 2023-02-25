use pip_upgrade::{show_outdated, update_all};
use std::io::Result;
// for each package in output, run `pip install --upgrade <package>`
fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        let arg = args[1].trim_start_matches('-');
        // remove trailing - or --
        match arg {
            "outdated" | "list" | "show" | "o" | "l" => show_outdated(),
            "version" | "v" => {
                print_version();
                Ok(())
            }
            "help" | "h" => {
                print_help();
                Ok(())
            }
            _ => {
                update_all()
            }
        }
    } else {
        update_all()
    }
}

fn print_version() {
    println!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
}

fn print_help() {
    print_version();
    println!("Usage: pip-upgrade [Command]");
    println!("Commands:");
    println!("  -h, --help\t\t\tPrint this help message");
    println!("  -o, --outdated, --list\tShow all outdated packages");
    println!("  -v, --version\t\t\tPrint the version");
}
