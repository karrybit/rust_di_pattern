pub struct MessageQueueConnection;

pub struct MessageQueue {
    conn: MessageQueueConnection,
}

impl MessageQueue {
    pub fn new(conn: MessageQueueConnection) -> Self {
        Self { conn }
    }

    pub fn conn(&self) -> &MessageQueueConnection {
        &self.conn
    }
}
