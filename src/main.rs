use clap::Parser;
use std::error::Error;

mod arg;
mod conf;

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

    // Read the session cache
    let session = match &conf.cookie_file {
        None => None,
        Some(cookie_file) => match std::fs::read_to_string(cookie_file) {
            Ok(v) => {
                if opts.verbose {
                    println!("Using cached session id: {}", v);
                }
                Some(v)
            }
            Err(e) => {
                eprintln!("Session cache file reading error: {}", e);
                None
            }
        },
    };

    if let Some(v) = opts.command {
        match v {
            arg::Commands::Query { query: q, arg: a } => match q {
                arg::Query::Uid => {
                    query_uid(&a, opts.verbose)?;
                }
                arg::Query::Electricity => {
                    let (result, _session) = yxy::query_ele(&a, None, opts.verbose)?;
                    print_ele(&result);
                }
            },
        }
    } else {
        // Default query electricity
        let (result, session) = yxy::query_ele(&conf.uid, session, opts.verbose)?;

        // Cache the session
        if let Some(cookie_file) = &conf.cookie_file {
            if let Err(e) = yxy::utils::file_write(&cookie_file, &session.unwrap()) {
                eprintln!("Fail to cache the session id: {}", e);
            } else if opts.verbose {
                println!("Session cached.")
            }
        }

        // Notification
        if opts.notify {
            // Message push service
            if let Some(sc) = conf.server_chan {
                println!("Pushing message to ServerChan channel...");
                if result.soc < sc.warning_threshold {
                    yxy::req::notice::push_message(
                        &sc.key,
                        &format!("{}{}", &sc.warning_title, &result.soc),
                        &fmt_ele_md(&result),
                    )?;
                } else if sc.log_level == 0 {
                    yxy::req::notice::push_message(
                        &sc.key,
                        &format!("{}{}", &sc.title, &result.soc),
                        &fmt_ele_md(&result),
                    )?;
                } else {
                    println!("Nothing to do.");
                }
                println!("Success.")
            } else {
                eprintln!("No message push config found");
            }
        } else {
            print_ele(&result);
        }
    }

    Ok(())
}

/// fmt & print electricity info
fn print_ele(info: &yxy::req::app::ElectricityInfo) {
    let surplus = &info.surplus_list[0];
    println!(
        "
Electricity Info: 
-----------------
Room: {}
Status: {}

Total Surplus: {} kW·h
Total Amount: ￥{}

Basic: {} kW·h | ￥{}
Subsidy : {} kW·h | ￥{}
",
        info.display_room_name,
        surplus.room_status,
        info.soc,
        info.total_soc_amount,
        surplus.surplus,
        surplus.amount,
        surplus.subsidy,
        surplus.subsidy_amount,
    );
}

/// fmt electricity info in markdown style
pub fn fmt_ele_md(info: &yxy::req::app::ElectricityInfo) -> String {
    let surplus = &info.surplus_list[0];
    format!(
        "\
# Electricity Info
-----------------
- Room: **{}**
- Status: **{}**

- Total Surplus: **{}** kW·h
- Total Amount: **￥{}**

- Basic: **{}** kW·h | **￥{}**
- Subsidy : **{}** kW·h | **￥{}**
",
        info.display_room_name,
        surplus.room_status,
        info.soc,
        info.total_soc_amount,
        surplus.surplus,
        surplus.amount,
        surplus.subsidy,
        surplus.subsidy_amount,
    )
}

/// Query UID procedure
fn query_uid(phone_num: &str, verbose: bool) -> Result<(), yxy::error::Error> {
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
