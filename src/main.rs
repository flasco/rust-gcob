mod lib;

use lib::*;

fn main() {
    match generate_branch_name() {
        Ok(name) => {
            println!("----{}", name);
            match checkout_branch(&name) {
                Ok(msg) => println!("{}", msg),
                Err(msg) => eprintln!("{}", msg),
            }
        }
        Err(e) => eprintln!("{}", e),
    }
}
