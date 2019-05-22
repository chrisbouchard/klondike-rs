use std::{error, fmt};

pub trait ErrorKind: fmt::Debug + fmt::Display {}

#[derive(Debug)]
pub struct Error<K>
where
    K: ErrorKind,
{
    pub kind: K,
    source: Option<Box<dyn error::Error + Send + Sync + 'static>>,
}

impl<K> fmt::Display for Error<K>
where
    K: ErrorKind,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.kind)?;

        if let Some(ref source) = self.source {
            write!(fmt, "\nCaused by: {}", source)?;
        }

        Ok(())
    }
}

impl<K> error::Error for Error<K>
where
    K: ErrorKind,
{
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        self.source.as_ref().map(|boxed| {
            let source: &(dyn error::Error + 'static) = boxed.as_ref();
            source
        })
    }
}

impl<K> From<K> for Error<K>
where
    K: ErrorKind,
{
    fn from(kind: K) -> Self {
        Error { kind, source: None }
    }
}

pub trait ErrorExt {
    fn wrap_as_source_of<K>(self, kind: K) -> Error<K>
    where
        K: ErrorKind;
}

impl<E> ErrorExt for E
where
    E: error::Error + Send + Sync + 'static,
{
    fn wrap_as_source_of<K>(self, kind: K) -> Error<K>
    where
        K: ErrorKind,
    {
        Error {
            kind,
            source: Some(Box::new(self)),
        }
    }
}

pub trait ResultExt<T> {
    fn map_err_as_source_of<K>(self, kind: K) -> Result<T, Error<K>>
    where
        K: ErrorKind;
}

impl<T, E> ResultExt<T> for Result<T, E>
where
    E: ErrorExt,
{
    fn map_err_as_source_of<K>(self, kind: K) -> Result<T, Error<K>>
    where
        K: ErrorKind,
    {
        self.map_err(|e| e.wrap_as_source_of(kind))
    }
}
