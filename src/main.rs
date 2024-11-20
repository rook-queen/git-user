use std::{
    fmt::Display,
    io::stdin,
    process::{Command, Output},
};

use text_io::try_read;

const COMMANDS: [&'static str; 3] = ["Exit", "Show Current User", "Configure User (local)"];

fn main() {
    loop {
        match handle_input() {
            Ok(num) => {
                match num {
                    0 => {
                        break;
                    }
                    1 => {
                        // show user
                        let res = env_git_user();
                        if let Err(err_msg) = res {
                            println!("{}", err_msg);
                            break;
                        }
                        let data = res.unwrap();
                        println!("{}", data);
                        if data.gpgsign != "true" {
                            println!("[Warning] commit.gpgsign != true. Auto-signing is disabled.");
                        }
                    }
                    2 => {
                        println!("Input name,email,signingkey,gpgsign separated by a new line, 0 = ignore");
                        // configure user
                        let res = stdin_git_user();
                        if let Err(_) = res {
                            println!("Failed to get user from input");
                            break;
                        }
                        let data = res.unwrap();
                        let set_res = data.set_as_current();
                        if let Err(_) = set_res {
                            println!("Failed to set data as current user");
                            break;
                        }
                        println!("Successfully set as current user!\n{}", data);
                    }
                    _ => {
                        println!("Unimplemented Command. Sorry >_<")
                    }
                }
            }
            Err(msg) => {
                println!("{}", msg);
            }
        }
    }
}

struct GitUserData {
    name: String,
    email: String,
    signingkey: String,
    gpgsign: String,
}

impl GitUserData {
    fn set_as_current(&self) -> Result<(), Box<dyn std::error::Error>> {
        set_git_config_property("user.name", &self.name)?;
        set_git_config_property("user.email", &self.email)?;
        set_git_config_property("user.signingkey", &self.signingkey)?;
        set_git_config_property("commit.gpgsign", &self.gpgsign)?;
        Ok(())
    }
}

impl Display for GitUserData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "user.name: {}\nuser.email: {}\nuser.signingkey: {}\ncommit.gpgsign: {}",
            self.name, self.email, self.signingkey, self.gpgsign
        )
    }
}

fn stdin_git_user() -> Result<GitUserData, Box<dyn std::error::Error>> {
    let mut user_data = GitUserData {
        name: String::new(),
        email: String::new(),
        signingkey: String::new(),
        gpgsign: String::new(),
    };

    user_data.name = get_or_default(readline_til_not_empty().as_str(), "user.name")?;
    user_data.email = get_or_default(readline_til_not_empty().as_str(), "user.email")?;
    user_data.signingkey = get_or_default(readline_til_not_empty().as_str(), "user.signingkey")?;
    user_data.gpgsign = get_or_default(readline_til_not_empty().as_str(), "commit.gpgsign")?;
    Ok(user_data)
}

fn get_or_default(val: &str, property: &str) -> Result<String, String> {
    match val {
        "0" => get_git_config_property(property),
        _ => Ok(val.to_owned()),
    }
}

/**
 * Read line till a line is not empty (ignoring all whitespaces)
 */
fn readline_til_not_empty() -> String {
    let mut buf = String::with_capacity(25);
    loop {
        if let Err(_) = stdin().read_line(&mut buf) {
            dbg!("readline issue");
        };
        buf = buf.trim().to_owned(); // a bit underperformant
        if buf.len() > 0 {
            break;
        }
    }
    buf
}

fn env_git_user() -> Result<GitUserData, String> {
    let user_data = GitUserData {
        name: get_git_config_property("user.name")?,
        email: get_git_config_property("user.email")?,
        signingkey: get_git_config_property("user.signingkey")?,
        gpgsign: get_git_config_property("commit.gpgsign")?,
    };
    Ok(user_data)
}

fn get_git_config_property(config_property: &str) -> Result<String, String> {
    match Command::new("git")
        .args(["config", config_property])
        .output()
    {
        Ok(out) => match String::from_utf8(out.stdout) {
            Ok(v) => Ok(v.trim().to_owned()),
            // Performance Issue, copy from v's slice into a new string and destruct v
            Err(_) => {
                return Err(format!("Failed to convert {} to utf8", config_property));
            }
        },
        Err(sth) => {
            return Err(format!("Failed to get {} due to {}", config_property, sth));
        }
    }
}

fn set_git_config_property(config_property: &str, val: &str) -> Result<Output, std::io::Error> {
    Command::new("git")
        .args(["config", config_property, val])
        .output()
}

fn handle_input() -> Result<i32, &'static str> {
    println!("*---*\nGit-User :3");
    let mut i = 0;
    for x in COMMANDS {
        println!("{}. {}", i, x);
        i += 1;
    }
    print!("*---*\n> ");
    let valres: Result<i32, _> = try_read!();
    if let Err(_) = valres {
        return Err("Wow");
    }
    let val = valres.unwrap();
    if val < 0 || val >= COMMANDS.len() as i32 {
        return Err("Invalid Command");
    }

    Ok(val)
}
