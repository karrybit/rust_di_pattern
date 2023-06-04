use database::{Database, DatabaseConnection};
use message_queue::{MessageQueue, MessageQueueConnection};
use repository::RepositoryProviderImpl;
use service::ServiceProviderImpl;
use use_case::UseCaseProviderImpl;

mod handler;

#[tokio::main]
async fn main() {
    let database_connection = DatabaseConnection {};
    let database = Database::new(database_connection);
    let message_queue_connection = MessageQueueConnection {};
    let message_queue = MessageQueue::new(message_queue_connection);
    let repository = RepositoryProviderImpl::new(&database, &message_queue);
    let use_case = UseCaseProviderImpl::new(&repository);
    let service = ServiceProviderImpl::new(&use_case);
    _ = handler::Handler::new(&service).run().await
}
