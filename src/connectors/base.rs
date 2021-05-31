use custom_error::custom_error;

pub const MIN_AFFINITY: f64 = 0.01;

pub struct TagResult {
    pub tag: String,
    pub affinity: f64
}

pub struct PictureResult {
    pub picture_url: String,
    pub source_url: String
}

impl PictureResult {
    pub fn new(picture_url: String, source_url: String) -> PictureResult {
        PictureResult {
            picture_url,
            source_url
        }
    }
}

custom_error!{pub ConnectorError
    ReqwestError{source: reqwest::Error}    = "Http Error",
    UrlParseError{source: url::ParseError}  = "URL Parse Error",
    XmlParseError{source: serde_xml_rs::Error} = "XML Parse Error",
    SfwNotSupported                         = "Cannot filter SFW posts",
    TagNotFound                             = "Tag not Found",
    PictureNotFound                         = "No picture was found with tag",
    Unknown                                 = "Unknown Error"
}
