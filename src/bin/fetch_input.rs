use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process;

const AOC_YEAR: &str = "2025";
const CONFIG_FILE: &str = ".aoc_session";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <day>", args[0]);
        eprintln!("Example: {} 1", args[0]);
        process::exit(1);
    }

    let day = match args[1].parse::<u32>() {
        Ok(d) if (1..=12).contains(&d) => d,
        _ => {
            eprintln!("Error: Day must be a number between 1 and 25");
            process::exit(1);
        }
    };

    // Read session cookie from config file
    let session = read_session_cookie();

    // Fetch input
    println!("Fetching input for day {}...", day);
    match fetch_input(day, &session) {
        Ok(input) => {
            let filename = format!("inputs/day{:02}.txt", day);
            save_input(&filename, &input);
            println!("✓ Saved to {}", filename);
        }
        Err(e) => {
            eprintln!("Error fetching input: {}", e);
            process::exit(1);
        }
    }
}

fn read_session_cookie() -> String {
    let config_path = Path::new(CONFIG_FILE);

    if !config_path.exists() {
        eprintln!("Error: Session cookie file '{}' not found!", CONFIG_FILE);
        eprintln!("\nTo create it:");
        eprintln!("1. Log in to https://adventofcode.com");
        eprintln!("2. Open browser DevTools (F12)");
        eprintln!("3. Go to Application/Storage → Cookies → https://adventofcode.com");
        eprintln!("4. Copy the value of the 'session' cookie");
        eprintln!(
            "5. Create file: echo 'your_session_cookie_here' > {}",
            CONFIG_FILE
        );
        process::exit(1);
    }

    match fs::read_to_string(config_path) {
        Ok(content) => content.trim().to_string(),
        Err(e) => {
            eprintln!("Error reading {}: {}", CONFIG_FILE, e);
            process::exit(1);
        }
    }
}

fn fetch_input(day: u32, session: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("https://adventofcode.com/{}/day/{}/input", AOC_YEAR, day);

    // Use curl command to fetch (works on macOS/Linux without dependencies)
    let output = process::Command::new("curl")
        .arg("-s") // Silent
        .arg("-f") // Fail on HTTP errors
        .arg("-H")
        .arg(format!("Cookie: session={}", session))
        .arg(&url)
        .output()?;

    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        return Err(format!(
            "Failed to fetch input. Status: {:?}\nError: {}",
            output.status.code(),
            error_msg
        )
        .into());
    }

    let input = String::from_utf8(output.stdout)?;

    if input.contains("Please log in") || input.contains("404 Not Found") {
        return Err(
            "Authentication failed or puzzle not yet available. Check your session cookie.".into(),
        );
    }

    Ok(input)
}

fn save_input(filename: &str, content: &str) {
    // Ensure inputs directory exists
    if let Some(parent) = Path::new(filename).parent() {
        fs::create_dir_all(parent).unwrap_or_else(|e| {
            eprintln!("Error creating directory: {}", e);
            process::exit(1);
        });
    }

    let mut file = fs::File::create(filename).unwrap_or_else(|e| {
        eprintln!("Error creating file {}: {}", filename, e);
        process::exit(1);
    });

    file.write_all(content.as_bytes()).unwrap_or_else(|e| {
        eprintln!("Error writing to file: {}", e);
        process::exit(1);
    });
}
