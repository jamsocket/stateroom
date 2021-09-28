use crate::{api::authenticate, cli_opts::LoginCommand, config::GlobalConfigHandle, API_BASE};
use colored::Colorize;

pub fn login(login_opts: LoginCommand) -> anyhow::Result<()> {
    let url = format!("{}login/github", API_BASE);
    let mut config = GlobalConfigHandle::new()?;

    if let Some(token) = login_opts.token {
        if config.config.token.is_some() && !login_opts.clear {
            println!("A token is already configured, use -c to overwrite it.");
            return Ok(());
        }

        match authenticate(&token) {
            Ok(true) => {}
            Ok(false) => {
                println!("Failed to authenticate with given token.");
                return Ok(());
            }
            Err(e) => {
                println!("Encountered error: {}", e);
                return Ok(());
            }
        }

        config.config.token = Some(token);
        config.write()?;

        return Ok(());
    } else if let Some(token) = config.config.token {
        if login_opts.clear {
            config.config.token = None;
            config.write()?;
            println!("Configured token has been cleared.");
            return Ok(());
        }

        // User is not setting a token, but one exists in the config,
        // so we will check it.
        match authenticate(&token) {
            Ok(true) => {
                println!(
                    "{}\nTo clear the current token, run `jamsocket login -c`.",
                    "Current token is valid.".green().bold()
                );
            }
            Ok(false) => {
                println!(
                    "{} To create a new one, open the URL below and follow the instructions.\n\n{}",
                    "Current token is not valid.".red().bold(),
                    url.yellow().bold()
                );
            }
            Err(e) => {
                println!("Encountered error: {}", e);
            }
        }
    } else {
        println!(
            "Log in by opening the following URL in a web browser and following the instructions.\n\n{}",
            url.yellow().bold()
        );
    }

    Ok(())
}
