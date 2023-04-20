// mod system;

use colored::*;
use libmacchina::{GeneralReadout, KernelReadout, MemoryReadout, PackageReadout};

fn seconds_to_hms_unsafe(seconds: usize) -> (usize, usize, usize) {
    let hours = seconds / 3600;
    let remainder = seconds % 3600;
    let minutes = remainder / 60;
    let seconds_remaining = remainder % 60;

    (hours, minutes, seconds_remaining)
}

fn main() {
    use libmacchina::traits::GeneralReadout as _;
    use libmacchina::traits::KernelReadout as _;
    use libmacchina::traits::MemoryReadout as _;
    use libmacchina::traits::PackageReadout as _;
    let general_readout = GeneralReadout::new();
    let package_readout = PackageReadout::new();
    let kernel_readout = KernelReadout::new();
    let memory_readout = MemoryReadout::new();

    // USER@HOST
    let user_host = format!(
        "{}@{}",
        general_readout.username().unwrap(),
        general_readout.hostname().unwrap()
    );
    println!("{}", user_host.bold());

    for _ in 0..user_host.len() {
        print!("-");
    }
    print!("\n");

    // OS
    println!(
        "{} {}",
        "OS:".blue().bold(),
        general_readout.distribution().unwrap(),
    );

    // KERNEL
    println!(
        "{} {}",
        "Kernel:".blue().bold(),
        kernel_readout.pretty_kernel().unwrap(),
    );

    // PACKAGES
    print!("{} ", "Packages:".blue().bold());
    for pkg_format in package_readout.count_pkgs() {
        print!("{} ({})", pkg_format.1, pkg_format.0.to_string());
        if pkg_format.1 != package_readout.count_pkgs().last().unwrap().1 {
            print!(", ");
        } else {
            print!("\n");
        }
    }

    // UPTIME
    let (hours, minutes, seconds) = seconds_to_hms_unsafe(general_readout.uptime().unwrap());
    print!("{} ", "Uptime:".blue().bold(),);
    if hours > 0 {
        print!("{}h ", hours);
    }
    if minutes > 0 {
        print!("{}m ", minutes);
    }
    println!("{}s", seconds);

    // SHELL
    print!(
        "{} {}",
        "Shell:".blue().bold(),
        general_readout
            .shell(
                libmacchina::traits::ShellFormat::Relative,
                libmacchina::traits::ShellKind::Current
            )
            .unwrap(),
    );

    // RAM
    println!(
        "{} {}MiB / {}MiB",
        "Memory:".blue().bold(),
        memory_readout.used().unwrap() / 1024,
        memory_readout.total().unwrap() / 1024
    );
}
