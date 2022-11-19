use actix_web::HttpResponse;
use anyhow::Result;
use serde::Serialize;

pub trait Mappable<Source, Destination>
where
    Destination: Serialize,
{
    fn map(self, mapper: fn(&Source) -> Destination) -> HttpResponse;
}

impl<Source, Destination> Mappable<Source, Destination> for Option<Source>
where
    Destination: Serialize,
{
    fn map(self, mapper: fn(&Source) -> Destination) -> HttpResponse {
        match self {
            Some(i) => HttpResponse::Ok().json(mapper(&i)),
            None => HttpResponse::NotFound().finish(),
        }
    }
}

impl<Source, Destination> Mappable<Source, Destination> for Vec<Source>
where
    Destination: Serialize,
{
    fn map(self, mapper: fn(&Source) -> Destination) -> HttpResponse {
        let x = self.iter().map(mapper).collect::<Vec<Destination>>();

        HttpResponse::Ok().json(x)
    }
}

pub fn result_to_response<Container, Source, Destination>(
    result: Result<Container>,
    mapper: fn(&Source) -> Destination,
) -> HttpResponse
where
    Container: Mappable<Source, Destination>,
    Destination: Serialize,
{
    match result {
        Ok(container) => container.map(mapper),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
