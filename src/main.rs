use std::process::Command;
use std::fmt::{self, Formatter, Display};

struct Args<'a, 'b>(&'a[&'b str]);
#[derive(Default)]
struct App<'a> {
    command: &'a str,
    args: &'a[&'a str]
}

impl<'a, 'b> Display for Args<'a, 'b> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0.join(" "))
    }
}

fn run(apps: &[&App]) {
    for app in apps.iter() {
        println!("");
        println!("========================");
        println!("$ {} {}", app.command, Args(app.args));
        println!("========================");

        let child = Command::new(app.command)
            .args(app.args)
            .spawn();

        match child {
            Err(error) => panic!("{}", error),
            Ok(mut result) => {
                match result.try_wait() {
                    Err(error) => panic!("{}", error),
                    Ok(Some(_status)) => continue,
                    Ok(None) => {
                        let wait = result.wait();

                        match wait {
                            Err(error) => panic!("{}", error),
                            Ok(_status) => continue
                        }
                    }
                };
            }
        };
    }
}

fn main() {
    let pacman_keyring = App {
        command: "sudo",
        args: &["pacman", "--noconfirm", "-S", "archlinux-keyring"]
    };
    let pacman_update = App {
        command: "sudo",
        args: &["pacman", "--noconfirm", "-Syu"]
    };
    let yay_update = App {
        command: "yay",
        args: &["-Syu"]
    };
    let yay_cleanup = App {
        command: "yay",
        args: &["-Yc"]
    };
    let apps: &[&App] = &[&pacman_keyring, &pacman_update, &yay_update, &yay_cleanup];

    run(apps);
}
