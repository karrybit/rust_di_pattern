use anyhow::Result;
use async_trait::async_trait;
use database::Database;
use domain::{
    Frog, FrogID, FrogRepository, RepositoryProvider, Slug, SlugID, SlugRepository, Snake, SnakeID,
    SnakeRepository,
};
use message_queue::MessageQueue;

pub struct Repository<'a> {
    database: &'a Database,
    message_queue: &'a MessageQueue,
}

impl<'a> Repository<'a> {
    pub fn new(database: &'a Database, message_queue: &'a MessageQueue) -> Self {
        Self {
            database,
            message_queue,
        }
    }
}

impl<'a> RepositoryProvider for Repository<'a> {
    type SnakeRepository = Self;
    type SlugRepository = Self;
    type FrogRepository = Self;

    fn snake_repository(&self) -> &Self::SnakeRepository {
        self
    }
    fn slug_repository(&self) -> &Self::SlugRepository {
        self
    }
    fn frog_repository(&self) -> &Self::FrogRepository {
        self
    }
}

#[async_trait]
impl<'a> SnakeRepository for Repository<'a> {
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
impl<'a> SlugRepository for Repository<'a> {
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
impl<'a> FrogRepository for Repository<'a> {
    async fn get_frog(&self, id: FrogID) -> Result<Frog> {
        _ = self.database.conn();
        _ = self.message_queue.conn();
        Ok(Frog {
            id,
            ..Default::default()
        })
    }
}
