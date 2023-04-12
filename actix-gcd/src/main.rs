use actix_web::{ HttpServer, App, HttpResponse, web};

use serde::Deserialize;

fn main() {
    println!("Hello, world!");

    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
    });

    println!("Server running on http://localhost:3000");

    server
        .bind("127.0.0.1:3000").expect("error binding to port")
        .run().expect("error occured while starting the server")
}

fn get_index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"<html>
                <head><title>GCD Calculator</title></head>
                <body>
                <p><b>Hello</b> <i>World</i></p>
                <form
                    method="POST"
                    action="/gcd"
                >
                    <label>n <input name="n" type="number" placeholder="n"></label>
                    <label>m <input name="m" type="number" placeholder="m"></label>
                    <button type="submit">Calculate</button>
                </form>
                </body>
                </html>
            "#
            )
}

#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}

fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .body("Cannot calculate greatest common denominator if one of the values is zero");
    }

    let response = format!("The greatest common denominator of {} and {} is <strong>{}</strong>",
        form.n,
        form.m,
        gcd(form.n, form.m));

    HttpResponse::Ok()
        .content_type("text/html")
        .body(response)
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n > 0 && m > 0);

    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);

    assert_eq!(gcd(2 * 3 * 5 * 11 * 17,
        3 * 7 * 11 * 13 * 19), 3 * 11)
}