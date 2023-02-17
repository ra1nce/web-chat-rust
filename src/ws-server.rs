#[macro_use]
extern crate dotenv_codegen;
extern crate parity_ws as ws;

mod database;

use crate::database::Database;

struct Server {
    ws: ws::Sender,
}

impl ws::Handler for Server {
    fn on_open(&mut self, shake: ws::Handshake) -> ws::Result<()> {
        let ip = match shake.remote_addr()? {
            Some(ip_addr) => {
                println!("Connection opened from {}.", ip_addr);
                ip_addr
            },
            None => {
                println!("Unable to obtain client's IP address.");
                "getting ip error".to_string()
            },
        };

        let db = Database::new();
        db.add_user("None".to_string(), ip);

        Ok(())
    }

    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        if msg.to_string().starts_with("send_message: ") {
            let data = msg.to_string().replace("send_message: ", "");
            let data: Vec<&str> = data.split(";").collect();
            let nickname = data.get(0).unwrap_or(&"None");
            let msg = data.get(1).unwrap_or(&"None");

            let db = Database::new();
            db.add_message(nickname.to_string(), "None".to_string(), msg.to_string());

            let text = format!("new_message: {nickname};{msg}");
            self.ws.broadcast(text)
        } else {
            self.ws.send("Unknown command!")
        }
    }
}

fn main() {
    let port = dotenv!("WS_PORT");
    ws::listen(format!("127.0.0.1:{port}"), |out| Server { ws: out })
        .unwrap()
}