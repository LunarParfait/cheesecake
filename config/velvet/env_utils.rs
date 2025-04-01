use std::ops::Deref;
use std::str::FromStr;
use std::sync::OnceLock;

use anyhow::bail;

/// Useful when you want to handle the Result yourself, and do not want the
/// result to be leaked.
///
/// The leaking version of this is `var_try`.
///
/// # Errors
/// When the environment variable is not found or when the parsing fails for R.
pub fn owned_var_try<T: FromStr>(name: &'static str) -> Result<T, anyhow::Error>
where
    anyhow::Error: From<<T as FromStr>::Err>,
{
    let var = dotenvy::var(name)?;
    if var.is_empty() {
        bail!("Empty environment variable {name}!");
    }
    Ok(var.parse::<T>()?)
}

/// Useful when your program requires a variable to be defined and cannot provide a
/// default alternative, but you do not want the parsed result to be leaked/static ref.
/// E.g.: Any Copy type. Not worth leaking.
///
/// The leaking version of this is `var`.
///
/// # Panics
/// When the environment variable is not found or when the parsing fails for T.
#[must_use]
pub fn owned_var<T: FromStr>(name: &'static str) -> T
where
    anyhow::Error: From<<T as FromStr>::Err>,
{
    owned_var_try(name).unwrap_or_else(|_| {
        panic!("Couldn't find or parse env variable {name} for given type")
    })
}

/// Useful when you want to provide a default value for the environment variable,
/// but you do not want the parsed result to be leaked or static.
/// E.g.: Any Copy type. Not worth leaking.
///
/// The leaking version of this function is `var_or`.
pub fn owned_var_or<T: FromStr>(name: &'static str, default: T) -> T
where
    anyhow::Error: From<<T as FromStr>::Err>,
{
    owned_var_try(name).unwrap_or(default)
}

/// Useful when you want to provide a default value for the environment variable,
/// but you do not want the parsed result to be leaked or static. Use this over
/// `owned_var_or` when you need to provide a closure for the default value.
///
/// The leaking version of this function is `var_or_else`.
pub fn owned_var_or_else<T: FromStr, V: FnOnce() -> T>(
    name: &'static str,
    default: V,
) -> T
where
    anyhow::Error: From<<T as FromStr>::Err>,
{
    owned_var_try(name).unwrap_or_else(|_| default())
}

/// Utility to attempt leaking a Box to your desired static reference type.
fn try_leak<ToLeak, R: ?Sized>(
    to_leak: ToLeak,
) -> Result<&'static R, <Box<R> as TryFrom<ToLeak>>::Error>
where
    Box<R>: TryFrom<ToLeak>,
{
    let leaked: &'static R = Box::<R>::try_from(to_leak).map(Box::leak)?;
    Ok(leaked)
}

/// Useful when you want to handle the Result yourself.
///
/// # Leaks
/// This function will leak the parsed value, if any.
///
/// # Errors
/// This function will error if it fails to parse the value, or the environment variable
/// is not found
pub fn var_try<Parsed: FromStr, R: ?Sized>(
    name: &'static str,
) -> Result<&'static R, anyhow::Error>
where
    Box<R>: TryFrom<Parsed>,
    anyhow::Error: From<<Box<R> as TryFrom<Parsed>>::Error>
        + From<<Parsed as FromStr>::Err>,
{
    Ok(try_leak(owned_var_try::<Parsed>(name)?)?)
}

/// Useful when your program requires a variable to be defined and cannot
/// provide a default alternative.
///
/// # Leaks
/// This function will leak the parsed value.
///
/// # Panics
/// When the environment variable is not found or when the parsing fails for R.
#[must_use]
pub fn var<Parsed: FromStr, R: ?Sized>(name: &'static str) -> &'static R
where
    Box<R>: TryFrom<Parsed>,
    anyhow::Error: From<<Box<R> as TryFrom<Parsed>>::Error>
        + From<<Parsed as FromStr>::Err>,
{
    var_try(name).unwrap_or_else(|e| {
        panic!("Couldn't find or parse env variable {name} for given type: {e}")
    })
}

/// Useful when you want to provide a default value for the environment variable,
/// and you have a static reference to your default value.
/// E.g.: A string literal that is stored in the binary.
///
/// # Leaks
/// This function will leak the parsed value.
pub fn var_or<Parsed: FromStr, R: ?Sized>(
    name: &'static str,
    default: &'static R,
) -> &'static R
where
    Box<R>: TryFrom<Parsed>,
    anyhow::Error: From<<Box<R> as TryFrom<Parsed>>::Error>
        + From<<Parsed as FromStr>::Err>,
{
    var_try(name).unwrap_or(default)
}

/// Useful when you want to provide a default value for the environment variable,
/// but you don't have a static reference to the value.
/// E.g.: An owned `PathBuf` -> A `&'static Path`.
///
/// # Leaks
/// This function will leak the parsed or the default value.
pub fn var_or_else<
    Parsed: Into<Box<R>> + FromStr,
    R: ?Sized,
    V: FnOnce() -> Parsed,
>(
    name: &'static str,
    default: V,
) -> &'static R
where
    Box<R>: TryFrom<Parsed>,
    anyhow::Error: From<<Box<R> as TryFrom<Parsed>>::Error>
        + From<<Parsed as FromStr>::Err>,
{
    var_or(name, Box::leak(default().into()))
}

pub trait AppEnvironmentTrait {
    fn new() -> Self;
}

pub struct EnvLock<T: AppEnvironmentTrait>(OnceLock<T>);

impl<T: AppEnvironmentTrait> EnvLock<T> {
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        Self(OnceLock::new())
    }

    pub fn init(&self) {
        self.0
            .set(T::new())
            .unwrap_or_else(|_| panic!("Failed to initialize environment"));
    }
}

impl<T: AppEnvironmentTrait> Deref for EnvLock<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0
            .get()
            .unwrap_or_else(|| panic!("Environment not initialized"))
    }
}

impl<T: AppEnvironmentTrait> AsRef<T> for EnvLock<T> {
    fn as_ref(&self) -> &T {
        self
    }
}
