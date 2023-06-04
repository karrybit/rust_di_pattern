use domain::{Frog, FrogID, FrogService, Slug, SlugID, SlugService, Snake, SnakeID, SnakeService};

pub(crate) struct Handler<T>
where
    T: SnakeService + SlugService + FrogService,
{
    service: T,
}

impl<T: SnakeService + SlugService + FrogService> Handler<T> {
    pub(crate) fn new(service: T) -> Self {
        Self { service }
    }
    pub(crate) async fn run(&self) -> (Snake, Slug, Frog) {
        let snake = self
            .service
            .get_snake_eating_frog_eating_slug(SlugID::default())
            .await
            .unwrap();
        let slug = self
            .service
            .get_slug_eating_snake_eating_frog(FrogID::default())
            .await
            .unwrap();
        let frog = self
            .service
            .get_frog_eating_slug_eating_snake(SnakeID::default())
            .await
            .unwrap();
        (snake, slug, frog)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;
    use async_trait::async_trait;
    use mockall::mock;

    mock! {
        Service {}
        #[async_trait]
        impl SnakeService for Service {
            async fn get_snake_eating_frog_eating_slug(&self, slug_id: SlugID) -> Result<Snake>;
        }
        #[async_trait]
        impl SlugService for Service {
            async fn get_slug_eating_snake_eating_frog(&self, frog_id: FrogID) -> Result<Slug>;
        }
        #[async_trait]
        impl FrogService for Service {
            async fn get_frog_eating_slug_eating_snake(&self, snake_id: SnakeID) -> Result<Frog>;
        }
    }

    #[tokio::test]
    async fn test_run() {
        let mut service = MockService::new();
        service
            .expect_get_snake_eating_frog_eating_slug()
            .returning(|_| Ok(Snake::default()));
        service
            .expect_get_slug_eating_snake_eating_frog()
            .returning(|_| Ok(Slug::default()));
        service
            .expect_get_frog_eating_slug_eating_snake()
            .returning(|_| Ok(Frog::default()));
        _ = Handler::new(service).run().await;
    }
}
