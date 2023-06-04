use anyhow::Result;
use async_trait::async_trait;
use domain::{
    Frog, FrogID, FrogService, FrogUseCase, ServiceProvider, Slug, SlugID, SlugService,
    SlugUseCase, Snake, SnakeID, SnakeService, SnakeUseCase, UseCaseProvider,
};

pub struct ServiceProviderImpl<'ucp, UCP: UseCaseProvider> {
    use_case: &'ucp UCP,
}

impl<'ucp, UCP: UseCaseProvider> ServiceProviderImpl<'ucp, UCP> {
    pub fn new(use_case: &'ucp UCP) -> Self {
        Self { use_case }
    }
}

impl<'ucp, UCP: UseCaseProvider> ServiceProvider for ServiceProviderImpl<'ucp, UCP> {
    type SnakeService<'a> = SnakeServiceImpl<UCP::SnakeUseCase<'ucp>, UCP::FrogUseCase<'ucp>> where 'ucp: 'a;
    type SlugService<'a> = SlugServiceImpl<UCP::SlugUseCase<'ucp>, UCP::SnakeUseCase<'ucp>> where 'ucp: 'a;
    type FrogService<'a> = FrogServiceImpl<UCP::FrogUseCase<'ucp>, UCP::SlugUseCase<'ucp>> where 'ucp: 'a;
    fn snake_service(&self) -> Self::SnakeService<'_> {
        Self::SnakeService {
            snake_use_case: self.use_case.snake_use_case(),
            frog_use_case: self.use_case.frog_use_case(),
        }
    }
    fn slug_service(&self) -> Self::SlugService<'_> {
        Self::SlugService {
            slug_use_case: self.use_case.slug_use_case(),
            snake_use_case: self.use_case.snake_use_case(),
        }
    }
    fn frog_service(&self) -> Self::FrogService<'_> {
        Self::FrogService {
            frog_use_case: self.use_case.frog_use_case(),
            slug_use_case: self.use_case.slug_use_case(),
        }
    }
}

pub struct SnakeServiceImpl<SnakeUC: SnakeUseCase, FrogUC: FrogUseCase> {
    snake_use_case: SnakeUC,
    frog_use_case: FrogUC,
}

#[async_trait]
impl<SnakeUC: SnakeUseCase, FrogUC: FrogUseCase> SnakeService
    for SnakeServiceImpl<SnakeUC, FrogUC>
{
    async fn get_snake_eating_frog_eating_slug(&self, slug_id: SlugID) -> Result<Snake> {
        let frog = self.frog_use_case.get_frog_eating_slug(slug_id).await?;
        self.snake_use_case.get_snake_eating_frog(frog.id).await
    }
}

pub struct SlugServiceImpl<SlugUC: SlugUseCase, SnakeUC: SnakeUseCase> {
    slug_use_case: SlugUC,
    snake_use_case: SnakeUC,
}

#[async_trait]
impl<SlugUC: SlugUseCase, SnakeUC: SnakeUseCase> SlugService for SlugServiceImpl<SlugUC, SnakeUC> {
    async fn get_slug_eating_snake_eating_frog(&self, frog_id: FrogID) -> Result<Slug> {
        let snake = self.snake_use_case.get_snake_eating_frog(frog_id).await?;
        self.slug_use_case.get_slug_eating_snake(snake.id).await
    }
}

pub struct FrogServiceImpl<FrogUC: FrogUseCase, SlugUC: SlugUseCase> {
    frog_use_case: FrogUC,
    slug_use_case: SlugUC,
}

#[async_trait]
impl<FrogUC: FrogUseCase, SlugUC: SlugUseCase> FrogService for FrogServiceImpl<FrogUC, SlugUC> {
    async fn get_frog_eating_slug_eating_snake(&self, snake_id: SnakeID) -> Result<Frog> {
        let slug = self.slug_use_case.get_slug_eating_snake(snake_id).await?;
        self.frog_use_case.get_frog_eating_slug(slug.id).await
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use domain::{MockFrogUseCase, MockSlugUseCase, MockSnakeUseCase};

    #[tokio::test]
    async fn test_get_snake_eating_frog_eating_slug() {
        let mut snake_use_case = MockSnakeUseCase::new();
        snake_use_case
            .expect_get_snake_eating_frog()
            .returning(|_| Ok(Snake::default()));
        let mut frog_use_case = MockFrogUseCase::new();
        frog_use_case
            .expect_get_frog_eating_slug()
            .returning(|_| Ok(Frog::default()));
        let use_case = SnakeServiceImpl {
            snake_use_case,
            frog_use_case,
        };
        _ = use_case
            .get_snake_eating_frog_eating_slug(SlugID::default())
            .await;
    }

    #[tokio::test]
    async fn test_get_slug_eating_snake_eating_frog() {
        let mut slug_use_case = MockSlugUseCase::new();
        slug_use_case
            .expect_get_slug_eating_snake()
            .returning(|_| Ok(Slug::default()));
        let mut snake_use_case = MockSnakeUseCase::new();
        snake_use_case
            .expect_get_snake_eating_frog()
            .returning(|_| Ok(Snake::default()));
        let use_case = SlugServiceImpl {
            slug_use_case,
            snake_use_case,
        };
        _ = use_case
            .get_slug_eating_snake_eating_frog(FrogID::default())
            .await;
    }

    #[tokio::test]
    async fn test_get_frog_eating_slug_eating_snake() {
        let mut frog_use_case = MockFrogUseCase::new();
        frog_use_case
            .expect_get_frog_eating_slug()
            .returning(|_| Ok(Frog::default()));
        let mut slug_use_case = MockSlugUseCase::new();
        slug_use_case
            .expect_get_slug_eating_snake()
            .returning(|_| Ok(Slug::default()));
        let use_case = FrogServiceImpl {
            frog_use_case,
            slug_use_case,
        };
        _ = use_case
            .get_frog_eating_slug_eating_snake(SnakeID::default())
            .await;
    }
}
