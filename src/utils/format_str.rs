use std::{fmt, iter::FromIterator};

#[derive(Clone, Default, Debug)]
pub struct FormattedString {
    len: usize,
    string: String,
}

impl From<String> for FormattedString {
    fn from(content: String) -> Self {
        Self::new_with_content(content)
    }
}

impl<'a> From<&'a str> for FormattedString {
    fn from(content: &'a str) -> Self {
        Self::new_with_content(content)
    }
}

impl FormattedString {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with_content<D>(content: D) -> Self
    where
        D: fmt::Display,
    {
        let content_str = format!("{}", content);
        let content_len = content_str.chars().count();

        FormattedString {
            len: content_len,
            string: content_str,
        }
    }

    pub fn new_with_formatting<D>(formatting: D) -> Self
    where
        D: fmt::Display,
    {
        FormattedString {
            len: 0,
            string: format!("{}", formatting),
        }
    }

    pub fn as_str(&self) -> &str {
        self.string.as_str()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push_content<D>(mut self, content: D) -> Self
    where
        D: fmt::Display,
    {
        let content_str = format!("{}", content);
        let content_len = content_str.chars().count();

        self.string.push_str(&content_str);
        self.len += content_len;

        self
    }

    pub fn push_formatting<D>(mut self, formatting: D) -> Self
    where
        D: fmt::Display,
    {
        self.string.push_str(&format!("{}", formatting));
        self
    }

    pub fn push_formatted_content(mut self, format_str: &FormattedString) -> Self {
        self.string.push_str(format_str.as_str());
        self.len += format_str.len();
        self
    }
}

impl fmt::Display for FormattedString {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "{}", self.string)
    }
}

impl<F> FromIterator<F> for FormattedString
where
    F: Into<FormattedString>,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = F>,
    {
        iter.into_iter().fold(Self::default(), |acc, s| {
            acc.push_formatted_content(&s.into())
        })
    }
}
