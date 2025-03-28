use std::convert::identity;
use std::sync::LazyLock;
#[cfg(debug_assertions)]
use hotwatch::{Event, EventKind, Hotwatch};
use minify_html::Cfg;
use serde::Serialize;
#[cfg(debug_assertions)]
use std::sync::RwLock;
use tera::Tera;

pub mod core;

macro_rules! templates_dir {
    () => {
        concat!(env!("CARGO_MANIFEST_DIR"), "/templates")
    };
}

static MINIFY_CONFIG: LazyLock<Cfg> = LazyLock::new(|| {
    let mut cfg = Cfg::new();
    cfg.minify_doctype = false;
    cfg.minify_js = true;
    cfg.minify_css = true;
    cfg.keep_comments = false;

    cfg
});

#[cfg(debug_assertions)]
static TERA: LazyLock<RwLock<Tera>> = LazyLock::new(|| {
    Tera::new(concat!(templates_dir!(), "**/*.html"))
        .unwrap()
        .into()
});

#[cfg(not(debug_assertions))]
static TERA: LazyLock<Tera> = LazyLock::new(|| {
    Tera::new(concat!(templates_dir!(), "**/*.html")).unwrap()
});

#[cfg(debug_assertions)]
static HOTWATCH: LazyLock<Hotwatch> = LazyLock::new(|| {
    use std::time::Duration;
    let mut hotwatch =
        Hotwatch::new_with_custom_delay(Duration::new(1, 0)).unwrap();
    hotwatch
        .watch(templates_dir!(), |event: Event| {
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

pub type RenderResult = Result<Vec<u8>, RenderError>;

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
) -> Result<Vec<u8>, tera::Error> {
    use minify_html::minify;

    ctx.insert("env_is_dev", &true);
    let mteradev = TERA.read().unwrap();
    let raw = mteradev.render(path, &ctx)?;
    let minified = minify(raw.as_bytes(), &MINIFY_CONFIG);

    Ok(minified)
}

#[cfg(not(debug_assertions))]
fn render_internal(
    path: &'static str,
    mut ctx: tera::Context,
) -> Result<Vec<u8>, tera::Error> {
    use minify_html::minify;

    ctx.insert("env_is_dev", &false);
    let raw = TERA.render(path, &ctx)?;
    let minified = minify(raw.as_bytes(), &MINIFY_CONFIG);

    Ok(minified)
}

impl<T: Serialize + Default> AppTemplate for T {
    fn render(self, path: &'static str) -> RenderResult {
        let ctx_json =
            serde_json::to_value(self).map_err(RenderError::Serde)?;
        let ctx = tera::Context::from_value(ctx_json)
            .map_or_else(|_| tera::Context::new(), identity);

        render_internal(path, ctx).map_err(RenderError::Tera)
    }
}
