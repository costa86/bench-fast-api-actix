use actix_web::{get, web, App, HttpServer, Responder, Result};
use serde::Serialize;
use std::error::Error;

#[derive(Debug, Serialize, Clone)]
struct Dress {
    sku: String,
    style: String,
    code: String,
    url: String,
}

fn get_dresses() -> Result<Vec<Dress>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path("dress.csv")?;
    let mut dresses_list: Vec<Dress> = vec![];

    for result in rdr.records() {
        let record = result?;
        let dress = Dress {
            sku: record.get(0).unwrap().to_string(),
            style: record.get(1).unwrap().to_string(),
            code: record.get(2).unwrap().to_string(),
            url: record.get(3).unwrap().to_string(),
        };
        dresses_list.push(dress);
    }
    Ok(dresses_list)
}

#[get("/dresses")]
async fn dresses_get() -> Result<impl Responder> {
    let dresses = get_dresses().unwrap().clone();
    Ok(web::Json(dresses))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(dresses_get))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
