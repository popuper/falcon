use chrono::Utc;

#[allow(dead_code)]
pub struct Response {
    status_line: StatusLine,
    response_header: ResponseHeader,
    response_body: String,
}

pub struct ResponseBody {
    content_type: String,
    content: String,
    len: usize,
}

impl ResponseBody {
    pub fn building(content_type: String, content: String, len: usize) -> Self {
        Self {
            content_type,
            content,
            len,
        }
    }
}

#[allow(dead_code)]
impl Response {
    pub fn custom_header(mut self, response_header: ResponseHeader) {
        self.response_header = response_header;
    }
    pub fn default_as_200(response_body: ResponseBody) -> Self {
        Self {
            status_line: StatusLine::generate_using_200(),
            response_header: ResponseHeader::building(response_body.len, response_body.content_type),
            response_body: response_body.content,
        }
    }
    pub fn default_as_404(response_body: ResponseBody) -> Self {
        Self {
            status_line: StatusLine::generate_using_404(),
            response_header: ResponseHeader::building(response_body.len, response_body.content_type),
            response_body: response_body.content,
        }
    }
    pub fn format_to_ready(&self) -> String {
        format!("{}{}{}", self.status_line.format_to_ready(), self.response_header.format_to_ready(), self.response_body)
    }
}


pub struct ResponseHeader {
    // what's  method you allow
    allow: String,
    // control the source how to store
    cache_control: String,
    // what's transfer protocol you allow
    content_encoding: String,
    // the content's nature language you are using
    content_language: String,
    //the content's len
    content_length: usize,
    //what's type about you response body
    content_type: String,
    expires: String,
    server: String,
    set_cookie: Cookies,
    vary: String,
    date: String,
}

impl ResponseHeader {
    pub fn building(content_length: usize, content_type: String) -> Self {
        Self {
            allow: "*".to_string(),
            cache_control: "no-store".to_string(),
            content_encoding: "".to_string(),
            content_language: "zh-cn".to_string(),
            content_length,
            content_type,
            expires: Utc::now().to_string().replace("UTC", "GMT"),
            server: "falcon/0.1".to_string(),
            set_cookie: Cookies { store: Vec::new() },
            vary: "".to_string(),
            date: Utc::now().to_string().replace("UTC", "GMT"),
        }
    }

    ///
    ///
    /// this is a example for HTTP Response headers:
    /// field:value CRLF(\r\n) field:value CRLF
    /// CRLF
    /// {body}
    fn format_to_ready(&self) -> String {
        format!("Allow:{}\r\n\
        Cache-Control:{}\r\n\
        Content-Encoding:{}\r\n\
        Content-Length:{}\r\n\
        Content-Type:{}\r\n\
        Expires:{}\r\n\
        Server:{}\r\n\
        Vary:{}\r\n\
        Date:{}\r\n\
        \r\n",
                self.allow,
                self.cache_control,
                self.content_encoding,
                self.content_length,
                self.content_type,
                self.expires,
                self.server,
                self.vary,
                self.date,
        )
    }
}

/*
i don't know how to deal with the cookie
*/
struct Cookie(String, String);
pub struct Cookies {
    store: Vec<Cookie>,
}

impl Cookies {
    pub fn default() -> Self {
        Self {
            store: Vec::new()
        }
    }
    pub fn put_cookie(&mut self, k: String, v: String) {
        self.store.push(Cookie(k, v))
    }

    pub fn format_to_ready(&self) {
        let mut str = String::new();
        for cookie in self.store.iter() {
            str.push_str(cookie.0.as_str().clone());
            str.push_str("=");
            str.push_str(cookie.1.as_str().clone());
        }
    }
}


/* -------------------------it's OK-----------------------*/
struct StatusLine {
    version: String,
    status_code: StatusCode,
    description: String,
}

#[allow(dead_code)]
impl StatusLine {
    //format_to_ready as 200
    fn generate_using_200() -> Self {
        Self {
            version: "HTTP/1.1".to_string(),
            status_code: StatusCode::Ok,
            description: "OK".to_string(),
        }
    }
    fn generate_using_400() -> Self {
        Self {
            version: "HTTP/1.1".to_string(),
            status_code: StatusCode::BadRequest,
            description: "Bad Request".to_string(),
        }
    }
    fn generate_using_401() -> Self {
        Self {
            version: "HTTP/1.1".to_string(),
            status_code: StatusCode::Unauthorized,
            description: "Unauthorized".to_string(),
        }
    }
    fn generate_using_403() -> Self {
        Self {
            version: "HTTP/1.1".to_string(),
            status_code: StatusCode::Forbidden,
            description: "Forbidden".to_string(),
        }
    }
    fn generate_using_500() -> Self {
        Self {
            version: "HTTP/1.1".to_string(),
            status_code: StatusCode::InternalError,
            description: "Internal Server Error".to_string(),
        }
    }
    fn generate_using_503() -> Self {
        Self {
            version: "HTTP/1.1".to_string(),
            status_code: StatusCode::ServerUnavailable,
            description: "Server Unavailable".to_string(),
        }
    }
    fn generate_using_404() -> Self {
        Self {
            version: "HTTP/1.1".to_string(),
            status_code: StatusCode::NotFound,
            description: "Not Found".to_string(),
        }
    }
    fn format_to_ready(&self) -> String {
        format!("{} {} {}\r\n", self.version, self.status_code as usize, self.description)
    }
}

enum StatusCode {
    Ok = 200,
    BadRequest = 400,
    Unauthorized = 401,
    Forbidden = 403,
    NotFound = 404,
    InternalError = 500,
    ServerUnavailable = 503,
}

#[cfg(test)]
mod tests {
    use chrono::{Utc};
    use crate::response::StatusLine;

    #[test]
    fn test_for_status_code() {
        let original = StatusLine::generate_using_200().format_to_ready();
        let expect = String::from("HTTP/1.1 200 OK\r\n");
        println!("{original}");
        assert_eq!(original, expect)
    }

    #[test]
    fn test_for_date() {
        let time = Utc::now();

        let string = time.to_string();

        let string = string.replace("UTC", "GMT");
        println!("{string}")
    }
}

