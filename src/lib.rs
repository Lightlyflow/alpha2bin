use std::error::Error;

pub struct Config {
    pub query_string: String,
    pub bin_to_alpha: bool,
    pub quiet: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 1 {
            return Err("Not enough arguments!");
        }

        // Default values
        let mut query_string = "".to_string();
        let mut bin_to_alpha = false;
        let mut quiet = false;

        // Parse cli args
        for (i, arg) in args.iter().enumerate() {
            if i == 0 { continue; }
            else if arg == "-r" { bin_to_alpha = true; }
            else if arg == "-q" { quiet = true; }
            else { query_string = arg.clone(); }
        }

        Ok(Config {
            query_string,
            bin_to_alpha,
            quiet,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let mut mode = "None".to_string();

    // Get output
    if config.bin_to_alpha {
        mode = "binary representation to string".to_string();
        output = bin_to_alpha(&config.query_string)?;
    } else {
        mode = "string to binary".to_string();
        output = str_to_bin(&config.query_string)?;
    }

    if !config.quiet { println!("Mode: {mode}"); }
    println!("{output}");

    Ok(())
}

fn str_to_bin(alpha_string: &String) -> Result<String, Box<dyn Error>> {
    let mut output: String = "".to_string();

    for char in alpha_string.to_string().into_bytes() {
        output += &format!("{:08b} ", char);
    }

    Ok(output)
}

fn bin_to_alpha(bin_string: &String) -> Result<String, Box<dyn Error>> {
    let vec_u8 = (0..bin_string.len())
                .step_by(9)
                .map(|i| u8::from_str_radix(&bin_string[i..i + 8], 2))
                .filter_map(|x| x.ok())
                .collect();

    Ok(String::from_utf8(vec_u8)?)
}