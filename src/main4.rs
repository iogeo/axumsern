use std::io::prelude::*;
use axum::{Json, response::{Html, IntoResponse, Response},routing::get,Router,http::{Uri, header::{self, HeaderMap, HeaderName}}};
use std::fs::File;
use std::fs;
use std::str::FromStr;
use std::env;
use axum::body::Full;
use std::process::Command;
use axum::extract::WebSocketUpgrade;
use std::thread::sleep;
use core::time::Duration;
use hyper::{Body, Method, Client, Request};
use aws_sdk_s3::types::ByteStream;
use aws_smithy_http::body::SdkBody;

struct js
{
    String:String
}

async fn response() -> axum::http::response::Builder {
    Response::builder()
}

async fn root(ws: WebSocketUpgrade) -> impl IntoResponse{
    ws.on_upgrade(move |mut sock| async move{
        let rqw = sock.recv().await.unwrap().unwrap().into_text().unwrap();
        let mut qw = sock.recv().await.unwrap().unwrap().into_text().unwrap();
        let mut q = String::new();
        let mut qp = 3;
        let mut rq = 2;
        let mut e = String::new();
        let shared_config = aws_config::load_from_env().await;
        let client = aws_sdk_s3::Client::new(&shared_config);
        while qp >= 3
        {
            match File::open("visits") {
                Err(p) => {sleep(Duration::from_millis(22));},
                _ => {
            let mut r=File::open("visits").unwrap();
            r.read_to_string(&mut e);
                    println!("{}", &e);
            rq = i32::from_str(&e).unwrap();
            rq += 1;
            fs::write("visits", rq.to_string().as_bytes()).unwrap();
            qp = 2;
        }}}
        while qw != "w"{
            let pq;
            let pqp = &("/qw/".to_string()+&qw+".mp4");
            if qw == "intro"
            {
                pq = "/intro/intro.mp4";
            }
            else if qw == "waiting"
            {
                pq = "/waiting/waiting.mp4";
            }
            else if qw == "end"
            {
                pq = "/end/end.mp4";
            }
            else if qw == "endwarn"
            {
                pq = "/endwarn/endwarn.mp4";
            }
            else if qw == "noreply"
            {
                pq = "/noreply/noreply.mp4";
            }
            else
            {
                pq = pqp;
            }
            let rwq = client.get_object().bucket("axumserws").key(rqw.clone()+&pq).send().await.unwrap().body.collect().await.unwrap().into_bytes().to_vec();
            fs::write(qw.clone()+".mp4", &rwq).unwrap();
            q += "file ";
            q += &qw;
            q += "\n";
            qw = sock.recv().await.unwrap().unwrap().into_text().unwrap();
        }
        q += "file end.mp4\n";
        fs::write("concat", &q.to_string().as_bytes()).unwrap();
        let qww = Command::new("sh")
            .arg("-c")
            .arg(format!("ffmpeg -f concat -safe 0 -i concat -preset ultrafast final.mp4"))
            .output()
            .unwrap();
        let mut r = File::open("final.mp4").unwrap();
        let mut p = vec![];
        r.read_to_end(&mut p);
        let qww2 = Command::new("sh")
            .arg("-c")
            .arg(format!("rm -r concat\n"))
            .output()
            .unwrap();
        let rq = client.put_object().bucket("axumserws").key(rqw+"/final/"+&rq.to_string()+".mp4").body(ByteStream::new(SdkBody::from(p))).send().await.unwrap();
    })
}

async fn index2() -> impl IntoResponse{
    fs::write("visits", "2").unwrap();
    response()
        .await.status(200)
        .header("Content-Type","text/html; charset=UTF-8")
        .header("Cross-Origin-Embedder-Policy","require-corp")
        .header("Cross-Origin-Opener-Policy","same-origin")
        .body(Full::from("p"))
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
