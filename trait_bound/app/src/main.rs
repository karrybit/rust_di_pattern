use database::{Database, DatabaseConnection};

use message_queue::{MessageQueue, MessageQueueConnection};
use repository::Repository;
use service::Service;
use use_case::UseCase;

mod handler;

#[tokio::main]
async fn main() {
    let database_connection = DatabaseConnection {};
    let database = Database::new(database_connection);
    let message_queue_connection = MessageQueueConnection {};
    let message_queue = MessageQueue::new(message_queue_connection);
    let repository = Repository::new(database, message_queue);
    let use_case = UseCase::new(repository);
    let service = Service::new(use_case);
    let handler = handler::Handler::new(service);
    _ = handler.run().await;
}
