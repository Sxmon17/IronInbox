use std::io::prelude::*;
use std::net::TcpStream;

struct ImapClient {
    stream: TcpStream,
}

impl ImapClient {
    /// Create a new IMAP client connected to the given server.
    fn new(server: &str, username: &str, password: &str) -> std::io::Result<Self> {
        let mut stream = TcpStream::connect(server)?;
        Self::send_command(&mut stream, "LOGIN", &format!("{} {}", username, password))?;
        Ok(Self { stream })
    }

    /// Send a command to the server.
    fn send_command(stream: &mut TcpStream, command: &str, args: &str) -> std::io::Result<()> {
        stream.write_all(format!("{} {}\r\n", command, args).as_bytes())?;
        stream.flush()
    }

    /// Read the response from the server.
    fn read_response(stream: &mut TcpStream) -> std::io::Result<String> {
        let mut response = String::new();
        stream.read_to_string(&mut response)?;
        Ok(response)
    }

    /// Receive emails.
    fn receive_mail(&mut self) -> std::io::Result<Vec<String>> {
        Self::send_command(&mut self.stream, "SELECT", "INBOX")?;
        Self::send_command(&mut self.stream, "FETCH", "1:* (BODY[TEXT])")?;
        let response = Self::read_response(&mut self.stream)?;
        let mails = response.split("\r\n")
            .filter(|line| line.starts_with("*"))
            .map(|line|line.trim_start_matches("* ").trim_end_matches(")").to_string())
            .collect();
        Ok(mails)
    }

    fn logout(&mut self) -> std::io::Result<()> {
        Self::send_command(&mut self.stream, "LOGOUT", "")
    }
}


