# DefinitelyHyped!

Get hyped for types!

This repository checks out https://github.com/DefinitelyTyped/DefinitelyTyped,
the canonical source for `@types` modules and puts them on
a convenient webpage.

Check to see if a module you want has types. If not, then
[get to work!](https://github.com/DefinitelyTyped/DefinitelyTyped#how-can-i-contribute)

## `definitely_hyped` rust executable
This executable queries the `DefinitelyTyped` github repo to get a list of all
`@typings` repositories to install and uploads it as a JSON array to the
provided AWS S3 bucket.

    # build exe for running on Lambda
    docker run -v $PWD/definitely_hyped:/definitely_hyped -w /definitely_hyped jimmycuadra/rust cargo build --release --target=x86_64-unknown-linux-gnu
