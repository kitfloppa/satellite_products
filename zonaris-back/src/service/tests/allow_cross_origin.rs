use std::str::FromStr;

use reqwest::{
    header::HeaderMap,
    redirect::{DefaultFilter, Filter},
};

use crate::service::oceancolor::AllowCrossOrigin;

fn create_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();

    let mut hv = reqwest::header::HeaderValue::from_str("Basic 123").unwrap();
    hv.set_sensitive(true);
    headers.append(reqwest::header::AUTHORIZATION, hv);

    return headers;
}

fn helper(next: &str, previous: &str, headers: &mut HeaderMap) {
    let filter = AllowCrossOrigin::<DefaultFilter>::default();

    let next = reqwest::Url::from_str(next).unwrap();
    let previous = reqwest::Url::from_str(previous).unwrap();

    filter.handle_sensitive_headers(headers, &next, &[previous]);
}

fn should_be(headers: &HeaderMap) {
    let hv = headers.get(reqwest::header::AUTHORIZATION);
    assert!(hv.is_some());

    let hv2 = hv.unwrap();
    assert!(hv2 == "Basic 123");
    assert!(hv2.is_sensitive());
}

fn should_not_be(headers: &HeaderMap) {
    assert!(!headers.contains_key(reqwest::header::AUTHORIZATION));
}

#[test]
fn nasa_gov2fakenasa_gov() {
    let mut headers = create_headers();

    helper(
        "https://fakenasa.gov/path1",
        "https://nasa.gov/path1",
        &mut headers,
    );

    should_not_be(&headers);
}

#[test]
fn nasa_gov2nasa_com() {
    let mut headers = create_headers();

    helper(
        "https://nasa.com/path1",
        "https://nasa.gov/path1",
        &mut headers,
    );

    should_not_be(&headers);
}

#[test]
fn nasa_gov2nasa() {
    let mut headers = create_headers();

    helper("https://nasa/path1", "https://nasa.gov/path1", &mut headers);

    should_not_be(&headers);
}

#[test]
fn nasa_gov2nasa_gov() {
    let mut headers = create_headers();

    helper(
        "https://nasa.gov/path1",
        "https://nasa.gov/path1",
        &mut headers,
    );

    should_be(&headers);
}

#[test]
fn nasa_gov2sd1_nasa_gov() {
    let mut headers = create_headers();

    helper(
        "https://sd1.nasa.gov/path1",
        "https://nasa.gov/path1",
        &mut headers,
    );

    should_be(&headers);
}

#[test]
fn nasa_gov2sd1_sd2_nasa_gov() {
    let mut headers = create_headers();

    helper(
        "https://sd1.sd2.nasa.gov/path1",
        "https://nasa.gov/path1",
        &mut headers,
    );

    should_be(&headers);
}
