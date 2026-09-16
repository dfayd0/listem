use axum::{
    extract::Request,
    http::{
        header::{
            COOKIE,
            SET_COOKIE,
        },
        HeaderValue,
        Method,
    },
    middleware::Next,
    response::{
        IntoResponse,
        Response,
    },
};

pub const CSRF_COOKIE: &str = "csrf_token";
pub const CSRF_HEADER: &str = "x-csrf-token";
pub const CSRF_TOKEN_BYTES: usize = 32;

/// Double submit cookie CSRF protection.
///
/// - Every response for a missing cookie sets a random token cookie.
/// - Unsafe methods (POST, PUT, PATCH, DELETE) must carry a header matching
///   the cookie value.
/// - The cookie is SameSite=Strict so browsers don't send it on cross-site
///   requests, and it is intentionally readable by JS (no HttpOnly) so the
///   client can echo it back in the header.
pub async fn csrf_protect(req: Request, next: Next) -> Response
{
    let cookie_token = cookie_value(req.headers().get(COOKIE), CSRF_COOKIE)
        .map(str::to_owned);

    if is_unsafe_method(req.method())
    {
        let header_token =
            req.headers().get(CSRF_HEADER).and_then(|v| v.to_str().ok());

        let valid = match (&cookie_token, header_token)
        {
            (Some(cookie), Some(header)) => constant_time_eq(cookie, header),
            _ => false,
        };

        if !valid
        {
            return (
                axum::http::StatusCode::FORBIDDEN,
                "invalid csrf token",
            )
                .into_response();
        }
    }

    let mut res = next.run(req).await;

    if cookie_token.is_none()
    {
        if let Ok(value) =
            HeaderValue::from_str(&format!("{}={}; Path=/; SameSite=Strict", CSRF_COOKIE, generate_token()))
        {
            res.headers_mut().append(SET_COOKIE, value);
        }
    }

    res
}

fn is_unsafe_method(method: &Method) -> bool
{
    matches!(
        *method,
        Method::POST | Method::PUT | Method::PATCH | Method::DELETE
    )
}

/// Extract a cookie value by name from a Cookie header value.
fn cookie_value<'a>(cookie_header: Option<&'a HeaderValue>, name: &str) -> Option<&'a str>
{
    let header = cookie_header?.to_str().ok()?;

    header.split(';').find_map(|pair| {
        let pair = pair.trim();
        let (key, value) = pair.split_once('=')?;
        if key.trim() == name
        {
            Some(value.trim())
        }
        else
        {
            None
        }
    })
}

/// Length independent and branchless comparison to avoid timing leaks.
fn constant_time_eq(a: &str, b: &str) -> bool
{
    let (a, b) = (a.as_bytes(), b.as_bytes());

    if a.len() != b.len()
    {
        return false;
    }

    a.iter()
        .zip(b.iter())
        .fold(0u8, |acc, (x, y)| acc | (x ^ y))
        == 0
}

/// Random token as hex, from the OS CSPRNG.
pub fn generate_token() -> String
{
    use rand::RngCore;

    let mut bytes = [0u8; CSRF_TOKEN_BYTES];
    rand::rngs::OsRng.fill_bytes(&mut bytes);

    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn generated_tokens_are_random_hex()
    {
        let a = generate_token();
        let b = generate_token();

        assert_eq!(a.len(), CSRF_TOKEN_BYTES * 2);
        assert!(a.chars().all(|c| c.is_ascii_hexdigit()));
        assert!(b.chars().all(|c| c.is_ascii_hexdigit()));
        assert_ne!(a, b);
    }

    #[test]
    fn constant_time_eq_matches_equal_strings()
    {
        assert!(constant_time_eq("abc", "abc"));
        assert!(!constant_time_eq("abc", "abd"));
        assert!(!constant_time_eq("abc", "abcd"));
        assert!(!constant_time_eq("", "a"));
    }

    #[test]
    fn cookie_value_parses_named_cookie()
    {
        let header = HeaderValue::from_static("other=1; csrf_token=deadbeef");

        assert_eq!(cookie_value(Some(&header), "csrf_token"), Some("deadbeef"));
        assert_eq!(cookie_value(Some(&header), "missing"), None);
        assert_eq!(cookie_value(None, "csrf_token"), None);
    }
}
