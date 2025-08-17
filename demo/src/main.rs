use std::sync::{Arc, LazyLock};

use salvo::prelude::*;
use serde::{Deserialize, Serialize};
use toolbox::{
    auth::{Jwt, JwtAuth, JwtConfig, JwtToken},
    logger::Logger,
    resolve,
    resp::Resp,
    validator::extractor::VJson,
};
use validator::Validate;

#[tokio::main]
async fn main() {
    // log::init();
    let router = Router::with_path("/user")
        .hoop(JwtAuth::<User, _>::new(&["/index", "/login"]))
        .push(Router::new().path("/index").get(index))
        .push(Router::new().path("/login").post(login))
        .push(Router::new().path("/info").get(user_info))
        .push(Router::new().path("/source").get(source));

    println!("App running at: http://0.0.0.0:8080");
    println!("{router:?}");
    let listener = TcpListener::new("0.0.0.0:8080").bind().await;
    let server = Service::new(router).hoop(Logger::default());
    Server::new(listener).serve(server).await;
}

#[handler]
async fn index() -> Resp<()> {
    resolve!(200, "Hello World!")
}

#[handler]
async fn source(depot: &mut Depot) -> Resp<&'static str> {
    let msg: Arc<str> = Arc::from("获取资源成功");
    depot.insert("other", msg);
    resolve!("获取资源成功" => 200, "OK")
}

#[handler]
async fn login(user: VJson<User>) -> Resp<String> {
    resolve!(user.0.encode()? => 200, "登录成功")
}

#[handler]
async fn user_info(user: Jwt<User>) -> Resp<User> {
    resolve!(user.0 => 200, "获取用户信息成功")
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Validate)]
struct User {
    #[validate(length(min = 3, max = 20))]
    #[validate(email)]
    username: String,

    #[validate(length(min = 8, max = 20))]
    password: String,
}

static JWT_CONFIG: LazyLock<JwtConfig> = LazyLock::new(|| JwtConfig::new("key", 24 * 3600));
impl JwtToken for User {
    fn config() -> &'static JwtConfig {
        &JWT_CONFIG
    }
}
