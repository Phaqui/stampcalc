use std::env;

use actix_web::{get, web, App, HttpServer, HttpResponse, Responder};
use serde::Deserialize;

use libstampcalc::Solutions;

#[derive(Deserialize)]
struct Input {
    price: u32,
    stamps: String,
}

fn parse_stamps(stamps: &str) -> Result<Vec<u32>, ()> {
    stamps.split(",")
          .map(|stamp| stamp.parse::<u32>().unwrap_or_default())
          .map(|int| if int == 0 { Err(()) } else { Ok(int) })
          .collect::<Result<Vec<u32>, _>>()
}

fn make_response_body(price: u32, stamps: &Vec<u32>) -> String {
    let mut solutions = Solutions::new(price, &stamps);
    let answers = solutions.make_into_iterator();
    let mut output: Vec<String> = Vec::new();
    for result in answers {
        let s = format!("{:?}", result);
        output.push(s);
    }
    output.join("\n")
}
#[get("/{price}/{stamps}")]
async fn index(input: web::Path<Input>) -> impl Responder {
    match parse_stamps(&input.stamps) {
        Err(_) => {
            HttpResponse::BadRequest().body("stamps must be positive integers")
        },
        Ok(stamps) => {
            let body = make_response_body(input.price, &stamps);
            HttpResponse::Ok().body(body)
        }
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = env::var("PORT").unwrap_or("8000".to_string());
    HttpServer::new(|| {
        App::new()
            .service(index)
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}

