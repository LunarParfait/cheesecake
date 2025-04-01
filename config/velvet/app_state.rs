use async_trait::async_trait;
use core::fmt;

pub trait AppStateTrait: fmt::Debug + Send + Sync + Clone + 'static {}

#[async_trait]
pub trait AppStateFactoryTrait<O: AppStateTrait> {
    async fn new() -> O;
}

impl<T: fmt::Debug + Send + Sync + Clone + 'static> AppStateTrait for T {}
