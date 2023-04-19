use std::env;
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};
use sysinfo::{System, SystemExt};
use whoami;

pub struct SystemInfo {
    pub user_host: String,
    pub os: String,
    pub kernel: String,
    pub shell: String,
    pub packages: String,
    pub cpu: String,
    pub memory: String,
}

pub fn get_system_info() -> SystemInfo {
    let system = System::new_all();
    let user_host = format!("{}@{}", whoami::username(), whoami::hostname());
    let os = format!(
        "{} {}",
        system.name().unwrap(),
        system.os_version().unwrap()
    );
    let kernel = system.kernel_version().unwrap();
    let shell = Path::new(env::var("SHELL").unwrap().as_str())
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    let packages = get_packages();
    let cpu = format!("{}", get_cpu_info());
    let memory = format!(
        "{}MiB / {}MiB",
        system.used_memory() / 1048576,
        system.total_memory() / 1048576
    );

    SystemInfo {
        user_host,
        os,
        kernel,
        shell,
        packages,
        cpu,
        memory,
    }
}

fn is_program_in_path(program: &str) -> bool {
    if let Ok(path) = env::var("PATH") {
        for p in path.split(":") {
            let p_str = format!("{}/{}", p, program);
            if fs::metadata(p_str).is_ok() {
                return true;
            }
        }
    }
    return false;
}

fn get_cpu_info() -> String {
    let mut model_name = String::from("Unknown");
    let output = Command::new("lscpu")
        .stdout(Stdio::piped())
        .output()
        .expect("failed to execute process");

    let stdout = String::from_utf8_lossy(&output.stdout);

    for line in stdout.lines() {
        if line.contains("Model name") {
            model_name = line.split(":").nth(1).unwrap().trim().to_string();
        }
    }

    return model_name;
}

fn get_packages() -> String {
    let mut output: std::process::Output;
    let native_packages: String;
    let flatpak_packages: String;
    let package_format: String;
    let mut packages = String::new();
    if is_program_in_path("rpm") {
        package_format = String::from("rpm");
        output = Command::new("rpm")
            .arg("-qa")
            .output()
            .expect("failed to execute process");
        native_packages = String::from_utf8_lossy(&output.stdout)
            .lines()
            .count()
            .to_string();
    } else if is_program_in_path("dpkg") {
        package_format = String::from("dpkg");
        output = Command::new("dpkg")
            .arg("-l")
            .output()
            .expect("failed to execute process");
        native_packages = String::from_utf8_lossy(&output.stdout)
            .lines()
            .count()
            .to_string();
    } else if is_program_in_path("pacman") {
        package_format = String::from("pacman");
        output = Command::new("pacman")
            .arg("-Q")
            .output()
            .expect("failed to execute process");
        native_packages = String::from_utf8_lossy(&output.stdout)
            .lines()
            .count()
            .to_string();
    } else {
        package_format = String::from("none");
        native_packages = String::from("0");
    }

    if package_format != "none" {
        packages = format!("{} ({})", native_packages, package_format);
    }

    if is_program_in_path("flatpak") {
        output = Command::new("flatpak")
            .arg("list")
            .output()
            .expect("failed to execute process");
        flatpak_packages = String::from_utf8_lossy(&output.stdout)
            .lines()
            .count()
            .to_string();
    } else {
        flatpak_packages = String::from("0");
    }

    if flatpak_packages != "0" {
        packages = format!("{}, {} (flatpak)", packages, flatpak_packages);
    }

    return packages;
}
