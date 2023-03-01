/*An actix Microservice that has multiple routes:
A.  / that turns a hello world
B. /fruit that returns a random fruit
C. /health that returns a 200 status code
D. /version that returns the version of the service
*/

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
//import the random fruit function from the lib.rs file
//use webdocker::random_fruit;

//create a function that returns a hello world
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the Inter-Currency Exchange Rate Service.\nYou can use '/exchange_rate' to find out which currency you can view the exchange rate with RMB in this service")
}

//create a function that returns a random fruit
#[get("/exchange_rate")]
async fn exchange_rate() -> impl Responder {
    HttpResponse::Ok().body("In this service you can check the exchange rates of these currencies:\nUSD, CAD, GBP, JPY, HKD, EUR, KRW, AUD...\nYou can use '/exchange_rate/<currency name>'\nFor example: '/exchange_rate/USD'")
}

#[get("/exchange_rate/USD")]
async fn usd() -> impl Responder {
    HttpResponse::Ok().body("1 RMB = 0.1456 USD\n1 USD = 6.866000 RMB")
}

#[get("/exchange_rate/JPY")]
async fn jpy() -> impl Responder {
    HttpResponse::Ok().body("1 RMB = 19.719600 JPY\n1 JPY = 0.0507 RMB")
}

#[get("/exchange_rate/GBP")]
async fn gbp() -> impl Responder {
    HttpResponse::Ok().body("1 RMB = 0.120730 GBP\n1 GBP = 8.2836 RMB")
}

#[get("/exchange_rate/CAD")]
async fn cad() -> impl Responder {
    HttpResponse::Ok().body("1 RMB = 0.198000 CAD\n1 CAD = 5.0521 RMB")
}

#[get("/exchange_rate/HKD")]
async fn hkd() -> impl Responder {
    HttpResponse::Ok().body("1 RMB = 1.143350 HKD\n1 HKD = 0.8746 RMB")
}

#[get("/exchange_rate/EUR")]
async fn eur() -> impl Responder {
    HttpResponse::Ok().body("1 RMB = 0.136330 EUR\n1 EUR = 7.3351 RMB")
}

#[get("/exchange_rate/KRW")]
async fn krw() -> impl Responder {
    HttpResponse::Ok().body("1 RMB = 189.430000 KRW\n1 KRW = 0.0053 RMB")
}

#[get("/exchange_rate/AUD")]
async fn aud() -> impl Responder {
    HttpResponse::Ok().body("1 RMB = 0.214800 AUD\n1 AUD = 4.6555 RMB")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //add a print message to the console that the service is running
    println!("Running the service");
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(exchange_rate)
            .service(usd)
            .service(jpy)
            .service(gbp)
            .service(cad)
            .service(hkd)
            .service(eur)
            .service(krw)
            .service(aud)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
