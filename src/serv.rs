use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn handle_query(info: web::Query<QueryParams>) -> impl Responder {
    let response = format!("varname: {}, varname2: {}", info.varname, info.varname2);
    println!("varname: {}, varname2: {}", info.varname, info.varname2);

    //can change the response to something else
    HttpResponse::Ok().body(response)
}

#[derive(serde::Deserialize)]
struct QueryParams {
    varname: String,
    varname2: String,
}
#[actix_web::main]
pub(crate) async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route("/", web::get().to(handle_query))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
