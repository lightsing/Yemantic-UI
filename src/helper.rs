use either::Either;

pub fn use_str<V: AsRef<str>>(val: V) -> Vec<String> {
    vec![val.as_ref().to_owned()]
}

pub fn use_key<K: AsRef<str>>(val: bool, key: K) -> Vec<String> {
    if val {
        vec![key.as_ref().to_owned()]
    } else {
        vec![]
    }
}

pub fn use_option<V: AsRef<str>>(val: &Option<V>) -> Vec<String> {
    if val.is_some() {
        vec![val.as_ref().unwrap().as_ref().to_owned()]
    } else {
        vec![]
    }
}

pub fn use_option_and_key<V: AsRef<str>, K: AsRef<str>>(val: &Option<V>, key: K) -> Vec<String> {
    if val.is_some() {
        vec![val.as_ref().unwrap().as_ref().to_owned(), key.as_ref().to_owned()]
    } else {
        vec![]
    }
}

pub fn use_key_or_option_and_key<V: AsRef<str>, K: AsRef<str>>(val: &Either<bool, V>, key: K) -> Vec<String> {
    match val {
        Either::Left(b) => {
            if *b {
                vec![key.as_ref().to_owned()]
            } else {
                vec![]
            }
        },
        Either::Right(v) => {
            vec![v.as_ref().to_owned(), key.as_ref().to_owned()]
        }
    }
}

pub fn use_text_align<V: AsRef<str>>(val: V) -> Vec<String> {
    if val.as_ref() == "justified" {
        vec!["justified".to_string()]
    } else {
        use_option_and_key(&Some(val), "aligned")
    }
}

#[macro_export]
macro_rules! cx {
    () => { vec![] };
    ($($e:expr), *) => {
        {
            let mut __gnerated_classes = vec![];
            $(
                __gnerated_classes.append(&mut $e);
            )*
            __gnerated_classes
        }
    }
}