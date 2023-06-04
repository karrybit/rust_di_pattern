use anyhow::Result;
use async_trait::async_trait;
use domain::{
    Frog, FrogID, FrogRepository, FrogUseCase, RepositoryProvider, Slug, SlugID, SlugRepository,
    SlugUseCase, Snake, SnakeID, SnakeRepository, SnakeUseCase, UseCaseProvider,
};

pub struct UseCaseProviderImpl<'rp, RP: RepositoryProvider> {
    repository: &'rp RP,
}

impl<'rp, RP: RepositoryProvider> UseCaseProviderImpl<'rp, RP> {
    pub fn new(repository: &'rp RP) -> Self {
        Self { repository }
    }
}

impl<'rp, RP: RepositoryProvider> UseCaseProvider for UseCaseProviderImpl<'rp, RP> {
    type SnakeUseCase<'a> = SnakeUseCaseImpl<RP::SnakeRepository<'rp>, RP::FrogRepository<'rp>> where 'rp : 'a;
    type SlugUseCase<'a> = SlugUseCaseImpl<RP::SlugRepository<'rp>, RP::SnakeRepository<'rp>> where 'rp: 'a;
    type FrogUseCase<'a> = FrogUseCaseImpl<RP::FrogRepository<'rp>, RP::SlugRepository<'rp>> where 'rp: 'a;

    fn snake_use_case(&self) -> Self::SnakeUseCase<'_> {
        Self::SnakeUseCase {
            snake_repository: self.repository.snake_repository(),
            frog_repository: self.repository.frog_repository(),
        }
    }
    fn slug_use_case(&self) -> Self::SlugUseCase<'_> {
        Self::SlugUseCase {
            slug_repository: self.repository.slug_repository(),
            snake_repository: self.repository.snake_repository(),
        }
    }
    fn frog_use_case(&self) -> Self::FrogUseCase<'_> {
        Self::FrogUseCase {
            frog_repository: self.repository.frog_repository(),
            slug_repository: self.repository.slug_repository(),
        }
    }
}

pub struct SnakeUseCaseImpl<SnakeR: SnakeRepository, FrogR: FrogRepository> {
    snake_repository: SnakeR,
    frog_repository: FrogR,
}

#[async_trait]
impl<SnakeR: SnakeRepository, FrogR: FrogRepository> SnakeUseCase
    for SnakeUseCaseImpl<SnakeR, FrogR>
{
    async fn get_snake(&self, id: SnakeID) -> Result<Snake> {
        self.snake_repository.get_snake(id).await
    }
    async fn get_snake_eating_frog(&self, frog_id: FrogID) -> Result<Snake> {
        let frog = self.frog_repository.get_frog(frog_id).await?;
        self.snake_repository.get_snake(frog.eaten_by).await
    }
}

pub struct SlugUseCaseImpl<SlugR: SlugRepository, SnakeR: SnakeRepository> {
    slug_repository: SlugR,
    snake_repository: SnakeR,
}

#[async_trait]
impl<SlugR: SlugRepository, SnakeR: SnakeRepository> SlugUseCase
    for SlugUseCaseImpl<SlugR, SnakeR>
{
    async fn get_slug(&self, id: SlugID) -> Result<Slug> {
        self.slug_repository.get_slug(id).await
    }
    async fn get_slug_eating_snake(&self, snake_id: SnakeID) -> Result<Slug> {
        let snake = self.snake_repository.get_snake(snake_id).await?;
        self.slug_repository.get_slug(snake.eaten_by).await
    }
}

pub struct FrogUseCaseImpl<FrogR: FrogRepository, SlugR: SlugRepository> {
    frog_repository: FrogR,
    slug_repository: SlugR,
}

#[async_trait]
impl<FrogR: FrogRepository, SlugR: SlugRepository> FrogUseCase for FrogUseCaseImpl<FrogR, SlugR> {
    async fn get_frog(&self, id: FrogID) -> Result<Frog> {
        self.frog_repository.get_frog(id).await
    }
    async fn get_frog_eating_slug(&self, slug_id: SlugID) -> Result<Frog> {
        let slug = self.slug_repository.get_slug(slug_id).await?;
        self.frog_repository.get_frog(slug.eaten_by).await
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use domain::{MockFrogRepository, MockSlugRepository, MockSnakeRepository};

    #[tokio::test]
    async fn test_get_snake() {
        let mut snake_repository = MockSnakeRepository::new();
        snake_repository
            .expect_get_snake()
            .returning(|_| Ok(Snake::default()));
        let snake_use_case = SnakeUseCaseImpl {
            snake_repository,
            frog_repository: MockFrogRepository::new(),
        };
        _ = snake_use_case.get_snake(SnakeID::default()).await;
    }

    #[tokio::test]
    async fn test_get_snake_eating_frog() {
        let mut snake_repository = MockSnakeRepository::new();
        snake_repository
            .expect_get_snake()
            .returning(|_| Ok(Snake::default()));
        let mut frog_repository = MockFrogRepository::new();
        frog_repository
            .expect_get_frog()
            .returning(|_| Ok(Frog::default()));
        let snake_use_case = SnakeUseCaseImpl {
            snake_repository,
            frog_repository,
        };
        _ = snake_use_case
            .get_snake_eating_frog(FrogID::default())
            .await;
    }

    #[tokio::test]
    async fn test_get_slug() {
        let mut slug_repository = MockSlugRepository::new();
        slug_repository
            .expect_get_slug()
            .returning(|_| Ok(Slug::default()));
        let slug_use_case = SlugUseCaseImpl {
            slug_repository,
            snake_repository: MockSnakeRepository::new(),
        };
        _ = slug_use_case.get_slug(SlugID::default()).await;
    }

    #[tokio::test]
    async fn test_get_slug_eating_snake() {
        let mut slug_repository = MockSlugRepository::new();
        slug_repository
            .expect_get_slug()
            .returning(|_| Ok(Slug::default()));
        let mut snake_repository = MockSnakeRepository::new();
        snake_repository
            .expect_get_snake()
            .returning(|_| Ok(Snake::default()));
        let slug_use_case = SlugUseCaseImpl {
            slug_repository,
            snake_repository,
        };
        _ = slug_use_case
            .get_slug_eating_snake(SnakeID::default())
            .await;
    }

    #[tokio::test]
    async fn test_get_frog() {
        let mut frog_repository = MockFrogRepository::new();
        frog_repository
            .expect_get_frog()
            .returning(|_| Ok(Frog::default()));
        let slug_use_case = FrogUseCaseImpl {
            frog_repository,
            slug_repository: MockSlugRepository::new(),
        };
        _ = slug_use_case.get_frog(FrogID::default()).await;
    }

    #[tokio::test]
    async fn test_get_frog_eating_slug() {
        let mut frog_repository = MockFrogRepository::new();
        frog_repository
            .expect_get_frog()
            .returning(|_| Ok(Frog::default()));
        let mut slug_repository = MockSlugRepository::new();
        slug_repository
            .expect_get_slug()
            .returning(|_| Ok(Slug::default()));
        let slug_use_case = FrogUseCaseImpl {
            frog_repository,
            slug_repository,
        };
        _ = slug_use_case.get_frog_eating_slug(SlugID::default()).await;
    }
}
