// use crate::{
//     models::models::{AuthResponse, Claims, CreateUser, ErrorResponse, LoginUser, User},
//     utils::constants::SALT_ROUNDS,
// };
// use actix_web::{HttpResponse, Responder, web};
// use bcrypt::{hash, verify};
// use chrono::{Duration, Utc};
// use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
// use sqlx::{Error, PgPool, Row, postgres::PgRow};

// //pub struct AuthService;

// // Helper function to convert PgRow to User
// fn row_to_user(row: PgRow) -> User {
//     User {
//         id: row.get("id"),
//         user_name: row.get("name"),
//         email: row.get("email"),
//         password_hash: row.get("password_hash"),
//         created_at: row.get("created_at"),
//         updated_at: row.get("updated_at"),
//     }
// }

// pub async fn sign_up_user(
//     pool: &PgPool,
//     user: &CreateUser,
// ) -> Result<User, Box<dyn std::error::Error>> {
//     // Check if user already exists
//     let existing_user = sqlx::query("SELECT id FROM users WHERE email = $1")
//         .bind(&user.email)
//         .fetch_optional(pool)
//         .await?;

//     if existing_user.is_some() {
//         return Err("User Already Exists".into());
//     }

//     // Hash password
//     let password_hash = hash(&user.password, SALT_ROUNDS)?;

//     // Create user
//     let new_user_row = sqlx::query(
//         "INSERT INTO users (name, email, password_hash, created_at, updated_at) VALUES ($1, $2, $3, NOW(), NOW()) RETURNING id, name, email, password_hash, created_at, updated_at",
//     )
//     .bind(&user.username)
//     .bind(&user.email)
//     .bind(password_hash)
//     .fetch_one(pool)
//     .await?;

//     Ok(row_to_user(new_user_row))
// }

// pub async fn register_user(
//     pool: web::Data<PgPool>,
//     payload: web::Json<CreateUser>,
// ) -> impl Responder {
//     let payload = payload.into_inner();

//     match sign_up_user(pool.get_ref(), &payload).await {
//         Ok(user) => match generate_token(&user) {
//             Ok(token) => HttpResponse::Ok().json(AuthResponse { token, user }),
//             Err(_) => HttpResponse::InternalServerError().json(ErrorResponse {
//                 error: "Failed To Generate Token".to_string(),
//             }),
//         },
//         Err(e) => HttpResponse::BadRequest().json(ErrorResponse {
//             error: e.to_string(),
//         }),
//     }
// }

// /// Logs User In
// pub async fn login_user(pool: &PgPool, user: &LoginUser) -> Result<User, sqlx::Error> {
//     let user_row = sqlx::query(
//         "SELECT id, name, email, password_hash, created_at, updated_at FROM users WHERE email = $1",
//     )
//     .bind(&user.email)
//     .fetch_optional(pool)
//     .await?;

//     match user_row {
//         Some(row) => Ok(row_to_user(row)),
//         None => Err(sqlx::Error::RowNotFound),
//     }
// }

// pub async fn sign_in_user(
//     pool: web::Data<PgPool>,
//     payload: web::Json<LoginUser>,
// ) -> impl Responder {
//     let payload = payload.into_inner();

//     let user = match login_user(pool.get_ref(), &payload).await {
//         Ok(user) => user,
//         Err(e) => {
//             return match e {
//                 Error::RowNotFound => HttpResponse::NotFound().json(ErrorResponse {
//                     error: "User not found".to_string(),
//                 }),
//                 _ => HttpResponse::InternalServerError().json(ErrorResponse {
//                     error: "Internal server error".to_string(),
//                 }),
//             };
//         }
//     };

//     let is_valid = match verify(&payload.password, &user.password_hash) {
//         Ok(valid) => valid,
//         Err(_) => {
//             return HttpResponse::InternalServerError().json(ErrorResponse {
//                 error: "Password Verification Failed".to_string(),
//             });
//         }
//     };

//     if !is_valid {
//         return HttpResponse::Unauthorized().json(ErrorResponse {
//             error: "Invalid Credentials".to_string(),
//         });
//     }

//     match generate_token(&user) {
//         Ok(token) => HttpResponse::Ok().json(AuthResponse { token, user }),
//         Err(_) => HttpResponse::InternalServerError().json(ErrorResponse {
//             error: "Failed to generate token".to_string(),
//         }),
//     }
// }

// pub fn generate_token(user: &User) -> Result<String, jsonwebtoken::errors::Error> {
//     let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key".to_string());

//     let now = Utc::now();
//     let expires_in = Duration::hours(72);

//     let claims = Claims {
//         sub: user.id.to_string(),
//         exp: (now + expires_in).timestamp() as usize,
//         iat: now.timestamp() as usize,
//         email: user.email.clone(),
//     };

//     let header = Header::new(Algorithm::HS256);
//     let encoding_key = EncodingKey::from_secret(secret.as_ref());

//     encode(&header, &claims, &encoding_key)
// }
