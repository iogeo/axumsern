use std::io::prelude::*;
use axum::{Json, response::{Html, IntoResponse, Response},routing::get,Router,http::{Uri, header::{self, HeaderMap, HeaderName}}};
use std::fs::File;
use std::fs;
use std::str::FromStr;
use std::env;
use axum::body::Full;
use std::process::Command;
use axum::extract::WebSocketUpgrade;
use shh;
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
    println!("q");
    sock.send(axum::extract::ws::Message::Text("e".to_string())).await.unwrap();
    let mut q =3;
    let mut rq =2;
    let mut e = String::new();
    while q >= 3
    {
    match File::open("visits") {
        Err(p) => {sleep(Duration::from_millis(22));},
        _ => {
    let mut r=File::open("visits").unwrap();
    r.read_to_string(&mut e);
    rq = i32::from_str(&e).unwrap();
    rq+=1;fs::write("visits", rq.to_string().as_bytes()).unwrap();q =2;}}}
    let qw=sock.recv().await.unwrap().unwrap().into_data();
    let qwp=sock.recv().await.unwrap().unwrap().into_data();
    println!("e");
    if qw.len()>= 9984894 || qwp.len()>= 9984894
    {panic!();}
    Command::new("sh")
            .arg("-c")
            .arg(format!("cd frame_interpolation\nmkdir qq{}",rq))
            .output()
            .expect("failed to execute process");
    fs::write(format!("./frame_interpolation/qq{}/q2.jpg", rq), qw).unwrap();
    fs::write(format!("./frame_interpolation/qq{}/q3.jpg", rq), qwp).unwrap();
    let mut q =3;
    while q >= 3
    {
    match File::open("qw") {
        Err(p) => {sleep(Duration::from_millis(22));
        sock.send(axum::extract::ws::Message::Text("e".to_string())).await.unwrap();
        sock.recv().await.unwrap().unwrap().into_data();},
        _ => {sleep(Duration::from_millis(22));
        sock.send(axum::extract::ws::Message::Text("e".to_string())).await.unwrap();
        sock.recv().await.unwrap().unwrap().into_data();
    let mut r=File::open("qw").unwrap();
    let mut e = String::new();
    r.read_to_string(&mut e);
    if e !="e"
    {fs::write("qw", "e".to_string().as_bytes()).unwrap();fs::write("qw", "e".to_string().as_bytes()).unwrap();q =2;}}}}
    fs::write("visits", &rq.to_string().as_bytes()).unwrap();
    let qww = Command::new("sh")
            .arg("-c")
            .arg(format!(r#"python3.9 -m frame_interpolation.eval.interpolator_cli --pattern "frame_interpolation/qq{}" --model_path eqqw/pretrained_models/film_net/Style/saved_model --times_to_interpolate 3 --output_video"#,rq))
            .spawn()
            .expect("failed to execute process");
    println!("w");
     q =3;
    let mut e =3;
    let mut r;
    while e >= 3
    {
    match File::open(format!("./frame_interpolation/qq{}/interpolated.mp4",rq)) {
        Err(p) => {sleep(Duration::from_millis(22));
        match sock.send(axum::extract::ws::Message::Text("e".to_string())).await
        {
        Err(p) => {sleep(Duration::from_millis(22));
    Command::new("sh")
            .arg("-c")
            .arg(format!("rm ./frame_interpolation/qq{} -r\nY\n",rq))
            .output()
            .expect("failed to execute process");fs::write("qw", "q".to_string().as_bytes()).unwrap();
    let mut r=File::open("qw").unwrap();
    let mut qwe = String::new();
    r.read_to_string(&mut qwe);e =2;
        while qwe=="e"{}; sock.send(axum::extract::ws::Message::Text("e".to_string())).await.unwrap();},
        _ => {}
        }
        sock.recv().await.unwrap().unwrap().into_data();},
        _ => {
    while q >= 3
    {
    match File::open("qw") {
        Err(p) => {sleep(Duration::from_millis(22));
        sock.send(axum::extract::ws::Message::Text("e".to_string())).await.unwrap();
        sock.recv().await.unwrap().unwrap().into_data();},
        _ => {
    println!("qq");
    fs::write("qw", "".to_string().as_bytes()).unwrap();q =2;}}}e =2;
    sleep(Duration::from_millis(222));
            r=File::open(format!("./frame_interpolation/qq{}/interpolated.mp4",rq)).unwrap();
    let mut p = vec![];
    r.read_to_end(&mut p);
    Command::new("sh")
            .arg("-c")
            .arg(format!("rm ./frame_interpolation/qq{} -r\nY\n",rq))
            .output()
            .expect("failed to execute process");
    sock.send(axum::extract::ws::Message::Binary(p)).await.unwrap();
            },
    };
    }
    sock.recv().await.unwrap().unwrap();
    })
}

async fn index2() -> impl IntoResponse{
    let mut f = 2;
    fs::write("qw", &f.to_string().as_bytes()).unwrap();
    fs::write("visits", &f.to_string().as_bytes()).unwrap();
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
