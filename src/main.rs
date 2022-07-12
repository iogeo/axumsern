use std::io::prelude::*;
use axum::{Json, response::{Html, IntoResponse, Response},routing::get,Router,http::{Uri, header::{self, HeaderMap, HeaderName}}};
use std::fs::File;
use std::fs;
use std::str::FromStr;
use std::env;
use axum::body::Full;

struct js
{
    String:String
}

async fn response() -> axum::http::response::Builder {
    Response::builder()
}

async fn root() -> impl IntoResponse{
    let mut r=File::open("indexmain.html").unwrap();
    let mut p = String::new();
    r.read_to_string(&mut p);
    response()
        .await.status(200)
        .header("Content-Type","text/html; charset=UTF-8")
        .header("Cross-Origin-Embedder-Policy","require-corp")
        .header("Cross-Origin-Opener-Policy","same-origin")
        .body(Full::from(p))
        .unwrap()
}

async fn index2() -> impl IntoResponse{
    let mut r=File::open("index2.html").unwrap();
    let mut p = String::new();
    r.read_to_string(&mut p);
    response()
        .await.status(200)
        .header("Content-Type","text/html; charset=UTF-8")
        .header("Cross-Origin-Embedder-Policy","require-corp")
        .header("Cross-Origin-Opener-Policy","same-origin")
        .body(Full::from(p))
        .unwrap()
}

async fn index3() -> impl IntoResponse{
    let mut r=File::open("index3.html").unwrap();
    let mut p = String::new();
    r.read_to_string(&mut p);
    response()
        .await.status(200)
        .header("Content-Type","text/html; charset=UTF-8")
        .header("Cross-Origin-Embedder-Policy","require-corp")
        .header("Cross-Origin-Opener-Policy","same-origin")
        .body(Full::from(p))
        .unwrap()
}

async fn index4() -> impl IntoResponse{
    let mut r=File::open("index4.html").unwrap();
    let mut p = String::new();
    r.read_to_string(&mut p);
    response()
        .await.status(200)
        .header("Content-Type","text/html; charset=UTF-8")
        .header("Cross-Origin-Embedder-Policy","require-corp")
        .header("Cross-Origin-Opener-Policy","same-origin")
        .body(Full::from(p))
        .unwrap()
}

async fn index5() -> impl IntoResponse{
    let mut r=File::open("index5.html").unwrap();
    let mut p = String::new();
    r.read_to_string(&mut p);
    response()
        .await.status(200)
        .header("Content-Type","text/html; charset=UTF-8")
        .header("Cross-Origin-Embedder-Policy","require-corp")
        .header("Cross-Origin-Opener-Policy","same-origin")
        .body(Full::from(p))
        .unwrap()
}

async fn index6() -> impl IntoResponse{
    let mut r=File::open("index6.html").unwrap();
    let mut p = String::new();
    r.read_to_string(&mut p);
    response()
        .await.status(200)
        .header("Content-Type","text/html; charset=UTF-8")
        .header("Cross-Origin-Embedder-Policy","require-corp")
        .header("Cross-Origin-Opener-Policy","same-origin")
        .body(Full::from(p))
        .unwrap()
}

async fn index6n() -> impl IntoResponse{
    let mut r=File::open("index6n.html").unwrap();
    let mut p = String::new();
    r.read_to_string(&mut p);
    response()
        .await.status(200)
        .header("Content-Type","text/html; charset=UTF-8")
        .header("Cross-Origin-Embedder-Policy","require-corp")
        .header("Cross-Origin-Opener-Policy","same-origin")
        .body(Full::from(p))
        .unwrap()
}

async fn index7() -> impl IntoResponse{
    let mut r=File::open("index7.html").unwrap();
    let mut p = String::new();
    r.read_to_string(&mut p);
    response()
        .await.status(200)
        .header("Content-Type","text/html; charset=UTF-8")
        .header("Cross-Origin-Embedder-Policy","require-corp")
        .header("Cross-Origin-Opener-Policy","same-origin")
        .body(Full::from(p))
        .unwrap()
}

async fn index8() -> impl IntoResponse{
    let mut r=File::open("index8.html").unwrap();
    let mut p = String::new();
    r.read_to_string(&mut p);
    response()
        .await.status(200)
        .header("Content-Type","text/html; charset=UTF-8")
        .header("Cross-Origin-Embedder-Policy","require-corp")
        .header("Cross-Origin-Opener-Policy","same-origin")
        .body(Full::from(p))
        .unwrap()
}

async fn index9() -> impl IntoResponse{
    let mut r=File::open("index9.html").unwrap();
    let mut p = String::new();
    r.read_to_string(&mut p);
    response()
        .await.status(200)
        .header("Content-Type","text/html; charset=UTF-8")
        .header("Cross-Origin-Embedder-Policy","require-corp")
        .header("Cross-Origin-Opener-Policy","same-origin")
        .body(Full::from(p))
        .unwrap()
}

async fn index9n() -> impl IntoResponse{
    let mut r=File::open("index9.html").unwrap();
    let mut p = String::new();
    r.read_to_string(&mut p);
    response()
        .await.status(200)
        .header("Content-Type","text/html; charset=UTF-8")
        .header("Cross-Origin-Embedder-Policy","require-corp")
        .header("Cross-Origin-Opener-Policy","same-origin")
        .body(Full::from(p))
        .unwrap()
}

async fn pkg2() -> impl IntoResponse{
    let mut r=File::open("./pkg/cqw.js").unwrap();
    let mut p = String::new();
    r.read_to_string(&mut p);
    response()
        .await.status(200)
        .header("Content-Type","text/javascript")
        .header("Cross-Origin-Embedder-Policy","require-corp")
        .header("Cross-Origin-Opener-Policy","same-origin")
        .body(Full::from(p))
        .unwrap()
}

async fn pkgbg() -> impl IntoResponse{
    let mut r=File::open("./pkg/cqw_bg.wasm").unwrap();
    let mut p = vec![];
    r.read_to_end(&mut p);
    response()
        .await.status(200)
        .header("Content-Type","application/wasm")
        .header("Cross-Origin-Embedder-Policy","require-corp")
        .header("Cross-Origin-Opener-Policy","same-origin")
        .body(Full::from(p))
        .unwrap()
}

async fn pkgjs() -> impl IntoResponse{
    let mut r=File::open("ffmpeg.min.js").unwrap();
    let mut p = String::new();
    r.read_to_string(&mut p);
    response()
        .await.status(200)
        .header("Content-Type","text/javascript; charset=UTF-8")
        .header("Cross-Origin-Embedder-Policy","require-corp")
        .header("Cross-Origin-Opener-Policy","same-origin")
        .body(Full::from(p))
        .unwrap()
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route(
        "/", get(root))
        .route(
        "/index2", get(index2))
        .route(
        "/index3", get(index3))
        .route(
        "/index4", get(index4))
        .route(
        "/index5", get(index5))
        .route(
        "/index6", get(index6))
        .route(
        "/index6n", get(index6n))
        .route(
        "/index7", get(index7))
        .route(
        "/index8", get(index8))
        .route(
        "/index9", get(index9))
        .route(
        "/index9", get(index9n))
        .route(
        "/pkg", get(pkg2))
        .route(
        "/cqw_bg.wasm", get(pkgbg))
        .route(
        "/ffmpeg.min.js", get(pkgjs));
    let q = env::var("PORT")
        .unwrap()
        .to_string();
    axum::Server::bind(&("0.0.0.0:".to_owned()+&q).parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
