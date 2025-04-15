// tools/run.rs
use std::env;
use std::process::Command;

fn ensure_cargo_tool(name: &str) {
    println!("Checking for cargo-{name}...");
    let is_installed = match Command::new("cargo").args([name, "-V"]).status() {
        Ok(status) => {
            match status.success() {
                true => true,
                false => false,
            }
        }
        Err(_) => false,
    };
    if !is_installed {
        println!("cargo-{name} not found.");
        println!("Installing {name}...");
        let status = Command::new("cargo")
            .args(["binstall", "-y", &format!("cargo-{name}")])
            .status()
            .unwrap_or_else(|_| panic!("Failed to install {name}"));
        if !status.success() {
            panic!("Failed to install {name}");
        }
    }
}
fn ensure_tool(name: &str) {
    ensure_cargo_tool("binstall");
    if Command::new(name).arg("--version").status().is_err() {
        println!("Installing {name}...");
        Command::new("cargo")
            .args(["binstall", "-y", name])
            .status()
            .unwrap_or_else(|_| panic!("Failed to install {name}"));
    }
}
fn setup() {
    mkpaths("WORKSPACE_ROOT_DIR", None);
    mkpaths("CRATES_ROOT_DIR", None);
    mkpaths("OUT_DIR", Some(true));
    mkpaths("PLAYGROUND_DIR", Some(true));
    ensure_cargo_tool("binstall");

    ensure_tool("just");
    ensure_tool("lumen");
    ensure_tool("mdcat");
    ensure_tool("rustdoc-md");
    ensure_cargo_tool("make");
    // ensure_cargo_tool("doc");
    ensure_cargo_tool("generate");
    ensure_cargo_tool("px");
    ensure_cargo_tool("llvm-cov");
    println!("Setup complete!");
}

fn mkpaths(name: &'static str, create: Option<bool>) {
    let definedenvdir = env::var(name).unwrap();
    println!("the ${name} variable at the time of compiling was: {definedenvdir}");
    if create.is_some() {
        std::fs::create_dir_all(&definedenvdir).expect("cant create ${name}  = {definedenvdir}");
    };
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.get(1).map(|s| s.as_str()) == Some("setup") {
        setup();
        return;
    }
    if Command::new("cargo").args(["make", "-V"]).status().is_err() {
        setup();
        return;
    }
    if args.len() > 1 {
        Command::new("cargo")
            .arg("make")
            .args(&args[1..])
            .status()
            .expect("Failed to run just");
    }

    // Try to use just if available
    // if Command::new("cargo").arg("--version").status().is_ok() {
    //     Command::new("just")
    //         .args(&args[1..])
    //         .status()
    //         .expect("Failed to run just");
    // } else {
    //     // Fallback to built-in commands
    //     match args.get(1).map(|s| s.as_str()) {
    //         Some("build") => {
    //             Command::new("cargo")
    //                 .args(&["build", "--all"])
    //                 .status()
    //                 .expect("Build failed");
    //         }
    //         Some("test") => {
    //             Command::new("cargo")
    //                 .args(&["test", "--all"])
    //                 .status()
    //                 .expect("Tests failed");
    //         }
    //         _ => {
    //             println!("Available commands:");
    //             println!("  setup - Install required tools");
    //             println!("  build - Build all packages");
    //             println!("  test  - Run all tests");
    //         }
    //     }
    // }
}
