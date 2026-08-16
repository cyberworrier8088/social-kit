

pub mod colors;
pub mod img;

use std::io;
use std::io::Write;
use std::time::Duration;
use std::thread::sleep;
use colors::*;

#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {

    clear_screen();

    println_color("Init System", BRIGHT_BLACK);
    sleep(Duration::from_millis(800));
    print_color("Bypassing security protocols...", YELLOW);
    sleep(Duration::from_millis(1200));
    println_color("Access granted.", GREEN);
    sleep(Duration::from_millis(500));

    clear_screen();


    sleep(Duration::from_secs(1));
    print_warning("CRITICAL ERROR: SYSTEM COMPROMISED!");
    sleep(Duration::from_secs(1));

    println!("{}{}{}Downloading payloads...{}", BOLD, BLINK, RED, RESET);
    for i in 1..=10 {
    
        sleep(Duration::from_millis(150));
        print!("{}[{}{}]{} ", BRIGHT_BLUE, i * 10, "%", RESET);
        io::stdout().flush().unwrap();
    }

    println!("\n");
    sleep(Duration::from_millis(600));


    println_color("Extracting browser password & history....", MAGENTA);
    sleep(Duration::from_millis(1000));
    println_color("Uploading 15.4 GB of data to remote server...", YELLOW);
    sleep(Duration::from_millis(500));
    
    for i in 0..100 {
        sleep(Duration::from_millis(15));
        let addr = 0x7FFA0000 + (i * 16);
        let dump = format!("0x{:08X}   8B 45 FC 03 45 08 89 45 F8 8B ... [ENCRYPTED]", addr);
        println_color(&dump, BRIGHT_GREEN);
    }
    
    sleep(Duration::from_secs(1));
    clear_screen();
    
    print_warning("FATAL: Commencing system wipe...");
    sleep(Duration::from_secs(1));
    println_color("Deleting C:\\Windows\\System32\\...", RED);
    for _ in 0..20 {
        sleep(Duration::from_millis(80));
        print!("{}.{}", RED, RESET);
        io::stdout().flush().unwrap();
    }
    println!("\n");
    
    print_hacker_style("Encryption Complete. Your system is mine now.");
    sleep(Duration::from_secs(2));
    
    let _ = img::gif();

    Ok(())

}