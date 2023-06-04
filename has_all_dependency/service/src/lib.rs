use anyhow::Result;
use async_trait::async_trait;
use domain::{
    Frog, FrogID, FrogService, FrogUseCase, ServiceProvider, Slug, SlugID, SlugService,
    SlugUseCase, Snake, SnakeID, SnakeService, SnakeUseCase, UseCaseProvider,
};

pub struct Service<'ucp, UCP: UseCaseProvider> {
    snake_use_case: &'ucp UCP::SnakeUseCase,
    slug_use_case: &'ucp UCP::SlugUseCase,
    frog_use_case: &'ucp UCP::FrogUseCase,
}

impl<'ucp, UCP: UseCaseProvider> Service<'ucp, UCP> {
    pub fn new(use_case: &'ucp UCP) -> Self {
        Self {
            snake_use_case: use_case.snake_use_case(),
            slug_use_case: use_case.slug_use_case(),
            frog_use_case: use_case.frog_use_case(),
        }
    }
}

impl<'ucp, UCP: UseCaseProvider> ServiceProvider for Service<'ucp, UCP> {
    type SnakeService = Self;
    type SlugService = Self;
    type FrogService = Self;

    fn snake_service(&self) -> &Self::SnakeService {
        self
    }
    fn slug_service(&self) -> &Self::SlugService {
        self
    }
    fn frog_service(&self) -> &Self::FrogService {
        self
    }
}

#[async_trait]
impl<'ucp, UCP: UseCaseProvider> SnakeService for Service<'ucp, UCP> {
    async fn get_snake_eating_frog_eating_slug(&self, slug_id: SlugID) -> Result<Snake> {
        let frog = self.frog_use_case.get_frog_eating_slug(slug_id).await?;
        self.snake_use_case.get_snake_eating_frog(frog.id).await
    }
}

#[async_trait]
impl<'ucp, UCP: UseCaseProvider> SlugService for Service<'ucp, UCP> {
    async fn get_slug_eating_snake_eating_frog(&self, frog_id: FrogID) -> Result<Slug> {
        let snake = self.snake_use_case.get_snake_eating_frog(frog_id).await?;
        self.slug_use_case.get_slug_eating_snake(snake.id).await
    }
}

#[async_trait]
impl<'ucp, UCP: UseCaseProvider> FrogService for Service<'ucp, UCP> {
    async fn get_frog_eating_slug_eating_snake(&self, snake_id: SnakeID) -> Result<Frog> {
        let slug = self.slug_use_case.get_slug_eating_snake(snake_id).await?;
        self.frog_use_case.get_frog_eating_slug(slug.id).await
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use domain::{MockFrogUseCase, MockSlugUseCase, MockSnakeUseCase, MockUseCaseProvider};

    #[tokio::test]
    async fn test_get_snake_eating_frog_eating_slug() {
        let mut snake_use_case = MockSnakeUseCase::new();
        snake_use_case
            .expect_get_snake_eating_frog()
            .returning(|_| Ok(Snake::default()));
        let slug_use_case = MockSlugUseCase::new();
        let mut frog_use_case = MockFrogUseCase::new();
        frog_use_case
            .expect_get_frog_eating_slug()
            .returning(|_| Ok(Frog::default()));
        let mut use_case = MockUseCaseProvider::new();
        use_case
            .expect_snake_use_case()
            .return_const(snake_use_case);
        use_case.expect_slug_use_case().return_const(slug_use_case);
        use_case.expect_frog_use_case().return_const(frog_use_case);
        let service = Service::new(&use_case);
        _ = service
            .get_snake_eating_frog_eating_slug(SlugID::default())
            .await;
    }

    #[tokio::test]
    async fn test_get_slug_eating_snake_eating_frog() {
        let mut snake_use_case = MockSnakeUseCase::new();
        snake_use_case
            .expect_get_snake_eating_frog()
            .returning(|_| Ok(Snake::default()));
        let mut slug_use_case = MockSlugUseCase::new();
        slug_use_case
            .expect_get_slug_eating_snake()
            .returning(|_| Ok(Slug::default()));
        let frog_use_case = MockFrogUseCase::new();
        let mut use_case = MockUseCaseProvider::new();
        use_case
            .expect_snake_use_case()
            .return_const(snake_use_case);
        use_case.expect_slug_use_case().return_const(slug_use_case);
        use_case.expect_frog_use_case().return_const(frog_use_case);
        let service = Service::new(&use_case);
        _ = service
            .get_slug_eating_snake_eating_frog(FrogID::default())
            .await;
    }

    #[tokio::test]
    async fn test_get_frog_eating_slug_eating_snake() {
        let snake_use_case = MockSnakeUseCase::new();
        let mut slug_use_case = MockSlugUseCase::new();
        slug_use_case
            .expect_get_slug_eating_snake()
            .returning(|_| Ok(Slug::default()));
        let mut frog_use_case = MockFrogUseCase::new();
        frog_use_case
            .expect_get_frog_eating_slug()
            .returning(|_| Ok(Frog::default()));
        let mut use_case = MockUseCaseProvider::new();
        use_case
            .expect_snake_use_case()
            .return_const(snake_use_case);
        use_case.expect_slug_use_case().return_const(slug_use_case);
        use_case.expect_frog_use_case().return_const(frog_use_case);
        let service = Service::new(&use_case);
        _ = service
            .get_frog_eating_slug_eating_snake(SnakeID::default())
            .await;
    }
}
