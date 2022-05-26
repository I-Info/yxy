use std::error::Error;

use clap::Parser;

pub mod arg;
pub mod conf;

fn main() -> Result<(), Box<dyn Error>> {
    let opts = arg::Options::parse();

    let conf_path = match &opts.config {
        Some(c) => c,
        None => "./conf.yaml",
    };

    let conf = match conf::Config::parse(&conf_path) {
        Ok(v) => v,
        Err(e) => {
            return Err(Box::new(yxy::error::Error::Runtime(format!(
                "Read/Parse conf.yaml file error: {}",
                e
            ))));
        }
    };

    if let Some(v) = opts.command {
        match v {
            arg::Commands::Query { query: q, arg: a } => match q {
                arg::Query::Uid => {
                    query_uid(&a, opts.verbose)?;
                }
                arg::Query::Electricity => {
                    let result = yxy::query_ele(&a, &None, opts.verbose)?;
                    println!("Electricity balance: {}", result.soc);
                }
            },
        }
    } else {
        // Default query electricity
        let result = yxy::query_ele(&conf.uid, &conf.cookie_file, opts.verbose)?;
        println!("Electricity balance: {}", result.soc);
    }

    Ok(())
}

/// Query UID procedure
pub fn query_uid(phone_num: &str, verbose: bool) -> Result<(), yxy::error::Error> {
    let handler = yxy::req::login::LoginHandler::new(phone_num.to_string())?;

    println!("Querying security token...");
    let security_token = handler.get_security_token()?;
    if verbose {
        println!("Success: {:?}", security_token);
    }

    if security_token.level != 0 {
        // image captcha required
        todo!()
    }

    println!("Sending verification code...");
    let user_exists = handler.send_verification_code(&security_token.security_token, None)?;
    if user_exists == false {
        eprintln!("Current user is not registered");
    }

    // Get code from stdin
    let mut code = String::new();
    println!("Send SMS successfully, please enter the verification code:");
    std::io::stdin().read_line(&mut code)?;

    println!("Login...");
    let result = handler.do_login(&code)?;
    if verbose {
        println!("Login response: {:?}", result);
    }
    println!("Login successfully. Here is your uid & other information:");

    // stdout infos
    println!(
        "

    UID: {}
    Token: {}
    Login by device id: {}
    ----------------------------
    Job no: {}
    ID card: {}
    Bind card status: {}
    Last login Time: {}

    ",
        result.id,
        result.token,
        result.device_id,
        result.job_no.unwrap_or_default(),
        result.user_idcard.unwrap_or_default(),
        result.bind_card_status,
        result.last_login,
    );

    Ok(())
}
