use std::io::prelude::*;
use axum::{Json, response::{Html, IntoResponse, Response},routing::get,Router,http::{Uri, header::{self, HeaderMap, HeaderName}},};
use std::fs::File;
use std::fs;
use std::str::FromStr;
use std::env;
use axum::body::Full;
use std::process::Command;
use axum::extract::WebSocketUpgrade;

use std::thread::sleep;
use core::time::Duration;

struct js
{
    String:String
}

async fn response() -> axum::http::response::Builder {
    Response::builder()
}

async fn root(ws: WebSocketUpgrade) -> impl IntoResponse{
    ws.on_upgrade(move |mut sock| async move{
    let qw=sock.recv().await.unwrap().unwrap().into_data();
    fs::write(".\\frame_interpolation\\qq\\q.jpg", qw).unwrap();
    let qwp=sock.recv().await.unwrap().unwrap().into_data();
    fs::write(".\\frame_interpolation\\qq\\q2.jpg", qwp).unwrap();
    let qww = Command::new("sh")
            .arg("-c")
            .arg(r#"python -m frame_interpolation.eval.interpolator_cli --pattern "frame_interpolation\\qq" --model_path eqqww\\\\film_net\\\\Style\\\\saved_model --times_to_interpolate 2 --output_video"#)
            .output()
            .expect("failed to execute process");
    fs::write(".\\frame_interpolation\\qq\\q3.txt", qww.stderr).unwrap();
    let mut e =3;
    let mut r;
    while e >= 3
    {
    match File::open(".\\frame_interpolation\\qq\\interpolated.mp4") {
        Err(p) => {sleep(Duration::from_millis(22));},
        _ => {e =2;
            r=File::open(".\\frame_interpolation\\qq\\interpolated.mp4").unwrap();
    let mut p = vec![];
    r.read_to_end(&mut p);
    sock.send(axum::extract::ws::Message::Binary(p)).await.unwrap();
            },
    };
    }
    sock.recv().await.unwrap().unwrap();
    })
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
        "/ffmpeg.min.js", get(pkgjs));
    let q = "80"
        .to_string();
    axum::Server::bind(&("0.0.0.0:".to_owned()+&q).parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
