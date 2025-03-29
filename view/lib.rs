use std::convert::identity;
use std::sync::LazyLock;
use axum::response::Html;
#[cfg(debug_assertions)]
use hotwatch::{Event, EventKind, Hotwatch};
use serde::Serialize;
#[cfg(debug_assertions)]
use std::sync::RwLock;
use tera::Tera;

pub mod root;

#[cfg(debug_assertions)]
static TERA: LazyLock<RwLock<Tera>> = LazyLock::new(|| {
    Tera::new(concat!("view/templates", "**/*.html"))
        .unwrap()
        .into()
});

#[cfg(not(debug_assertions))]
static TERA: LazyLock<Tera> = LazyLock::new(|| {
    Tera::new(concat!("dist/templates", "**/*.html")).unwrap()
});

#[cfg(debug_assertions)]
static HOTWATCH: LazyLock<Hotwatch> = LazyLock::new(|| {
    use std::time::Duration;
    let mut hotwatch =
        Hotwatch::new_with_custom_delay(Duration::new(0, 300000000)).unwrap();
    hotwatch
        .watch("view/templates", |event: Event| {
            match event.kind {
                EventKind::Any | EventKind::Other => (),
                _ => drop(TERA.write().unwrap().full_reload()),
            };
        })
        .unwrap();

    hotwatch
});

#[cfg(debug_assertions)]
pub fn setup_hotwatch() {
    let _ = &*HOTWATCH;
}

#[derive(Debug, thiserror::Error)]
pub enum RenderError {
    #[error("Tera: {0}")]
    Tera(tera::Error),
    #[error("Serde: {0}")]
    Serde(serde_json::Error),
}

pub type RenderResult = Result<Html<String>, RenderError>;

pub trait AppTemplate: Serialize + Default {
    /// Renders the template with given path/name
    ///
    /// # Errors
    /// This method errors if Tera fails to render the template,
    /// or if the serialization with serde fails, or if
    /// (dev-only) tera full reload fails
    fn render(self, path: &'static str) -> RenderResult;
}

#[cfg(debug_assertions)]
fn render_internal(
    path: &'static str,
    mut ctx: tera::Context,
) -> Result<String, tera::Error> {
    ctx.insert("env_is_dev", &true);
    let mteradev = TERA.read().unwrap();
    let raw = mteradev.render(path, &ctx)?;

    Ok(raw)
}

#[cfg(not(debug_assertions))]
fn render_internal(
    path: &'static str,
    mut ctx: tera::Context,
) -> Result<String, tera::Error> {
    ctx.insert("env_is_dev", &false);
    let raw = TERA.render(path, &ctx)?;

    Ok(raw)
}

impl<T: Serialize + Default> AppTemplate for T {
    fn render(self, path: &'static str) -> RenderResult {
        let ctx_json =
            serde_json::to_value(self).map_err(RenderError::Serde)?;
        let ctx = tera::Context::from_value(ctx_json)
            .map_or_else(|_| tera::Context::new(), identity);

        render_internal(path, ctx).map_err(RenderError::Tera)
            .map(Html)
    }
}
