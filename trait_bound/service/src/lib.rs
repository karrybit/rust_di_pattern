use anyhow::Result;
use async_trait::async_trait;
use domain::{
    Frog, FrogID, FrogService, FrogUseCase, Slug, SlugID, SlugService, SlugUseCase, Snake, SnakeID,
    SnakeService, SnakeUseCase,
};

pub struct Service<T> {
    use_case: T,
}

impl<T> Service<T> {
    pub fn new(use_case: T) -> Self {
        Self { use_case }
    }
}

#[async_trait]
impl<T: SnakeUseCase + FrogUseCase> SnakeService for Service<T> {
    async fn get_snake_eating_frog_eating_slug(&self, slug_id: SlugID) -> Result<Snake> {
        let frog = self.use_case.get_frog_eating_slug(slug_id).await.unwrap();
        self.use_case.get_snake_eating_frog(frog.id).await
    }
}

#[async_trait]
impl<T: SlugUseCase + SnakeUseCase> SlugService for Service<T> {
    async fn get_slug_eating_snake_eating_frog(&self, frog_id: FrogID) -> Result<Slug> {
        let snake = self.use_case.get_snake_eating_frog(frog_id).await.unwrap();
        self.use_case.get_slug_eating_snake(snake.id).await
    }
}

#[async_trait]
impl<T: FrogUseCase + SlugUseCase> FrogService for Service<T> {
    async fn get_frog_eating_slug_eating_snake(&self, snake_id: SnakeID) -> Result<Frog> {
        let slug = self.use_case.get_slug_eating_snake(snake_id).await.unwrap();
        self.use_case.get_frog_eating_slug(slug.id).await
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;
    use async_trait::async_trait;
    use mockall::mock;

    mock! {
        SnakeFrogUseCase {}
        #[async_trait]
        impl SnakeUseCase for SnakeFrogUseCase {
            async fn get_snake(&self, id: SnakeID) -> Result<Snake>;
            async fn get_snake_eating_frog(&self, frog_id: FrogID) -> Result<Snake>;
        }
        #[async_trait]
        impl FrogUseCase for SnakeFrogUseCase {
            async fn get_frog(&self, id: FrogID) -> Result<Frog>;
            async fn get_frog_eating_slug(&self, slug_id: SlugID) -> Result<Frog>;
        }
    }

    #[tokio::test]
    async fn test_get_snake_eating_frog_eating_slug() {
        let mut use_case = MockSnakeFrogUseCase::new();
        use_case
            .expect_get_frog_eating_slug()
            .returning(|_| Ok(Frog::default()));
        use_case
            .expect_get_snake_eating_frog()
            .returning(|_| Ok(Snake::default()));
        let service = Service::new(use_case);
        _ = service
            .get_snake_eating_frog_eating_slug(SlugID::default())
            .await;
    }

    mock! {
        SlugSnakeUseCase {}
        #[async_trait]
        impl SlugUseCase for SlugSnakeUseCase {
            async fn get_slug(&self, id: SlugID) -> Result<Slug>;
            async fn get_slug_eating_snake(&self, snake_id: SnakeID) -> Result<Slug>;
        }
        #[async_trait]
        impl SnakeUseCase for SlugSnakeUseCase {
            async fn get_snake(&self, id: SnakeID) -> Result<Snake>;
            async fn get_snake_eating_frog(&self, frog_id: FrogID) -> Result<Snake>;
        }
    }

    #[tokio::test]
    async fn test_get_slug_eating_snake_eating_frog() {
        let mut use_case = MockSlugSnakeUseCase::new();
        use_case
            .expect_get_slug_eating_snake()
            .returning(|_| Ok(Slug::default()));
        use_case
            .expect_get_snake_eating_frog()
            .returning(|_| Ok(Snake::default()));
        let service = Service::new(use_case);
        _ = service
            .get_slug_eating_snake_eating_frog(FrogID::default())
            .await;
    }

    mock! {
        FrogSlugUseCase {}
        #[async_trait]
        impl FrogUseCase for FrogSlugUseCase {
            async fn get_frog(&self, id: FrogID) -> Result<Frog>;
            async fn get_frog_eating_slug(&self, slug_id: SlugID) -> Result<Frog>;
        }
        #[async_trait]
        impl SlugUseCase for FrogSlugUseCase {
            async fn get_slug(&self, id: SlugID) -> Result<Slug>;
            async fn get_slug_eating_snake(&self, snake_id: SnakeID) -> Result<Slug>;
        }
    }

    #[tokio::test]
    async fn test_get_frog_eating_slug_eating_snake() {
        let mut use_case = MockFrogSlugUseCase::new();
        use_case
            .expect_get_frog_eating_slug()
            .returning(|_| Ok(Frog::default()));
        use_case
            .expect_get_slug_eating_snake()
            .returning(|_| Ok(Slug::default()));
        let service = Service::new(use_case);
        _ = service
            .get_frog_eating_slug_eating_snake(SnakeID::default())
            .await;
    }
}
