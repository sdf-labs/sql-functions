use arrow::array::ListArray;
use arrow::array::ListBuilder;
use arrow::array::StringArray;
use arrow::array::StringBuilder;

pub(crate) struct StringArrayExt(StringArray);

impl StringArrayExt {
    pub fn into_string_array(self) -> StringArray {
        self.0
    }
}

/// Enables collect::<ListArrayExt> from an Iterator of Option<&'a str>,
/// where None represents a null list.
impl<'a> FromIterator<Option<&'a str>> for StringArrayExt {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Option<&'a str>>,
    {
        let mut builder = StringBuilder::new();
        for item in iter {
            match item {
                Some(s) => {
                    builder.append_value(s);
                }
                None => {
                    builder.append_null();
                }
            }
        }
        StringArrayExt(builder.finish())
    }
}

/// Enables collect::<ListArrayExt> from an Iterator of Option<String>,
/// where None represents a null list.
impl<'a> FromIterator<Option<String>> for StringArrayExt {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Option<String>>,
    {
        let mut builder = StringBuilder::new();
        for item in iter {
            match item {
                Some(s) => {
                    builder.append_value(s);
                }
                None => {
                    builder.append_null();
                }
            }
        }
        StringArrayExt(builder.finish())
    }
}

pub(crate) struct ListArrayExt(ListArray);

impl ListArrayExt {
    pub fn into_list_array(self) -> ListArray {
        self.0
    }
}

/// Enables collect::<ListArrayExt> from an Iterator of Option<Vec<&'a str>>,
/// where None represents a null list.
impl<'a> FromIterator<Option<Vec<&'a str>>> for ListArrayExt {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Option<Vec<&'a str>>>,
    {
        let mut builder = ListBuilder::new(StringBuilder::new());
        for item in iter {
            match item {
                Some(vec) => {
                    for s in vec {
                        builder.values().append_value(s);
                    }
                    builder.append(true);
                }
                None => {
                    builder.append(false);
                }
            }
        }
        ListArrayExt(builder.finish())
    }
}

/// Enables collect::<ListArrayExt> from an Iterator of Option<Vec<Option<&'a str>>>,
/// where None represents a null list and a null string in a list.
impl<'a> FromIterator<Option<Vec<Option<&'a str>>>> for ListArrayExt {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Option<Vec<Option<&'a str>>>>,
    {
        let mut builder = ListBuilder::new(StringBuilder::new());
        for item in iter {
            match item {
                Some(vec) => {
                    for s in vec {
                        builder.values().append_option(s);
                    }
                    builder.append(true);
                }
                None => {
                    builder.append(false);
                }
            }
        }
        ListArrayExt(builder.finish())
    }
}
