use domain::{
    Frog, FrogID, FrogService, ServiceProvider, Slug, SlugID, SlugService, Snake, SnakeID,
    SnakeService,
};

pub(crate) struct Handler<'sp, SP: ServiceProvider> {
    service: &'sp SP,
}

impl<'sp, SP: ServiceProvider> Handler<'sp, SP> {
    pub(crate) fn new(service: &'sp SP) -> Self {
        Self { service }
    }

    pub(crate) async fn run(&self) -> (Snake, Slug, Frog) {
        let snake = self
            .service
            .snake_service()
            .get_snake_eating_frog_eating_slug(SlugID::default())
            .await
            .unwrap();
        let slug = self
            .service
            .slug_service()
            .get_slug_eating_snake_eating_frog(FrogID::default())
            .await
            .unwrap();
        let frog = self
            .service
            .frog_service()
            .get_frog_eating_slug_eating_snake(SnakeID::default())
            .await
            .unwrap();
        (snake, slug, frog)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use domain::{MockFrogService, MockServiceProvider, MockSlugService, MockSnakeService};

    #[tokio::test]
    async fn test() {
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
        let mut provider = MockServiceProvider::new();
        provider
            .expect_snake_service()
            .return_once(|| snake_service);
        provider.expect_slug_service().return_once(|| slug_service);
        provider.expect_frog_service().return_once(|| frog_service);

        let handler = Handler::new(&provider);
        _ = handler.run().await;
    }
}
