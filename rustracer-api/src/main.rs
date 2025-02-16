use tide::{Request, Response};
use femme;
use rustracer_core::scene::Scene;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    femme::start();
    app.with(tide::log::LogMiddleware::new());

    app.at("/api/render").post(handle_render);
    app.at("/").get(|_| async { Ok("Hello, world!") });

    println!("Server running at http://127.0.0.1:8080");
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

async fn handle_render(mut req: Request<()>) -> tide::Result {
    let scene: Scene = req.body_json().await?;
    println!("{}", scene);
    let response = Response::builder(200)
        .header("Access-Control-Allow-Origin", "http://localhost:5173")
        .build();
    Ok(response)
}