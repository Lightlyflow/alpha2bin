use std::error::Error;

pub struct Config {
    pub query_string: String,
    pub bin_to_alpha: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 1 {
            return Err("Not enough arguments!");
        }

        let mut query_string = "".to_string();
        let mut bin_to_alpha = false;
        // Parse cli args
        for (i, arg) in args.iter().enumerate() {
            if i == 0 { continue; }
            else if arg == "-r" { bin_to_alpha = true; }
            else { query_string = arg.clone(); }
        }

        Ok(Config {
            query_string,
            bin_to_alpha,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut output = String::new();

    if config.bin_to_alpha {
        output = bin_to_alpha(&config.query_string)?;
    } else {
        output = str_to_bin(&config.query_string)?;
    }

    println!("{}", output);

    Ok(())
}

fn str_to_bin(alpha_string: &String) -> Result<String, Box<dyn Error>> {
    println!("Mode: string to binary");

    let mut output: String = "".to_string();
    for word in alpha_string.split_whitespace() {
        for char in word.to_string().into_bytes() {
            output += &format!("0{:b} ", char);
        }
    }

    Ok(output)
}

fn bin_to_alpha(bin_string: &String) -> Result<String, Box<dyn Error>> {
    println!("Mode: binary representation to string");

    let vec_u8 = (0..bin_string.len())
                .step_by(9)
                .map(|i| u8::from_str_radix(&bin_string[i..i + 8], 2))
                .filter_map(|x| x.ok())
                .collect();

    Ok(String::from_utf8(vec_u8)?)
}