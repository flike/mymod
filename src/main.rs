//下面这个申明表示：编译器会去查找同级别的hello.rs或者hello/mod.rs
mod hello;
mod models;
mod routers;
fn main() {
    crate::hello::hello();
    crate::routers::health_router::print_health_router();
    crate::models::user_model::print_user_router();
    crate::models::print_user_model();
}
