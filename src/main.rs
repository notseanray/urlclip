use actix_web::{http::StatusCode, web::Data, HttpResponse};
use std::sync::Arc;
use std::{
    collections::HashMap,
    sync::RwLock,
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use actix_web::{get, web, App, HttpServer, Responder};
use lazy_static::lazy_static;
use rand::Rng;
lazy_static! {
    pub static ref LINKS: Arc<RwLock<HashMap<String, Link>>> =
        Arc::new(RwLock::new(HashMap::new()));
}
// one week
const KEEP_TIME: u64 = 604800;
const MAX_URLS: usize = 50000;
type Link = (String, u64);

macro_rules! time {
    () => {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs()
    };
}

fn generate_pair(data: &HashMap<String, Link>) -> String {
    let mut rng = rand::thread_rng();
    let mut result = String::with_capacity(4);
    loop {
        for _ in 0..4 {
            result.push(char::from_u32(rng.gen_range(97..122)).unwrap_or('a'));
        }
        if data.contains_key(&result) {
            result.clear();
        } else {
            break;
        }
    }
    result
}

const BG: &str = "<head><style>* { background-color: black; }</style></head>";

#[get("/{name}")]
async fn index(name: web::Path<String>) -> impl Responder {
    if let Some(v) = LINKS.read().unwrap().get(name.as_str()) {
        HttpResponse::new(StatusCode::OK).set_body(format!(
            "<html>{BG}<script>window.location=\"{}\"</script></html>",
            v.0.clone()
        ))
    } else {
        HttpResponse::new(StatusCode::NOT_FOUND).set_body("Not found".to_string())
    }
}

#[get("/new/{url:.*}")]
async fn new(url: web::Path<String>) -> impl Responder {
    if LINKS.read().unwrap().len() > MAX_URLS {
        HttpResponse::new(StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS)
            .set_body(format!("<html>{BG}Full</html>"));
    }
    let pair = generate_pair(&LINKS.read().unwrap());
    LINKS
        .write()
        .unwrap()
        .insert(pair.clone(), (url.to_string(), time!()));
    HttpResponse::new(StatusCode::OK)
        .set_body(format!("<html>{BG}<a href=\"/{pair}\">{pair}</a></html>"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    thread::spawn(|| loop {
        std::thread::sleep(Duration::from_secs(3600));
        let time = time!();
        *LINKS.clone().write().unwrap() = LINKS
            .clone()
            .write()
            .unwrap()
            .iter()
            .filter(|v| (v.1).1 + KEEP_TIME < time)
            .map(|x| (x.0.clone(), x.1.clone()))
            .collect::<HashMap<String, Link>>();
    });
    HttpServer::new(|| {
        App::new()
            .app_data(Data::new(LINKS.clone()))
            .service(new)
            .service(index)
    })
    .bind(("127.0.0.1", 5456))?
    .run()
    .await
}
