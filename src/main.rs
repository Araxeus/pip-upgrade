use pip_upgrade::{show_outdated, update_all};
use std::io::Result;
// for each package in output, run `pip install --upgrade <package>`
fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        let arg = args[1].trim_start_matches('-');
        // remove trailing - or --
        match arg {
            "update" | "upgrade" | "u" => update_all(),
            "outdated" | "list" | "show" | "o" | "l" => show_outdated(),
            "version" | "v" => {
                print_version();
                Ok(())
            }
            _ => {
                print_help();
                Ok(())
            }
        }
    } else {
        print_help();
        Ok(())
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
    println!("  -u, --update, --upgrade\tUpdate all outdated packages");
    println!("  -o, --outdated, --list\tShow all outdated packages");
    println!("  -v, --version\t\t\tPrint the version");
}
