use poem::{
    http::StatusCode,
    web::{
        headers,
        headers::{authorization::Bearer, HeaderMapExt},
    },
    Endpoint, Error, Middleware, Request, Result,
};

pub struct Auth;

impl<E: Endpoint> Middleware<E> for Auth {
    type Output = AuthEndpoint<E>;

    fn transform(&self, ep: E) -> Self::Output {
        AuthEndpoint { ep }
    }
}

pub struct AuthEndpoint<E> {
    ep: E,
}

#[poem::async_trait]
impl<E: Endpoint> Endpoint for AuthEndpoint<E> {
    type Output = E::Output;

    async fn call(&self, req: Request) -> Result<Self::Output> {
        if req.uri() == "/login" {
            return self.ep.call(req).await;
        }

        if let Some(_auth) = req.headers().typed_get::<headers::Authorization<Bearer>>() {
            self.ep.call(req).await
        } else {
            Err(Error::from_status(StatusCode::UNAUTHORIZED))
        }
    }
}
