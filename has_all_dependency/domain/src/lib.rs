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

#[automock(
    type SnakeRepository=MockSnakeRepository;
    type SlugRepository=MockSlugRepository;
    type FrogRepository=MockFrogRepository;
)]
pub trait RepositoryProvider {
    type SnakeRepository: SnakeRepository;
    type SlugRepository: SlugRepository;
    type FrogRepository: FrogRepository;
    fn snake_repository(&self) -> &Self::SnakeRepository;
    fn slug_repository(&self) -> &Self::SlugRepository;
    fn frog_repository(&self) -> &Self::FrogRepository;
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

#[automock(
    type SnakeUseCase=MockSnakeUseCase;
    type SlugUseCase=MockSlugUseCase;
    type FrogUseCase=MockFrogUseCase;
)]
pub trait UseCaseProvider {
    type SnakeUseCase: SnakeUseCase;
    type SlugUseCase: SlugUseCase;
    type FrogUseCase: FrogUseCase;
    fn snake_use_case(&self) -> &Self::SnakeUseCase;
    fn slug_use_case(&self) -> &Self::SlugUseCase;
    fn frog_use_case(&self) -> &Self::FrogUseCase;
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

#[automock(
    type SnakeService=MockSnakeService;
    type SlugService=MockSlugService;
    type FrogService=MockFrogService;
)]
pub trait ServiceProvider {
    type SnakeService: SnakeService;
    type SlugService: SlugService;
    type FrogService: FrogService;

    fn snake_service(&self) -> &Self::SnakeService;
    fn slug_service(&self) -> &Self::SlugService;
    fn frog_service(&self) -> &Self::FrogService;
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
