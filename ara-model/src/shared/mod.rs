use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct PageRequest {
    /// Page number
    number: u32,

    /// Page Size
    size: u32,
}

#[derive(Serialize)]
pub struct Page<T: Serialize> {
    number: u32,
    size: u32,
    number_of_elements: u32,
    total_elements: u32,
    content: Vec<T>,
}

impl<T: fmt::Debug + Serialize> fmt::Debug for Page<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Page")
            .field("number", &self.number)
            .field("size", &self.size)
            .field("number_of_elements", &self.number_of_elements)
            .field("total_elements", &self.total_elements)
            .field("content", &self.content)
            .finish()
    }
}
