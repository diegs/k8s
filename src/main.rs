extern crate futures;
extern crate hyper;
extern crate tokio_core;

use std::io::{self, Write};
use futures::{Future, Stream};
use hyper::Client;
use tokio_core::reactor::Core;

fn http_get() -> std::result::Result<(), hyper::Error> {
    let mut core = Core::new()?;
    let client = Client::new(&core.handle());

    let uri = "http://httpbin.org/ip".parse()?;
    let work = client.get(uri).and_then(|res| {
        println!("Response: {}", res.status());

        res.body().for_each(|chunk| {
            io::stdout().write_all(&chunk).map(|_| ()).map_err(
                From::from,
            )
        })
    });
    core.run(work)
}

// 1. Read kubeconfig.
// 2. Set auth.
// 3. Construct request.
// 4. Send request.
// 5. Parse response.
fn main() {
    http_get().expect("http get");
}
