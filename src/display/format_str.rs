use std::fmt::{self, Write};

#[derive(Clone, Debug)]
pub struct FormattedString {
    len: usize,
    string: String,
}

impl FormattedString {
    pub fn new() -> Self {
        FormattedString {
            len: 0,
            string: String::new(),
        }
    }

    pub fn of_content<D>(content: D) -> Self {
        let content_str = format!("{}", content);
        let content_len = content_str.chars().count();

        FormattedString {
            len: content_len,
            string: content_str,
        }
    }

    pub fn as_str(&self) -> &str {
        self.string.as_str()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push_content<D>(self, content: D) -> Self {
        let content_str = format!("{}", content);
        let content_len = content_str.chars().count();

        self.string.push_str(content_str);
        self.len += content_len;

        self
    }

    pub fn push_formatting<D>(self, formatting: D) -> Self {
        self.string.push_str(format!("{}", formatting));
        self
    }
}

impl fmt::Display for FormattedString {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.string)
    }
}
