use paperclip::actix::web;
use paperclip::actix::{OpenApiExt, Apiv2Schema};
use paperclip::actix::api_v2_operation;
use actix_web::{App, Error, test};
use serde::{Serialize, Deserialize};
use paperclip::v2::models::Tag;
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Apiv2Schema)]
#[serde(rename_all = "camelCase")]
/// Pets are awesome!
pub struct Pet {
    /// Pick a good one.
    name: String,
    id: Option<u64>,
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
    Ok(web::Json(Pet { name: "my puppy".to_string(), id: None }))
}

/// Any kind of a pet
#[api_v2_operation()]
async fn abstract_pet<P, T: 'static>(_data: web::Data<T>, mut _pet: web::Json<AbstractPet<P>>) -> Result<web::Json<Pet>, Error>
where P: Serialize + for <'de> Deserialize< 'de> + 'static
{
    Ok(web::Json(Pet { name: "my super puppy".to_string(), id: Some(1) }))
}

#[actix_rt::main]
async fn main() {

    let mut tags = HashMap::new();
    tags.insert("pets", vec![Tag {name: "pets".to_string(), description: Some("Pets".to_string()), external_docs: None}]);

    let mut tag_vec = vec![];
    for tag in tags.values() {
        tag_vec.extend(tag.clone());
    }

    let mut app_wrapper = App::new()
        .wrap_api();
    app_wrapper.update_tags(tag_vec);
    let mut app = test::init_service(
        app_wrapper
        .service(web::resource("/random")
            .route(web::post().to(some_pet))//.tags(tags.get("pets").unwrap())
            .route(web::get().to(abstract_pet::<String, u16>))//.tags(tags.get("pets").unwrap())
        )

        .with_json_spec_at("/api/spec")
        .build()
    ).await;

    let req = test::TestRequest::with_uri("/api/spec").to_request();
    let res = String::from_utf8(test::read_response(&mut app, req).await.to_vec()).unwrap();
    println!("{}", res);
}
