mod render;

use tide::{Body, Request, Response};
use femme;
use rustracer_core::scene::Scene;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    femme::start();
    app.with(tide::log::LogMiddleware::new());

    app.at("/api/render").post(render);
    app.at("/").get(|_| async { Ok("Hello, world!") });

    println!("Server running at http://127.0.0.1:8080");
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

async fn render(mut req: Request<()>) -> tide::Result {
    let scene: Scene = req.body_json().await?;
    println!("Rendering scene: {:?}", scene);
    let image = render::handle_render(scene);
    let mut response = Response::new(200);
    response.set_body(Body::from_bytes(image));
    response.set_content_type("image/jpeg");
    Ok(response)
}