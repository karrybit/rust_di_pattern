use anyhow::Result;
use async_trait::async_trait;
use domain::{
    Frog, FrogID, FrogRepository, FrogUseCase, RepositoryProvider, Slug, SlugID, SlugRepository,
    SlugUseCase, Snake, SnakeID, SnakeRepository, SnakeUseCase, UseCaseProvider,
};

pub struct UseCsae<'r, RP: RepositoryProvider> {
    snake_repository: &'r RP::SnakeRepository,
    slug_repository: &'r RP::SlugRepository,
    frog_repository: &'r RP::FrogRepository,
}

impl<'r, RP: RepositoryProvider> UseCsae<'r, RP> {
    pub fn new(repository: &'r RP) -> Self {
        Self {
            snake_repository: repository.snake_repository(),
            slug_repository: repository.slug_repository(),
            frog_repository: repository.frog_repository(),
        }
    }
}

impl<'r, RP: RepositoryProvider> UseCaseProvider for UseCsae<'r, RP> {
    type SnakeUseCase = Self;
    type SlugUseCase = Self;
    type FrogUseCase = Self;

    fn snake_use_case(&self) -> &Self::SnakeUseCase {
        self
    }
    fn slug_use_case(&self) -> &Self::SlugUseCase {
        self
    }
    fn frog_use_case(&self) -> &Self::FrogUseCase {
        self
    }
}

#[async_trait]
impl<'r, RP: RepositoryProvider> SnakeUseCase for UseCsae<'r, RP> {
    async fn get_snake(&self, id: SnakeID) -> Result<Snake> {
        self.snake_repository.get_snake(id).await
    }
    async fn get_snake_eating_frog(&self, frog_id: FrogID) -> Result<Snake> {
        let frog = self.frog_repository.get_frog(frog_id).await?;
        self.snake_repository.get_snake(frog.eaten_by).await
    }
}

#[async_trait]
impl<'r, RP: RepositoryProvider> SlugUseCase for UseCsae<'r, RP> {
    async fn get_slug(&self, id: SlugID) -> Result<Slug> {
        self.slug_repository.get_slug(id).await
    }
    async fn get_slug_eating_snake(&self, snake_id: SnakeID) -> Result<Slug> {
        let snake = self.snake_repository.get_snake(snake_id).await?;
        self.slug_repository.get_slug(snake.eaten_by).await
    }
}

#[async_trait]
impl<'r, RP: RepositoryProvider> FrogUseCase for UseCsae<'r, RP> {
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
    use domain::{
        MockFrogRepository, MockRepositoryProvider, MockSlugRepository, MockSnakeRepository,
    };

    #[tokio::test]
    async fn test_get_snake() {
        let mut snake_repository = MockSnakeRepository::new();
        snake_repository
            .expect_get_snake()
            .returning(|_| Ok(Snake::default()));
        let slug_repository = MockSlugRepository::new();
        let frog_repository = MockFrogRepository::new();
        let mut repository = MockRepositoryProvider::new();
        repository
            .expect_snake_repository()
            .return_const(snake_repository);
        repository
            .expect_slug_repository()
            .return_const(slug_repository);
        repository
            .expect_frog_repository()
            .return_const(frog_repository);
        let use_case = UseCsae::new(&repository);
        _ = use_case.get_snake(SnakeID::default()).await;
    }

    #[tokio::test]
    async fn test_get_snake_eating_frog() {
        let mut snake_repository = MockSnakeRepository::new();
        snake_repository
            .expect_get_snake()
            .returning(|_| Ok(Snake::default()));
        let slug_repository = MockSlugRepository::new();
        let mut frog_repository = MockFrogRepository::new();
        frog_repository
            .expect_get_frog()
            .returning(|_| Ok(Frog::default()));
        let mut repository = MockRepositoryProvider::new();
        repository
            .expect_snake_repository()
            .return_const(snake_repository);
        repository
            .expect_slug_repository()
            .return_const(slug_repository);
        repository
            .expect_frog_repository()
            .return_const(frog_repository);
        let use_case = UseCsae::new(&repository);
        _ = use_case.get_snake_eating_frog(FrogID::default()).await;
    }

    #[tokio::test]
    async fn test_get_slug() {
        let snake_repository = MockSnakeRepository::new();
        let mut slug_repository = MockSlugRepository::new();
        slug_repository
            .expect_get_slug()
            .returning(|_| Ok(Slug::default()));
        let frog_repository = MockFrogRepository::new();
        let mut repository = MockRepositoryProvider::new();
        repository
            .expect_snake_repository()
            .return_const(snake_repository);
        repository
            .expect_slug_repository()
            .return_const(slug_repository);
        repository
            .expect_frog_repository()
            .return_const(frog_repository);
        let use_case = UseCsae::new(&repository);
        _ = use_case.get_slug(SlugID::default()).await;
    }

    #[tokio::test]
    async fn test_get_slug_eating_snake() {
        let mut snake_repository = MockSnakeRepository::new();
        snake_repository
            .expect_get_snake()
            .returning(|_| Ok(Snake::default()));
        let mut slug_repository = MockSlugRepository::new();
        slug_repository
            .expect_get_slug()
            .returning(|_| Ok(Slug::default()));
        let frog_repository = MockFrogRepository::new();
        let mut repository = MockRepositoryProvider::new();
        repository
            .expect_snake_repository()
            .return_const(snake_repository);
        repository
            .expect_slug_repository()
            .return_const(slug_repository);
        repository
            .expect_frog_repository()
            .return_const(frog_repository);
        let use_case = UseCsae::new(&repository);
        _ = use_case.get_slug_eating_snake(SnakeID::default()).await;
    }

    #[tokio::test]
    async fn test_get_frog() {
        let snake_repository = MockSnakeRepository::new();
        let slug_repository = MockSlugRepository::new();
        let mut frog_repository = MockFrogRepository::new();
        frog_repository
            .expect_get_frog()
            .returning(|_| Ok(Frog::default()));
        let mut repository = MockRepositoryProvider::new();
        repository
            .expect_snake_repository()
            .return_const(snake_repository);
        repository
            .expect_slug_repository()
            .return_const(slug_repository);
        repository
            .expect_frog_repository()
            .return_const(frog_repository);
        let use_case = UseCsae::new(&repository);
        _ = use_case.get_frog(FrogID::default()).await;
    }

    #[tokio::test]
    async fn test_get_frog_eating_slug() {
        let snake_repository = MockSnakeRepository::new();
        let mut slug_repository = MockSlugRepository::new();
        slug_repository
            .expect_get_slug()
            .returning(|_| Ok(Slug::default()));
        let mut frog_repository = MockFrogRepository::new();
        frog_repository
            .expect_get_frog()
            .returning(|_| Ok(Frog::default()));
        let mut repository = MockRepositoryProvider::new();
        repository
            .expect_snake_repository()
            .return_const(snake_repository);
        repository
            .expect_slug_repository()
            .return_const(slug_repository);
        repository
            .expect_frog_repository()
            .return_const(frog_repository);
        let use_case = UseCsae::new(&repository);
        _ = use_case.get_frog_eating_slug(SlugID::default()).await;
    }
}
