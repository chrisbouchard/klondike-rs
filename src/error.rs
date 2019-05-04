use std::{fmt, io, num};

error_chain! {
    foreign_links {
        Fmt(fmt::Error);
        Io(io::Error);
        TryFromInt(num::TryFromIntError);
    }
}
