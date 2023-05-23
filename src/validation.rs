use rocket::{
  data::{Data, FromData, Outcome as DataOutcome},
  form,
  form::{DataField, FromForm, ValueField},
  http::Status,
  outcome::Outcome,
  request::{FromRequest, Request},
  serde::{json::Json, Serialize},
};
use std::fmt::Debug;
pub use validator::{Validate, ValidationErrors};

////  Struct used for Request Guards
#[derive(Clone, Debug)]
pub struct Validated<T>(pub T);

///  Impl to get type T of `Json`
impl<T> Validated<Json<T>> {
  #[inline]
  pub fn into_deep_inner(self) -> T {
    self.0 .0
  }
}

///  Impl to get type T
impl<T> Validated<T> {
  #[inline]
  pub fn into_inner(self) -> T {
    self.0
  }
}

///  Struct representing errors sent by the catcher
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Error<'a> {
  code: u128,
  message: &'a str,
  errors: Option<&'a ValidationErrors>,
}

///  Catcher to return validation errors to the client
///  ```rust
///  # #[macro_use] extern crate rocket;
///  #[launch]
///  fn rocket() -> _ {
///      rocket::build()
///          .mount("/", routes![/*validated_hello*/])
///  /* right here ---->*/.register("/", catchers![rocket_validation::validation_catcher])
///  }
///  ```
#[catch(400)]
pub fn validation_catcher<'a>(req: &'a Request) -> Json<Error<'a>> {
  Json(Error {
    code: 400,
    message: "Bad Request. The request could not be understood by the server due to malformed \
                  syntax.",
    errors: req.local_cache(|| CachedValidationErrors(None)).0.as_ref(),
  })
}

///  Wrapper used to store `ValidationErrors` within the scope of the request
#[derive(Clone)]
pub struct CachedValidationErrors(pub Option<ValidationErrors>);

///  Implementation of `Validated` for `Json`
//
///  An example with `Json`
///  ```rust
///  # #[macro_use] extern crate rocket;
///  use rocket::serde::{json::Json, Deserialize, Serialize};
///  use rocket_validation::{Validate, Validated};
///  
///  #[derive(Debug, Deserialize, Serialize, Validate)]
///  #[serde(crate = "rocket::serde")]
///  pub struct HelloData {
///      #[validate(length(min = 1))]
///      name: String,
///      #[validate(range(min = 0, max = 100))]
///      age: u8,
///  }
//
///  #[post("/hello", format = "application/json", data = "<data>")]
///  fn validated_hello(data: Validated<Json<HelloData>>) -> Json<HelloData> {
///      Json(data.into_deep_inner())
///  }
///  
///  #[launch]
///  fn rocket() -> _ {
///      rocket::build()
///          .mount("/", routes![validated_hello])
///          .register("/", catchers![rocket_validation::validation_catcher])
///  }
///  ```
#[rocket::async_trait]
impl<'r, D: Validate + rocket::serde::Deserialize<'r>> FromData<'r> for Validated<Json<D>> {
  type Error = Result<ValidationErrors, rocket::serde::json::Error<'r>>;

  async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> DataOutcome<'r, Self> {
    let data_outcome = <Json<D> as FromData<'r>>::from_data(req, data).await;

    match data_outcome {
      Outcome::Failure((status, err)) => Outcome::Failure((status, Err(err))),
      Outcome::Forward(err) => Outcome::Forward(err),
      Outcome::Success(data) => match data.validate() {
        Ok(_) => Outcome::Success(Validated(data)),
        Err(err) => {
          req.local_cache(|| CachedValidationErrors(Some(err.to_owned())));
          Outcome::Failure((Status::BadRequest, Ok(err)))
        }
      },
    }
  }
}

///  Implementation of `Validated` for `FromRequest` implementing `Validate`
//
///  Anything you implement `FromRequest` for as well as `Validate`
#[rocket::async_trait]
impl<'r, D: Validate + FromRequest<'r>> FromRequest<'r> for Validated<D> {
  type Error = Result<ValidationErrors, D::Error>;
  async fn from_request(req: &'r Request<'_>) -> rocket::request::Outcome<Self, Self::Error> {
    let data_outcome = D::from_request(req).await;

    match data_outcome {
      Outcome::Failure((status, err)) => Outcome::Failure((status, Err(err))),
      Outcome::Forward(err) => Outcome::Forward(err),
      Outcome::Success(data) => match data.validate() {
        Ok(_) => Outcome::Success(Validated(data)),
        Err(err) => {
          req.local_cache(|| CachedValidationErrors(Some(err.to_owned())));
          Outcome::Failure((Status::BadRequest, Ok(err)))
        }
      },
    }
  }
}

///  Implementation of `Validated` for `FromForm`
///
///  An example validating a query struct
///  ```rust
///  # #[macro_use] extern crate rocket;
///  use rocket::serde::{json::Json, Deserialize, Serialize};
///  use rocket_validation::{Validate, Validated};
///  
///  #[derive(Debug, Deserialize, Serialize, Validate, FromForm)]
///  #[serde(crate = "rocket::serde")]
///  pub struct HelloData {
///      #[validate(length(min = 1))]
///      name: String,
///      #[validate(range(min = 0, max = 100))]
///      age: u8,
///  }
//
///  #[get("/validated-hello?<params..>", format = "application/json")]
///  fn validated_hello(params: Validated<HelloData>) -> Json<HelloData> {
///      Json(params.into_inner())
///  }
///  
///  #[launch]
///  fn rocket() -> _ {
///      rocket::build()
///          .mount("/", routes![validated_hello])
///          .register("/", catchers![rocket_validation::validation_catcher])
///  }
///  ```
#[rocket::async_trait]
impl<'r, T: Validate + FromForm<'r>> FromForm<'r> for Validated<T> {
  type Context = T::Context;

  #[inline]
  fn init(opts: form::Options) -> Self::Context {
    T::init(opts)
  }

  #[inline]
  fn push_value(ctxt: &mut Self::Context, field: ValueField<'r>) {
    T::push_value(ctxt, field)
  }

  #[inline]
  async fn push_data(ctxt: &mut Self::Context, field: DataField<'r, '_>) {
    T::push_data(ctxt, field).await
  }

  fn finalize(this: Self::Context) -> form::Result<'r, Self> {
    match T::finalize(this) {
      Err(err) => Err(err),
      Ok(data) => match data.validate() {
        Ok(_) => Ok(Validated(data)),
        Err(err) => Err(
          err
            .into_errors()
            .into_iter()
            .map(|e| form::Error {
              name: Some(e.0.into()),
              kind: form::error::ErrorKind::Validation(std::borrow::Cow::Borrowed(e.0)),
              value: None,
              entity: form::error::Entity::Value,
            })
            .collect::<Vec<_>>()
            .into(),
        ),
      },
    }
  }
}
