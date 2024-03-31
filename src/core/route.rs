
pub enum RouteStrategy {
    Tag,
    Domain,
    IP
}

pub struct Route {
    strategy: RouteStrategy,
}
