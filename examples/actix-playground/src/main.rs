use actix_web::{test, App, Error};
use paperclip::actix::api_v2_operation;
use paperclip::actix::web;
use paperclip::actix::{Apiv2Schema, OpenApiExt};
use paperclip::v2::models::Tag;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Apiv2Schema)]
#[serde(rename_all = "camelCase")]
/// Pets are awesome!
pub struct Pet {
    /// Pick a good one.
    name: String,
    id: Option<uuid::Uuid>,
}

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct Output {
    /// Answer on question
    answer: String,
}

#[derive(Deserialize, Serialize, Apiv2Schema)]
#[serde(rename_all = "camelCase")]
/// Pets are awesome!
pub struct AbstractPet<P> {
    /// Kind of a pet.
    kind: P,
    id: Option<u64>,
}

/// Some simple pet
///
/// Pet with 4 legs like a cat or dog.
#[api_v2_operation("pets,dogs,cats")]
async fn some_pet(_data: web::Data<String>, _pet: web::Json<Pet>) -> Result<web::Json<Pet>, Error> {
    Ok(web::Json(Pet {
        name: "my puppy".to_string(),
        id: None,
    }))
}

/// Any kind of a pet
#[api_v2_operation]
async fn abstract_pet<P, T: 'static>(
    _data: web::Data<T>,
    mut _pet: web::Json<AbstractPet<P>>,
) -> Result<web::Json<Pet>, Error>
where
    P: Serialize + for<'de> Deserialize<'de> + 'static,
{
    Ok(web::Json(Pet {
        name: "my super puppy".to_string(),
        id: Some(1),
    }))
}


/// Any kind of a pet
#[api_v2_operation]
async fn get_answer() -> Result<web::Json<Output>, Error>
{
    Ok(web::Json(Output { answer: "earth has a shape of dino".to_string() }))
}

#[actix_rt::main]
async fn main() {
    let mut tags = HashMap::new();
    tags.insert(
        "pets",
        vec![Tag {
            name: "pets".to_string(),
            description: Some("Pets".to_string()),
            external_docs: None,
        }],
    );

    let mut tag_vec = vec![];
    for tag in tags.values() {
        tag_vec.extend(tag.clone());
    }

    let mut app = test::init_service(
        App::new()
            .wrap_api()
            .update_tags(tag_vec)
            .service(
                web::resource("/random")
                    .route(web::post().to(some_pet))
                    .route(web::get().to(abstract_pet::<String, u16>)),
                    .route(web::get().to(get_answer))
            )
            .with_json_spec_at("/api/spec")
            .build(),
    )
    .await;

    let req = test::TestRequest::with_uri("/api/spec").to_request();
    let res = String::from_utf8(test::read_response(&mut app, req).await.to_vec()).unwrap();
    println!("{}", res);
}
