use std::env;
use actix_web::{web, HttpRequest};
use mongodb::{
    Database,
    bson::{self, doc, Document}
};
use actix_web::error::ErrorUnauthorized;
use jsonwebtoken::{decode, DecodingKey, Validation};
use crate::app::errors::AppErrors;
use crate::pengguna::models::Klaim;
use crate::app::permissions::UserPermissions;
use crate::pengguna::helpers::{PenggunaHelpers, PenggunaHelpersTrait};
use crate::app::helpers::{AppHelpers, AppHelpersTrait};

impl UserPermissions {
    /// # Fungsi is_admin
    ///
    /// Fungsi ini digunakan untuk cek status admin pengguna.
    ///
    /// <br />
    ///
    /// # Masukan
    ///
    /// * `request` - HttpRequest.
    /// * `db` - mongodb Database type yang dishare melalui _application state_.
    ///
    /// <br />
    ///
    /// # Keluaran
    ///
    /// * `Result<(), AppErrors>` - keluaran berupa enum `Result`
    /// yang terdiri dari () dan _enum_ `AppErrors`
    pub async fn is_admin(
        request: HttpRequest,
        db: web::Data<Database>
    ) -> Result<(), AppErrors> {
        let headers = request.headers().get("authorization");
        let error_message = AppErrors::ActixWebError(
            ErrorUnauthorized("Hak akses tidak ditemukan.")
        );

        let token = <AppHelpers as AppHelpersTrait>::get_token(headers)?;

        let secret = env::var("APP_SECRET")?;
        let payload = decode::<Klaim>(
            &token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default()
        )?;

        let email = payload.claims.get_email();
        let collection = db.collection("pengguna");
        let result = collection.find_one(
            doc! { "email": email },
            None
        ).await?;

        match result {
            Some(document) => {
                let dok = bson::from_document::<Document>(document)?;
                let peg = <PenggunaHelpers as PenggunaHelpersTrait>::doc_to_pengguna(dok)?;

                if peg.isadmin {
                    Ok(())
                } else { Err(error_message) }
            }
            None => Err(error_message)
        }
    }
}
