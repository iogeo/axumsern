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
        let mut rqwp = 0;
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
        let qww = Command::new("sh")
            .arg("-c")
            .arg(format!("mkdir q{}", rq))
            .output()
            .unwrap();
        let rwq = client.get_object().bucket("axumserws").key(rqw.clone()+"/waiting/waiting.mp4").send().await.unwrap().body.collect().await.unwrap().into_bytes().to_vec();
        fs::write(format!("./q{}/waiting.mp4", rq), &rwq).unwrap();
        Command::new("sh")
            .arg("-c")
            .arg(format!("cd q{}\nffmpeg -y -i waiting.mp4 -preset ultrafast -chroma_sample_location top waiting.webm", rq))
            .output()
            .unwrap();
        Command::new("sh")
            .arg("-c")
            .arg(format!("cd q{}\nffmpeg -y -i waiting.webm -preset ultrafast -chroma_sample_location top waiting.mp4", rq))
            .output()
            .unwrap();
        let rwq = client.get_object().bucket("axumserws").key(rqw.clone()+"/end/end.mp4").send().await.unwrap().body.collect().await.unwrap().into_bytes().to_vec();
        fs::write(format!("./q{}/end.mp4", rq), &rwq).unwrap();
        Command::new("sh")
            .arg("-c")
            .arg(format!("cd q{}\nffmpeg -y -i end.mp4 -preset ultrafast -chroma_sample_location top end.webm", rq))
            .output()
            .unwrap();
        Command::new("sh")
            .arg("-c")
            .arg(format!("cd q{}\nffmpeg -y -i end.webm -preset ultrafast -chroma_sample_location top end.mp4", rq))
            .output()
            .unwrap();
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
            if qw != "waiting"{
                let rwq = client.get_object().bucket("axumserws").key(rqw.clone()+&pq).send().await.unwrap().body.collect().await.unwrap().into_bytes().to_vec();
                fs::write(format!("./q{}/", rq)+&qw.clone()+".mp4", &rwq).unwrap();
                let qww = Command::new("sh")
                .arg("-c")
                .arg(format!("cd q{}\nffmpeg -y -i ", rq)+&qw.clone()+".mp4 -chroma_sample_location top -preset ultrafast "+&qw.clone()+".webm")
                .output()
                .unwrap();
                let qww = Command::new("sh")
                .arg("-c")
                .arg(format!("cd q{}\nffmpeg -y -i ", rq)+&qw.clone()+".webm -chroma_sample_location top -preset ultrafast "+&qw.clone()+".mp4")
                .output()
                .unwrap();
            }
            q += "-i ";
            q += &qw;
            q += ".mp4 ";
            rqwp += 1;
            let qwl = sock.recv().await;
            match sock.send(axum::extract::ws::Message::Text("e".to_string())).await{
                Ok(p) => {
                    qw = qwl.unwrap().unwrap().into_text().unwrap();
                }
                _ => {
                    qw = "w".to_string();
                }
            }
        }
        q += "-i end.mp4";
        fs::write(format!("./q{}/concat", rq), &q.to_string().as_bytes()).unwrap();
        let qww = Command::new("sh")
            .arg("-c")
            .arg(format!("cd q{}\nffmpeg -y {} -filter_complex 'concat=n={}' -chroma_sample_location top -preset ultrafast final.mp4", rq, q, rqwp))
            .output()
            .unwrap();
        let mut r = File::open(format!("./q{}/final.mp4", rq)).unwrap();
        let mut p = vec![];
        r.read_to_end(&mut p);
        let rq = client.put_object().bucket("axumserws").key(rqw+"/final/"+&rq.to_string()+".mp4").body(ByteStream::new(SdkBody::from(p.clone()))).send().await.unwrap();
        sock.send(axum::extract::ws::Message::Binary(p)).await.unwrap();
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
