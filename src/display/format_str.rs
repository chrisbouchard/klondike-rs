use std::fmt;

#[derive(Clone, Debug)]
pub struct FormattedString {
    len: usize,
    string: String,
}

impl Default for FormattedString {
    fn default() -> Self {
        FormattedString {
            len: 0,
            string: String::new(),
        }
    }
}

impl From<String> for FormattedString {
    fn from(content: String) -> Self {
        FormattedString::of_content(content)
    }
}

impl<'a> From<&'a str> for FormattedString {
    fn from(content: &'a str) -> Self {
        FormattedString::of_content(content)
    }
}

impl FormattedString {
    pub fn of_content<D>(content: D) -> Self
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
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.string)
    }
}
