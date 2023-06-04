use domain::{
    Frog, FrogID, FrogService, ServiceProvider, Slug, SlugID, SlugService, Snake, SnakeID,
    SnakeService,
};

pub(crate) struct Handler<'sp, SP: ServiceProvider> {
    snake_service: &'sp SP::SnakeService,
    slug_service: &'sp SP::SlugService,
    frog_service: &'sp SP::FrogService,
}

impl<'sp, SP: ServiceProvider> Handler<'sp, SP> {
    pub(crate) fn new(service: &'sp SP) -> Self {
        Self {
            snake_service: service.snake_service(),
            slug_service: service.slug_service(),
            frog_service: service.frog_service(),
        }
    }

    pub(crate) async fn run(&self) -> (Snake, Slug, Frog) {
        let snake = self
            .snake_service
            .get_snake_eating_frog_eating_slug(SlugID::default())
            .await
            .unwrap();
        let slug = self
            .slug_service
            .get_slug_eating_snake_eating_frog(FrogID::default())
            .await
            .unwrap();
        let frog = self
            .frog_service
            .get_frog_eating_slug_eating_snake(SnakeID::default())
            .await
            .unwrap();
        (snake, slug, frog)
    }
}

#[cfg(test)]
mod test {
    use domain::{MockFrogService, MockServiceProvider, MockSlugService, MockSnakeService};

    use super::*;

    #[tokio::test]
    async fn test_run() {
        let mut snake_service = MockSnakeService::new();
        snake_service
            .expect_get_snake_eating_frog_eating_slug()
            .returning(|_| Ok(Snake::default()));
        let mut slug_service = MockSlugService::new();
        slug_service
            .expect_get_slug_eating_snake_eating_frog()
            .returning(|_| Ok(Slug::default()));
        let mut frog_service = MockFrogService::new();
        frog_service
            .expect_get_frog_eating_slug_eating_snake()
            .returning(|_| Ok(Frog::default()));
        let mut service = MockServiceProvider::new();
        service.expect_snake_service().return_const(snake_service);
        service.expect_slug_service().return_const(slug_service);
        service.expect_frog_service().return_const(frog_service);
        let handler = Handler::new(&service);
        _ = handler.run().await;
    }
}
