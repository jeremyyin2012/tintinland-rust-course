use std::ops::Deref;

use rocket::State;
use rocket_okapi::openapi;

use crate::business::Business;
use crate::resp::Error::ParamsError;
use crate::resp::{Error, Resp};
use crate::store::Store;

pub mod api;

/// #接口标题
///
/// 这是一个完全自动生成的接口文档，你只需要编写标题和描述，如果实在不需要也确实可以不写。
///
/// 只需要像 `#[openapi(tag = "Hello World")]` 这样在接口路由上加一行，就可以了。
///
/// tag 用于将接口进行分类，是比较建议加上的，当然，接口量如果非常少那也是可以不用的。
///
/// `#`开头就是标题，换行就空一行，其他就参考 markdown 格式吧。
#[openapi(tag = "Hello World")]
#[get("/")]
pub async fn index() -> &'static str {
    "Hello, world!"
}

#[openapi(tag = "Hello World")]
#[get("/favicon.ico")]
pub async fn favicon() -> &'static str {
    "Hello, world!"
}

#[openapi(tag = "Hello World")]
#[get("/hello")]
pub async fn hello() -> &'static str {
    "Hello, world!"
}

#[openapi(tag = "Ping")]
#[get("/ping")]
pub async fn ping() -> &'static str {
    "pong"
}

#[openapi(tag = "Ping")]
#[get("/api/ping")]
pub async fn api_ping(biz: &State<Business>) -> Result<String, Error> {
    let res = biz.ping.do_ping().await?;
    Ok(res)
}

#[openapi(tag = "Ping")]
#[get("/api/ping_mysql")]
pub async fn api_ping_mysql(biz: &State<Business>) -> Result<String, Error> {
    let res = biz.ping.do_ping_mysql().await?;
    Ok(res)
}

#[openapi(tag = "Ping")]
#[get("/api/ping_pgsql")]
pub async fn api_ping_pgsql(biz: &State<Business>) -> Result<String, Error> {
    let res = biz.ping.do_ping_pgsql().await?;
    Ok(res)
}

#[openapi(tag = "Ping")]
#[get("/api/ping_redis")]
pub async fn api_ping_redis(biz: &State<Business>) -> Result<String, Error> {
    let res = biz.ping.do_ping_redis().await?;
    Ok(res)
}
