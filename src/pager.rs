//it's will find all the page under the pages directory


use std::path::Path;

pub struct PageFinder {
    pages: Vec<Page>,
}

pub struct Page {
    page_name: String,
    page_content: String,
}


impl PageFinder {
    pub fn initial_and_loading(&self) -> Self {
        let path = Path::new("./pages");
        let mut pages = vec![];
        Self {
            pages
        }
    }
}


