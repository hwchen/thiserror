use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
struct BracedWithSourceDuplicate {
    source: io::Error,
    // this should cause compilation error
    // becuase there's two `source`
    #[source]
    cause: io::Error,
}

#[derive(Error, Debug)]
struct BracedWithFromDuplicate1 {
    #[from]
    source: io::Error,
    // this should cause compilation error
    // because there's two `from`
    #[from]
    cause: io::Error,
}

#[derive(Error, Debug)]
struct BracedWithFromDuplicateSource {
    source: io::Error,
    // this should cause compilation error
    // because from implies source
    //
    // TODO should this error actually be that there's a from tha't not on source?
    #[from]
    cause: io::Error,
}

#[derive(Error, Debug)]
struct BracedWithFromExtraField1 {
    #[from]
    source: io::Error,
    // this should cause compilation error
    // because fields other than `from` are not allowed (except backtrace in future)
    msg: String,
}

fn main() {}
