use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PaginationMetadata {
    pub total_results: u64,
    pub page_size: u64,
    pub current_page: u64,
}

#[derive(Serialize, Deserialize)]
pub struct PaginationResponse<T> {
    pub metadata: PaginationMetadata,
    pub results: Vec<T>,
    pub links: PaginationLinks,
}

#[derive(Serialize, Deserialize)]
pub struct PaginationLinks {
    pub previous: Option<String>,
    pub next: Option<String>,
}

pub fn generate_pagination_response<T>(
    results: Vec<T>,
    total_results: u64,
    current_page: u64,
    page_size: u64,
    base_url: &str,
) -> PaginationResponse<T> {
    let mut links = PaginationLinks {
        previous: None,
        next: None,
    };

    // Calculate total number of pages
    let total_pages = (total_results as f64 / page_size as f64).ceil() as u64;

    // Check if there's a previous page
    if current_page > 1 {
        links.previous = Some(format!(
            "{}?page={}&size={}",
            base_url,
            current_page - 1,
            page_size
        ));
    }

    // Check if there's a next page
    if current_page < total_pages {
        links.next = Some(format!(
            "{}?page={}&size={}",
            base_url,
            current_page + 1,
            page_size
        ));
    }

    PaginationResponse {
        metadata: PaginationMetadata {
            total_results,
            page_size,
            current_page,
        },
        results,
        links,
    }
}
