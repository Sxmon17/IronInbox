mod smtp_client;
mod imap_client;
mod utils;
mod cli;

use clap::{Clap, Command};
use smtp_client::SmtpClient;
use imap_client::ImapClient;
use utils::{print_success, print_error};

fn main() {
    let opts: Opts = Opts::parse();

    match opts.cmd {
        Some(Command::Send(send)) => {
            // ... SMTP code here ...
        }
        Some(Command::Receive) => {
            match ImapClient::new(&opts.imap_server, &opts.username, &opts.password) {
                Ok(mut client) => {
                    match client.receive_mail() {
                        Ok(mails) => {
                            for (i, mail) in mails.iter().enumerate() {
                                println!("Mail {}: {}", i + 1, mail.green());
                            }
                        }
                        Err(e) => {
                            print_error(&format!("Failed to receive mail: {}", e));
                        }
                    }
                    if let Err(e) = client.logout() {
                        print_error(&format!("Failed to logout: {}", e));
                    }
                }
                Err(e) => {
                    print_error(&format!("Failed to connect to IMAP server: {}", e));
                }
            }
        }
        _ => {}
    }
}
