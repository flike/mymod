pub fn print_user_model() {
    println!("user_model");
}

pub fn print_user_router() {
    crate::routers::health_router::print_health_router();
    super::super::routers::health_router::print_health_router();
}
