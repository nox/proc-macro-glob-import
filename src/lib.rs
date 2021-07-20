pub use macros::settings;

// RA fails to resolve this, as long as there is a `settings` module in
// `$ROOT/macros/src/lib.rs`.
pub use settings::Settings;
