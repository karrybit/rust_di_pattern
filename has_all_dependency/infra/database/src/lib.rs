pub struct DatabaseConnection;

pub struct Database {
    conn: DatabaseConnection,
}

impl Database {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }

    pub fn conn(&self) -> &DatabaseConnection {
        &self.conn
    }
}
