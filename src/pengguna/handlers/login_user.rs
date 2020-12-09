//! # Module Login User Handler
//!
//! Module ini digunakan untuk login pengguna sebagai `handlers`.
//!
//! <br />
//!
//! # Contoh
//!
//! ```rust
//! use crate::pengguna::handlers::login_user::{...}
//! ```
use mongodb::Database;
use actix_web::{
    web,
    post,
    HttpResponse,
};
use crate::app::errors::AppErrors;
use crate::app::dto::UmpanBalik;
use crate::pengguna::{
    services::login_user,
    dto::LoginPenggunaDto,
};


/// # Fungsi masuk
///
/// Fungsi ini untuk menampilkan _response_ umpan balik hasil masuk pengguna sesuai inputan
/// pengguna saat mengunjungi _endpoint root_ `v1/pengguna/login`.
///
/// <br />
///
/// # Masukan
///
/// * `payload` - inputan pengguna berupa email dan password.
/// * `db` - mongodb Database type yang dishare melalui _application state_.
///
/// <br />
///
/// # Keluaran
///
/// * `Result<HttpResponse, AppErrors>` - keluaran berupa _enum_ `Result` yang terdiri dari kumpulan
/// `HttpResponse` dan _Enum_ `AppErrors`.
#[post("/pengguna/login/")]
pub async fn masuk(
    payload: web::Form<LoginPenggunaDto>,
    db: web::Data<Database>,
) -> Result<HttpResponse, AppErrors> {
    let valid = login_user::verify(payload, db).await?;

    if valid.is_none() {
        Ok(HttpResponse::NotFound().json(UmpanBalik::<()> {
            sukses: false,
            pesan: "Email/Password tidak ditemukan".to_string(),
            hasil: (),
        }))
    } else {
        Ok(HttpResponse::Accepted().json(UmpanBalik::<Option<String>> {
            sukses: true,
            pesan: "Pengguna tervalidasi".to_string(),
            hasil: valid,
        }))
    }
}