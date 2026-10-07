# Meteroid Rust SDK

The official Rust SDK for [Meteroid](https://meteroid.com), the open-source billing and pricing platform. Meteroid manages subscriptions, usage-based billing and metering, invoicing and revenue analytics; this library calls its REST API and verifies its webhooks, against Meteroid Cloud (`https://api.meteroid.com`) or a self-hosted instance.

[Website](https://meteroid.com) · [Documentation](https://docs.meteroid.com) · [API reference](https://docs.meteroid.com/api-reference) · [Meteroid on GitHub](https://github.com/meteroid-oss/meteroid)

```sh
cargo add meteroid-rs
```

Calls are futures: run them on Tokio. Every method of the API is listed in [api.md](api.md).

## Usage

```rust
use meteroid_rs::api::Meteroid;

let client = Meteroid::builder()
    .token("your-api-key")
    .build()?;

let add_on = client.add_ons().retrieve("addon_id").await?;
println!("{add_on:?}");
```

`Meteroid::from_env()?` takes the token from `METEROID_API_KEY` and the base URL from
`METEROID_BASE_URL` when set; `builder()` reads no environment variable, and
`MeteroidBuilder::from_env()` starts from them. The builder also sets the timeout, retries
and default headers:

```rust
let client = Meteroid::builder()
    .token("your-api-key")
    .base_url("https://meteroid.your-company.com")
    .timeout(std::time::Duration::from_secs(20))
    .max_retries(3)
    .header("x-team", "billing")
    .build()?;
```

The base URL defaults to `https://api.meteroid.com`; building a client fails with
`Error::Request` when it is invalid.

Every API area hangs off the client (`client.add_ons()`), and `with_options` sets headers, the
timeout, retries or the idempotency key of the calls made through it. Clients are cheap to clone
and can move to other tasks.

Operations take their path parameters, their body, then their query and header parameters as an
options struct, built with the required ones by `new(...)`; when every parameter is optional, pass
the struct or `None`. Models keep the properties this version of the SDK does not know in `extra`,
and send them back. Request models are built from their required fields by `new(...)`:

```rust
use meteroid_rs::models::CreateOnboardingLinkRequest;

let onboarding_link_response = client.connect().create_onboarding_link("id", CreateOnboardingLinkRequest::new("redirect_url")).await?;
```

Models only found in responses are `#[non_exhaustive]`: build them, in tests say, with
`new(required...)` and field assignments. Operations that may answer without a body return an
`Option`.

## Raw responses

Awaiting a call gives its decoded body; `with_response()` also gives the status and headers:

```rust
let response = client.add_ons().retrieve("addon_id").with_response().await?;
println!("{} {:?}", response.status(), response.request_id());
let add_on = response.into_data();
```

## Errors

Every call fails with `meteroid_rs::error::Error`: `Api` for a non-2xx response (after
retries), `Timeout`, `Connection`, `Decode` for an unexpected body, `Request` for a request that
could not be built.

```rust
use meteroid_rs::error::{ApiErrorKind, Error};

match client.add_ons().retrieve("addon_id").await {
    Err(Error::Api(error)) if error.kind() == ApiErrorKind::NotFound => {}
    Err(error) => eprintln!("{error} (request {:?})", error.api().and_then(|e| e.request_id())),
    Ok(add_on) => println!("{add_on:?}"),
}
```

`ApiError::payload()` decodes the body as the API's common error schema, and `json::<T>()` as any
other.

## Retries and timeouts

Connection errors, timeouts, 408, 429 and 5xx responses are retried twice with jittered
backoff, honoring `Retry-After`, when the request is idempotent: GET, PUT, DELETE, or any request
with an `Idempotency-Key`. Each attempt times out after 60
seconds by default.

```rust
use meteroid_rs::api::RequestOptions;

let options = RequestOptions::new().max_retries(0).timeout(std::time::Duration::from_secs(5));
client.add_ons().with_options(options).retrieve("addon_id").await?;
```

## Pagination

A paginated list returns a `PageCall`. Awaited, it gives the first `Page`, which dereferences to
the response body and lists this page's items with `items()`; `next_page()` fetches the next one.
`items()` on the call streams every item across pages, fetching each page when needed, and
`pages()` every page; both are `futures_core::Stream`s too:

```rust
let page = client.add_ons().list(None).await?;
println!("{:?}, {} items", page.pagination_meta, page.items().len());
if let Some(next) = page.next_page().await? {
    println!("{} more", next.items().len());
}

let mut items = client.add_ons().list(None).items();
while let Some(add_on) = items.next().await {
    println!("{:?}", add_on?);
}

let mut pages = client.add_ons().list(None).pages();
while let Some(page) = pages.next().await {
    println!("{} items", page?.items().len());
}
```

`into_inner()` gives a page's body, and `with_response()` the status and headers of the first page.

## Features

`rustls-tls` (default) or `native-tls`, `http2`, and `webhooks` (default) for the webhook verifier.

- Source: https://github.com/meteroid-oss/meteroid-rust
- License: Apache-2.0

_Generated by [perseid](https://github.com/meteroid-oss/perseid)._
