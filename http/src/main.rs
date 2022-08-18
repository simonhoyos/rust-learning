use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct SumParameters {
    n: u64,
    m: u64,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .service(post_sum)
    });

    println!("App running at http://localhost:3000");

    server
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}

async fn get_index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
                <title>Sum Calculator</title>
                <form action="/sum" method="post">
                    <input type="text" name="n" />
                    <input type="text" name="m" />
                    <button type="submit">Compute sum</button>
                </form>
            "#
        )
}

#[post("/sum")]
async fn post_sum(form: web::Form<SumParameters>) -> impl Responder {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("couldn't compute")
    }

    let response = format!("The sum of the numbers {} and {} \
                            is <b>{}</b>\n",
                            form.n, form.m, sum(form.n, form.m));

    HttpResponse::Ok()
        .content_type("text/html")
        .body(response)
}

fn sum(n: u64, m: u64) -> u64 {
    n + m
}

#[test]
fn sum_test() {
    assert_eq!(sum(1, 2), 3)
}
