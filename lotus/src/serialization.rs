use bytes::Bytes;
use std::borrow::Cow;

pub trait Encoding {
    fn encoding(&self) -> &'static encoding_rs::Encoding;
}

impl Encoding for reqwest::Response {
    fn encoding(&self) -> &'static encoding_rs::Encoding {
        let content_type = self
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.parse::<mime::Mime>().ok());
        let encoding_name = content_type
            .as_ref()
            .and_then(|mime| mime.get_param("charset").map(|charset| charset.as_str()))
            .unwrap_or("utf-8");
        let encoding = encoding_rs::Encoding::for_label(encoding_name.as_bytes())
            .unwrap_or(encoding_rs::UTF_8);
        encoding
    }
}

pub fn deserialize_json<T: serde::de::DeserializeOwned>(bytes: &Bytes) -> serde_json::Result<T> {
    serde_json::from_slice(bytes)
}

pub fn deserialize_text(bytes: &Bytes, encoding: &'static encoding_rs::Encoding) -> String {
    let (text, _, _) = encoding.decode(bytes);
    if let Cow::Owned(s) = text {
        return s;
    }
    unsafe {
        // decoding returned Cow::Borrowed, meaning these bytes
        // are already valid utf8
        String::from_utf8_unchecked(bytes.to_vec())
    }
}
