// use async_trait::async_trait;
// use axum::{
//     extract::FromRequestParts,
//     http::{StatusCode, header::AUTHORIZATION, request::Parts},
// };

// use crate::models::Claims;

// #[async_trait]
// impl<S> FromRequestParts<S> for Claims
// where
//     S: Send + Sync,
// {
//     type Rejection = StatusCode;

//     async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
//         // 1. Get the Authorization header
//         let auth_header = parts
//             .headers
//             .get(AUTHORIZATION)
//             .and_then(|h| h.to_str().ok())
//             .ok_or(StatusCode::UNAUTHORIZED)?;

//         if !auth_header.starts_with("Bearer ") {
//             return Err(StatusCode::UNAUTHORIZED);
//         }

//         let token = &auth_header[7..]; // Strip "Bearer "

//         // 2. Validate the token
//         let token_data = jsonwebtoken::decode::<Claims>(
//             token,
//             &jsonwebtoken::DecodingKey::from_secret("your_super_secret_key".as_ref()),
//             &jsonwebtoken::Validation::default(),
//         )
//         .map_err(|_| StatusCode::UNAUTHORIZED)?;

//         Ok(token_data.claims)
//     }
// }
