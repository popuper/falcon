
#[allow(dead_code)]
pub struct Body {
    content: String,
	len: usize,
	content_type: String,
}

#[allow(dead_code)]
impl Body {
    pub fn from_string(string: String) -> Self {
        Self {
            content: string.clone(),
			len: string.len(),
			content_type: parse_type(string.as_str()),
		}
    }
	pub fn from_file(path: String) -> Self {
        let content = fs::read_to_string(path).unwrap();
        Self {
            content: content.clone(),
			len: content.len(),
			content_type: parse_type(&content),
		}
    }
}

fn parse_type(string: &str) -> String {
    let mut res = String::new();
    if string.starts_with("<!DOCTYPE html>") {
        res.push_str("text/HTML");
        res
    } else {
        res.push_str("text/plain");
        res
    }
}