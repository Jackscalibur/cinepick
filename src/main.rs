use tide::{Request, Response, StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct RecommendQuery {
    user_id: u32,
}

#[derive(Debug, Serialize)]
struct Recommendation {
    film_id: u32,
    title: String,
}

async fn recommend(req: Request<()>) -> tide::Result {
    let query: RecommendQuery = req.query()?;

    // Dummy response for now
    let recs = vec![
        Recommendation {
            film_id: 1,
            title: "Inception".to_string(),
        },
        Recommendation {
            film_id: 2,
            title: "The Matrix".to_string(),
        },
    ];

    let mut response = Response::new(StatusCode::Ok);
    response.set_body(tide::Body::from_json(&recs)?);
    Ok(response)
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/recommend").get(recommend);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
