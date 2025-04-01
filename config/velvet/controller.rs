use axum::Router;
use crate::app::app_state::AppState;

pub trait Controller {
    fn router() -> (&'static str, Router<AppState>);
}

#[macro_export]
macro_rules! create_single_route {
    ($routes:ident, $controller:ident) => {
        let (name, crouter) = $controller::router();
        if name == "/" {
            $routes = $routes.merge(crouter);
        } else {
            $routes = $routes.nest(name, crouter);
        }
    };
}

#[macro_export]
macro_rules! create_routes_internal {
    ($routes:ident, $controller:ident) => {
        create_single_route! { $routes, $controller }
    };

    ($routes:ident, $controller:ident, $($other:ident),+) => {
        create_single_route! { $routes, $controller }
        create_routes! { $routes, $($other),+ };
    };
}

#[macro_export]
macro_rules! create_routes {
    ($($controller:ident),+) => {{
        use config::create_routes_internal;
        use config::create_single_route;
        use config::velvet::controller::Controller;
        let mut routes = Router::new();
        create_routes_internal! { routes, $($controller),+ };
        routes
    }};
}
