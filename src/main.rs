mod system;

use colored::*;

fn main() {
    let system = system::get_system_info();

    println!("{}", system.user_host.bold());

    for _ in 0..system.user_host.len() {
        print!("-");
    }
    print!("\n");

    // OS
    println!(
        "{} {}",
        "OS:".blue().bold(),
        system.os,
    );

    // KERNEL
    println!(
        "{} {}",
        "Kernel:".blue().bold(),
        system.kernel,
    );

    // SHELL
    println!(
        "{} {}",
        "Shell:".blue().bold(),
        system.shell,
    );

    // PACKAGES
    println!(
        "{} {}",
        "Packages:".blue().bold(),
        system.packages,
    );
    
    // CPU
    println!(
        "{} {}",
        "CPU:".blue().bold(),
        system.cpu,
    );

    // RAM
    println!("{} {}", "Memory:".blue().bold(), system.memory);
}
