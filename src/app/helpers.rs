//! # Module Helpers
//!
//! Module ini digunakan untuk membantu keperluan _module_ masing-masing `services`
//!
//! <br />
//!
//! # Contoh
//!
//! ```rust
//! use crate::app::helpers::{...}
//! ```
use chrono::{DateTime, Utc};
use mongodb::bson::Bson;

/// Trait digunakan untuk menerapkan fungsi yang diperlukan oleh masing-masing `Services`.
pub trait AppHelpersTrait {
    fn last_modified(docu: Option<&Bson>) -> Option<DateTime<Utc>>;
    fn optional_string(docu: Option<&Bson>) -> Option<String>;
}

/// Struct untuk memberikan fungsi-fungsi bantuan melalui implementasi
pub struct AppHelpers;

impl AppHelpersTrait for AppHelpers {
    /// # Fungsi last_modified
    ///
    /// Fungsi ini untuk mendapatkan waktu saat dokumen berhasil diubah.
    ///
    /// <br />
    ///
    /// # Masukan
    ///
    /// * `docu` - masukan dengan _type_ `Document`.
    ///
    /// <br />
    ///
    /// # Keluaran
    ///
    /// * `Option<DateTime<Utc>>` - keluaran berupa _enum_ `Option` yang terdiri dari
    /// `DateTime<Utc>`.
    fn last_modified(docu: Option<&Bson>) -> Option<DateTime<Utc>> {
        let diubah: Option<DateTime<Utc>>;

        if let Some(data) = docu {
            if let Some(kapan) = data.as_datetime() {
                diubah = Some(*kapan)
            } else { diubah = None }
        } else { diubah = None }

        diubah
    }

    /// # Fungsi optional_string
    ///
    /// Fungsi ini untuk mendapatkan optional text saat dokumen berhasil diubah.
    ///
    /// <br />
    ///
    /// # Masukan
    ///
    /// * `docu` - masukan dengan _type_ `Document`.
    ///
    /// <br />
    ///
    /// # Keluaran
    ///
    /// * `Option<String>` - keluaran berupa _enum_ `Option` yang terdiri dari
    /// `String`.
    fn optional_string(docu: Option<&Bson>) -> Option<String> {
        let text: Option<String>;

        if let Some(data) = docu {
            if let Some(kapan) = data.as_str() {
                text = Some(kapan.to_string())
            } else { text = None }
        } else { text = None }

        text
    }
}
