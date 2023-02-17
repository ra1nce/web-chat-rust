use chrono::prelude::*;
use rusqlite::Connection;

pub struct Database {
    conn: Connection
}

impl Database {
    pub fn new() -> Database {
        let conn = Connection::open("database.db").unwrap();

        conn.execute("CREATE TABLE IF NOT EXISTS users (nickname TEXT, ip TEXT, reg_date TEXT);", ())
            .unwrap();
        conn.execute("CREATE TABLE IF NOT EXISTS messages (nickname TEXT, ip TEXT, msg TEXT, date TEXT);", ())
            .unwrap();

        Database { conn }
    }

    pub fn get_messages(&self) -> Vec<(String, String)> {
        let mut stmt = self.conn.prepare("SELECT nickname, msg FROM messages").unwrap();
        let rows = stmt.query_map([], |row| {
            Ok(
                (row.get(0)?, row.get(1)?)
            )
        }).unwrap();

        let mut messages = Vec::new();
        for msg in rows {
            messages.push(msg.unwrap());
        }

        messages
    }

    pub fn add_user(&self, nickname: String, ip: String) {
        let date = self.get_date();
        let sql = "INSERT INTO users (nickname, ip, reg_date) VALUES (?1, ?2, ?3);";
        self.conn.execute(sql, (nickname, ip, date)).unwrap();
    }

    pub fn add_message(&self, nickname: String, ip: String, msg: String) {
        let date = self.get_date();
        let sql = "INSERT INTO messages (nickname, ip, msg, date) VALUES (?1, ?2, ?3, ?4);";
        self.conn.execute(sql, (nickname, ip, msg, date)).unwrap();
    }

    fn get_date(&self) -> String {
        let local: DateTime<Local> = Local::now();
        let date_string = local.format("%Y-%m-%d %H:%M:%S").to_string();

        date_string
    }
}