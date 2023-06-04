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
    type SnakeRepository<'a>: SnakeRepository
    where
        Self: 'a;
    type SlugRepository<'a>: SlugRepository
    where
        Self: 'a;
    type FrogRepository<'a>: FrogRepository
    where
        Self: 'a;
    fn snake_repository(&self) -> Self::SnakeRepository<'_>;
    fn slug_repository(&self) -> Self::SlugRepository<'_>;
    fn frog_repository(&self) -> Self::FrogRepository<'_>;
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
    type SnakeUseCase<'a>: SnakeUseCase
    where
        Self: 'a;
    type SlugUseCase<'a>: SlugUseCase
    where
        Self: 'a;
    type FrogUseCase<'a>: FrogUseCase
    where
        Self: 'a;
    fn snake_use_case(&self) -> Self::SnakeUseCase<'_>;
    fn slug_use_case(&self) -> Self::SlugUseCase<'_>;
    fn frog_use_case(&self) -> Self::FrogUseCase<'_>;
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
    type SnakeService<'a>: SnakeService
    where
        Self: 'a;
    type SlugService<'a>: SlugService
    where
        Self: 'a;
    type FrogService<'a>: FrogService
    where
        Self: 'a;
    fn snake_service(&self) -> Self::SnakeService<'_>;
    fn slug_service(&self) -> Self::SlugService<'_>;
    fn frog_service(&self) -> Self::FrogService<'_>;
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
