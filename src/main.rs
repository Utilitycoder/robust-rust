use robust_rust::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    run()?.await
}
