use std::{fmt::Display, io::stdin, process::Command};

use text_io::try_read;

const COMMANDS: [&'static str; 3] = ["Exit", "Show Current User", "Configure User (local)"];

fn main() {
    let mut running = true;
    while running {
        match handle_input() {
            Ok(num) => match num {
                0 => {
                    running = false;
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
                }
                2 => {
                    // configure user
                    let res = stdin_git_uesr();
                    if let Err(_) = res {
                        println!("Failed to get user from input");
                        break;
                    }
                    let data = res.unwrap();
                    println!("{}", data);
                    unimplemented!();
                }
                _ => {
                    println!("Unimplemented Command. Sorry >_<")
                }
            },
            Err(msg) => {
                println!("{}", msg);
            }
        }
    }
}

struct GitUserData {
    name: String,
    email: String,
}

impl Display for GitUserData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "user.name: {}\nuser.email: {}", self.name, self.email)
    }
}

fn stdin_git_uesr() -> Result<GitUserData, Box<dyn std::error::Error>> {
    let mut user_data = GitUserData {
        name: String::new(),
        email: String::new(),
    };

    let mut val = String::with_capacity(25);
    stdin().read_line(&mut val)?;
    user_data.name = val.clone();

    stdin().read_line(&mut val)?;
    user_data.email = val;

    Ok(user_data)
}

fn env_git_user() -> Result<GitUserData, String> {
    let user_data = GitUserData {
        name: get_git_config_property("user.name")?,
        email: get_git_config_property("user.email")?,
    };
    Ok(user_data)
}

fn get_git_config_property(config_property: &str) -> Result<String, String> {
    match Command::new("git")
        .args(["config", config_property])
        .output()
    {
        Ok(out) => match String::from_utf8(out.stdout) {
            Ok(v) => Ok(v.trim().into()),
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
