#[macro_export]
macro_rules! back_to_enum {
    ($(#[$meta:meta])* $vis:vis enum $name:ident {
        $($(#[$vmeta:meta])* $vname:ident $(= $val:expr)?,)*
    }) => {
        $(#[$meta])*
        $vis enum $name {
            $($(#[$vmeta])* $vname $(= $val)?,)*
        }

        impl std::convert::TryFrom<i32> for $name {
            type Error = ();

            fn try_from(v: i32) -> Result<Self, Self::Error> {
                match v {
                    $(x if x == $name::$vname as i32 => Ok($name::$vname),)*
                    _ => Err(()),
                }
            }
        }
    }
}
pub struct Response {
    status_line: StatusLine,
    response_header: ResponseHeader,
    response_body: String,
}
impl Response {
    pub fn default_as_200(response_body: String) -> Self {
        Self {
            status_line: StatusLine::generate_using_200(),
            response_header: ResponseHeader {},
            response_body,
        }
    }
    pub fn default_as_404(response_body: String) -> Self {
        Self {
            status_line: StatusLine::generate_using_404(),
            response_header: ResponseHeader {},
            response_body,
        }
    }
    pub fn format_to_ready(&self) -> String {
        format!("{}{}{}", self.status_line.format_to_ready(), self.response_header.default(), self.response_body)
    }
}

struct ResponseHeader {}


impl ResponseHeader {
    fn default(&self) -> String {
        format!("\r\n\r\n")
    }
}

pub struct StatusLine {
    version: String,
    status_code: StatusCode,
    description: String,
}

impl StatusLine {
    //default as 200
    pub fn generate_using_200() -> Self {
        Self {
            version: "HTTP/1.1".to_string(),
            status_code: StatusCode::Ok,
            description: "Yes you find it".to_string(),
        }
    }
    pub fn generate_using_400() -> Self {
        Self {
            version: "HTTP/1.1".to_string(),
            status_code: StatusCode::BadRequest,
            description: "Bad Request".to_string(),
        }
    }
    pub fn generate_using_401() -> Self {
        Self {
            version: "HTTP/1.1".to_string(),
            status_code: StatusCode::Unauthorized,
            description: "Unauthorized".to_string(),
        }
    }
    pub fn generate_using_403() -> Self {
        Self {
            version: "HTTP/1.1".to_string(),
            status_code: StatusCode::Forbidden,
            description: "Forbidden".to_string(),
        }
    }
    pub fn generate_using_500() -> Self {
        Self {
            version: "HTTP/1.1".to_string(),
            status_code: StatusCode::InternalError,
            description: "Internal Server Error".to_string(),
        }
    }
    pub fn generate_using_503() -> Self {
        Self {
            version: "HTTP/1.1".to_string(),
            status_code: StatusCode::ServerUnavailable,
            description: "Server Unavailable".to_string(),
        }
    }
    pub fn generate_using_404() -> Self {
        Self {
            version: "HTTP/1.1".to_string(),
            status_code: StatusCode::NotFound,
            description: "Not Found".to_string(),
        }
    }
    pub fn format_to_ready(&self) -> String {
        format!("{} {} {}\r\n", self.version, self.status_code as usize, self.description)
    }
}
back_to_enum! {
    pub enum StatusCode {
    Ok = 200,
    BadRequest = 400,
    Unauthorized = 401,
    Forbidden = 403,
    NotFound = 404,
    InternalError = 500,
    ServerUnavailable =503,
    }
}

#[cfg(test)]
mod tests {
    use crate::response::StatusLine;

    #[test]
    pub fn status_line_testing() {
        let original = StatusLine::generate_using_200().format_to_ready();
        let expect = String::from("HTTP/1.1 200 OK\r\n");
        assert_eq!(original, expect);
    }
}