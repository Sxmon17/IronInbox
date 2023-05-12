use std::io::prelude::*;
use std::net::TcpStream;

pub struct SmtpClient {
    stream: TcpStream,
}

impl SmtpClient {
    /// Create a new SMTP client connected to the given server.
    pub fn new(server: &str) -> std::io::Result<Self> {
        let stream = TcpStream::connect(server)?;
        Ok(Self { stream })
    }

    /// Send a command to the server.
    pub fn send_command(&mut self, cmd: &str) -> std::io::Result<()> {
        self.stream.write_all(cmd.as_bytes())?;
        self.stream.flush()
    }

    /// Send an email.
    pub fn send_mail(&mut self, from: &str, to: &str, subject: &str, body: &str) -> std::io::Result<()> {
        self.send_command(&format!("MAIL FROM:<{}>\r\n", from))?;
        self.send_command(&format!("RCPT TO:<{}>\r\n", to))?;
        self.send_command("DATA\r\n")?;
        self.send_command(&format!("Subject: {}\r\n", subject))?;
        self.send_command("\r\n")?;
        self.send_command(body)?;
        self.send_command(".\r\n")
    }

    /// Quit the session.
    pub fn quit(&mut self) -> std::io::Result<()> {
        self.send_command("QUIT\r\n")
    }
}
