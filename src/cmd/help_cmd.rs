use crate::errors::*;

use crate::shell::Readline;


#[inline]
fn help(name: &str, descr: &str) {
    println!("    \x1b[32m{:13}\x1b[0m {}", name, descr);
}

pub fn run(_rl: &mut Readline, _args: &[String]) -> Result<()> {

    println!("\n\x1b[33mCOMMANDS:\x1b[0m");
    help("add",        "Add new entities to the database");
    help("delete",     "Delete entities from the database");
    help("keyring",    "Manage saved credentials");
    help("mod",        "Manage installed modules");
    help("noscope",    "Exclude entities from scope");
    help("quickstart", "Install all featured modules");
    help("run",        "Run the currently selected module");
    help("scope",      "Include entities in the scope again");
    help("select",     "Select entities from the database");
    help("target",     "Preview targeted entities or narrow them down");
    help("use",        "Select a module");
    help("workspace",  "Switch to a different workspace");
    help("help",       "Prints this message");
    println!("\nRun <command> -h for more help.\n");

    Ok(())
}