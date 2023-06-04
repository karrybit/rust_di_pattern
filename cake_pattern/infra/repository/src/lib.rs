use anyhow::Result;
use async_trait::async_trait;
use database::Database;
use domain::{
    Frog, FrogID, FrogRepository, RepositoryProvider, Slug, SlugID, SlugRepository, Snake, SnakeID,
    SnakeRepository,
};
use message_queue::MessageQueue;

pub struct RepositoryProviderImpl<'r> {
    database: &'r Database,
    message_queue: &'r MessageQueue,
}

impl<'r> RepositoryProviderImpl<'r> {
    pub fn new(database: &'r Database, message_queue: &'r MessageQueue) -> Self {
        Self {
            database,
            message_queue,
        }
    }
}

impl<'r> RepositoryProvider for RepositoryProviderImpl<'r> {
    type SnakeRepository<'a> = SnakeRepositoryImpl<'r> where 'r: 'a;
    type SlugRepository<'a> = SlugRepositoryImpl<'r> where 'r: 'a;
    type FrogRepository<'a> = FrogRepositoryImpl<'r> where 'r: 'a;

    fn snake_repository(&self) -> Self::SnakeRepository<'_> {
        Self::SnakeRepository {
            database: self.database,
            message_queue: self.message_queue,
        }
    }
    fn slug_repository(&self) -> Self::SlugRepository<'_> {
        Self::SlugRepository {
            database: self.database,
            message_queue: self.message_queue,
        }
    }
    fn frog_repository(&self) -> Self::FrogRepository<'_> {
        Self::FrogRepository {
            database: self.database,
            message_queue: self.message_queue,
        }
    }
}

pub struct SnakeRepositoryImpl<'a> {
    database: &'a Database,
    message_queue: &'a MessageQueue,
}

#[async_trait]
impl<'a> SnakeRepository for SnakeRepositoryImpl<'a> {
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
impl<'a> SlugRepository for SlugRepositoryImpl<'a> {
    async fn get_slug(&self, id: SlugID) -> Result<Slug> {
        _ = self.database.conn();
        _ = self.message_queue.conn();
        Ok(Slug {
            id,
            ..Default::default()
        })
    }
}

pub struct SlugRepositoryImpl<'a> {
    database: &'a Database,
    message_queue: &'a MessageQueue,
}

pub struct FrogRepositoryImpl<'a> {
    database: &'a Database,
    message_queue: &'a MessageQueue,
}

#[async_trait]
impl<'a> FrogRepository for FrogRepositoryImpl<'a> {
    async fn get_frog(&self, id: FrogID) -> Result<Frog> {
        _ = self.database.conn();
        _ = self.message_queue.conn();
        Ok(Frog {
            id,
            ..Default::default()
        })
    }
}
