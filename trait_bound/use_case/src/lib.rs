use anyhow::Result;
use async_trait::async_trait;
use domain::{
    Frog, FrogID, FrogRepository, FrogUseCase, Slug, SlugID, SlugRepository, SlugUseCase, Snake,
    SnakeID, SnakeRepository, SnakeUseCase,
};

pub struct UseCase<T> {
    repository: T,
}

impl<T> UseCase<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<T: SnakeRepository + FrogRepository> SnakeUseCase for UseCase<T> {
    async fn get_snake(&self, id: SnakeID) -> Result<Snake> {
        self.repository.get_snake(id).await
    }

    async fn get_snake_eating_frog(&self, frog_id: FrogID) -> Result<Snake> {
        let frog = self.repository.get_frog(frog_id).await.unwrap();
        self.repository.get_snake(frog.eaten_by).await
    }
}

#[async_trait]
impl<T: SlugRepository + SnakeRepository> SlugUseCase for UseCase<T> {
    async fn get_slug(&self, id: SlugID) -> Result<Slug> {
        self.repository.get_slug(id).await
    }

    async fn get_slug_eating_snake(&self, snake_id: SnakeID) -> Result<Slug> {
        let snake = self.repository.get_snake(snake_id).await.unwrap();
        self.repository.get_slug(snake.eaten_by).await
    }
}

#[async_trait]
impl<T: FrogRepository + SlugRepository> FrogUseCase for UseCase<T> {
    async fn get_frog(&self, id: FrogID) -> anyhow::Result<domain::Frog> {
        self.repository.get_frog(id).await
    }
    async fn get_frog_eating_slug(&self, slug_id: SlugID) -> Result<Frog> {
        let slug = self.repository.get_slug(slug_id).await.unwrap();
        self.repository.get_frog(slug.eaten_by).await
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;
    use async_trait::async_trait;
    use mockall::mock;

    mock! {
        SnakeFrogRepository {}
        #[async_trait]
        impl SnakeRepository for SnakeFrogRepository {
            async fn get_snake(&self, id: SnakeID) -> Result<Snake>;
        }
        #[async_trait]
        impl FrogRepository for SnakeFrogRepository {
            async fn get_frog(&self, id: FrogID) -> Result<Frog>;
        }
    }

    #[tokio::test]
    async fn test_get_snake() {
        let mut repository = MockSnakeFrogRepository::new();
        repository
            .expect_get_snake()
            .returning(|_| Ok(Snake::default()));
        let use_case = UseCase::new(repository);
        _ = use_case.get_snake(SnakeID::default()).await;
    }

    #[tokio::test]
    async fn test_get_snake_eating_frog() {
        let mut repository = MockSnakeFrogRepository::new();
        repository
            .expect_get_snake()
            .returning(|_| Ok(Snake::default()));
        repository
            .expect_get_frog()
            .returning(|_| Ok(Frog::default()));
        let use_case = UseCase::new(repository);
        _ = use_case.get_snake_eating_frog(FrogID::default()).await;
    }

    mock! {
        SlugSnakeRepository {}
        #[async_trait]
        impl SlugRepository for SlugSnakeRepository {
            async fn get_slug(&self, id: SlugID) -> Result<Slug>;
        }
        #[async_trait]
        impl SnakeRepository for SlugSnakeRepository {
            async fn get_snake(&self, id: SnakeID) -> Result<Snake>;
        }
    }

    #[tokio::test]
    async fn test_get_slug() {
        let mut repository = MockSlugSnakeRepository::new();
        repository
            .expect_get_slug()
            .returning(|_| Ok(Slug::default()));
        let use_case = UseCase::new(repository);
        _ = use_case.get_slug(SlugID::default()).await;
    }

    #[tokio::test]
    async fn test_get_slug_eating_snake() {
        let mut repository = MockSlugSnakeRepository::new();
        repository
            .expect_get_slug()
            .returning(|_| Ok(Slug::default()));
        repository
            .expect_get_snake()
            .returning(|_| Ok(Snake::default()));
        let use_case = UseCase::new(repository);
        _ = use_case.get_slug_eating_snake(SnakeID::default()).await;
    }

    mock! {
        FrogSlugRepository {}
        #[async_trait]
        impl FrogRepository for FrogSlugRepository {
            async fn get_frog(&self, id: FrogID) -> Result<Frog>;
        }
        #[async_trait]
        impl SlugRepository for FrogSlugRepository {
            async fn get_slug(&self, id: SlugID) -> Result<Slug>;
        }
    }

    #[tokio::test]
    async fn test_get_frog() {
        let mut repository = MockFrogSlugRepository::new();
        repository
            .expect_get_frog()
            .returning(|_| Ok(Frog::default()));
        let use_case = UseCase::new(repository);
        _ = use_case.get_frog(FrogID::default()).await;
    }

    #[tokio::test]
    async fn test_get_frog_eating_slug() {
        let mut repository = MockFrogSlugRepository::new();
        repository
            .expect_get_frog()
            .returning(|_| Ok(Frog::default()));
        repository
            .expect_get_slug()
            .returning(|_| Ok(Slug::default()));
        let use_case = UseCase::new(repository);
        _ = use_case.get_frog_eating_slug(SlugID::default()).await;
    }
}
