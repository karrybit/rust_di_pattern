use anyhow::Result;
use async_trait::async_trait;
use database::Database;
use domain::{
    Frog, FrogID, FrogRepository, Slug, SlugID, SlugRepository, Snake, SnakeID, SnakeRepository,
};
use message_queue::MessageQueue;

pub struct Repository {
    database: Database,
    message_queue: MessageQueue,
}

impl Repository {
    pub fn new(database: Database, message_queue: MessageQueue) -> Self {
        Self {
            database,
            message_queue,
        }
    }
}

#[async_trait]
impl SnakeRepository for Repository {
    async fn get_snake(&self, id: SnakeID) -> Result<Snake> {
        _ = self.database.conn();
        _ = self.message_queue.conn();
        Ok(Snake {
            id,
            ..Default::default()
        })
    }
}

#[async_trait]
impl SlugRepository for Repository {
    async fn get_slug(&self, id: SlugID) -> Result<Slug> {
        _ = self.database.conn();
        _ = self.message_queue.conn();
        Ok(Slug {
            id,
            ..Default::default()
        })
    }
}

#[async_trait]
impl FrogRepository for Repository {
    async fn get_frog(&self, id: FrogID) -> Result<Frog> {
        _ = self.database.conn();
        _ = self.message_queue.conn();
        Ok(Frog {
            id,
            ..Default::default()
        })
    }
}
