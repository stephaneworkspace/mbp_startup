use std::io::{stdout, Write};
use curl::easy::Easy;

fn execute_startup_command() {
    let process = std::process::Command::new("sudo")
    .arg("/Users/stephane/mbp_startup.sh")
        .output()
        .ok()
        .expect("Failed to execute");
    let out = std::string::String::from_utf8(process.stdout)
        .ok()
        .expect("Failed to read");
    println!("{}", out);
}

fn main() {
    // Script startup
    execute_startup_command();

    // Check startup
    let process = std::process::Command::new("/Users/stephane/Backup/vps/ssl-cert-check")
        .arg("-s")
        .arg("stephane-bressani.ch")
        .arg("-p")
        .arg("443")
        .output()
        .ok()
        .expect("Failed to execute");
    let out = std::string::String::from_utf8(process.stdout)
        .ok()
        .expect("Failed to read");
    println!("{}", out);

    let process = std::process::Command::new("/Users/stephane/Backup/vps/ssl-cert-check")
        .arg("-s")
        .arg("www.stephane-bressani.ch")
        .arg("-p")
        .arg("443")
        .output()
        .ok()
        .expect("Failed to execute");
    let out = std::string::String::from_utf8(process.stdout)
        .ok()
        .expect("Failed to read");
    println!("{}", out);

    let process = std::process::Command::new("/Users/stephane/Backup/vps/ssl-cert-check")
        .arg("-s")
        .arg("bressart.ch")
        .arg("-p")
        .arg("443")
        .output()
        .ok()
        .expect("Failed to execute");
    let out = std::string::String::from_utf8(process.stdout)
        .ok()
        .expect("Failed to read");
    println!("{}", out);

    let process = std::process::Command::new("/Users/stephane/Backup/vps/ssl-cert-check")
        .arg("-s")
        .arg("www.bressart.ch")
        .arg("-p")
        .arg("443")
        .output()
        .ok()
        .expect("Failed to execute");
    let out = std::string::String::from_utf8(process.stdout)
        .ok()
        .expect("Failed to read");
    println!("{}", out);

    println!("https://bressart.ch/api/test");
    let mut easy = Easy::new();
    easy.url("https://bressart.ch/api/test").unwrap();
    easy.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    }).unwrap();
    easy.perform().unwrap();
    println!("\n{}", easy.response_code().unwrap());

    println!("https://www.bressart.ch/api/test");
    let mut easy = Easy::new();
    easy.url("https://www.bressart.ch/api/test").unwrap();
    easy.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    }).unwrap();
    easy.perform().unwrap();
    println!("\n{}", easy.response_code().unwrap());

    let process = std::process::Command::new("/Users/stephane/Backup/vps/ssl-cert-check")
        .arg("-s")
        .arg("astrologie.bressani.dev")
        .arg("-p")
        .arg("443")
        .output()
        .ok()
        .expect("Failed to execute");
    let out = std::string::String::from_utf8(process.stdout)
        .ok()
        .expect("Failed to read");
    println!("{}", out);

    let process = std::process::Command::new("/Users/stephane/Backup/vps/ssl-cert-check")
        .arg("-s")
        .arg("ukulele.stephane-bressani.ch")
        .arg("-p")
        .arg("443")
        .output()
        .ok()
        .expect("Failed to execute");
    let out = std::string::String::from_utf8(process.stdout)
        .ok()
        .expect("Failed to read");
    println!("{}", out);
}