use anyhow::Result;
use async_trait::async_trait;
use mockall::automock;

#[derive(Default)]
pub struct SnakeID;
#[derive(Default)]
pub struct SlugID;
#[derive(Default)]
pub struct FrogID;

#[derive(Default)]
pub struct Snake {
    pub id: SnakeID,
    pub eaten_by: SlugID,
}

#[derive(Default)]
pub struct Slug {
    pub id: SlugID,
    pub eaten_by: FrogID,
}

#[derive(Default)]
pub struct Frog {
    pub id: FrogID,
    pub eaten_by: SnakeID,
}

#[automock]
#[async_trait]
pub trait SnakeRepository: Send + Sync {
    async fn get_snake(&self, id: SnakeID) -> Result<Snake>;
}
#[automock]
#[async_trait]
pub trait SlugRepository: Send + Sync {
    async fn get_slug(&self, id: SlugID) -> Result<Slug>;
}
#[automock]
#[async_trait]
pub trait FrogRepository: Send + Sync {
    async fn get_frog(&self, id: FrogID) -> Result<Frog>;
}

#[automock]
#[async_trait]
pub trait SnakeUseCase: Send + Sync {
    async fn get_snake(&self, id: SnakeID) -> Result<Snake>;
    async fn get_snake_eating_frog(&self, frog_id: FrogID) -> Result<Snake>;
}
#[automock]
#[async_trait]
pub trait SlugUseCase: Send + Sync {
    async fn get_slug(&self, id: SlugID) -> Result<Slug>;
    async fn get_slug_eating_snake(&self, snake_id: SnakeID) -> Result<Slug>;
}
#[automock]
#[async_trait]
pub trait FrogUseCase: Send + Sync {
    async fn get_frog(&self, id: FrogID) -> Result<Frog>;
    async fn get_frog_eating_slug(&self, slug_id: SlugID) -> Result<Frog>;
}

#[automock]
#[async_trait]
pub trait SnakeService: Send + Sync {
    async fn get_snake_eating_frog_eating_slug(&self, slug_id: SlugID) -> Result<Snake>;
}
#[automock]
#[async_trait]
pub trait SlugService: Send + Sync {
    async fn get_slug_eating_snake_eating_frog(&self, frog_id: FrogID) -> Result<Slug>;
}
#[automock]
#[async_trait]
pub trait FrogService: Send + Sync {
    async fn get_frog_eating_slug_eating_snake(&self, snake_id: SnakeID) -> Result<Frog>;
}
