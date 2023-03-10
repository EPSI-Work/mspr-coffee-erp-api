use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PaginationMetadata {
    pub total_results: u32,
    pub page_size: u32,
    pub current_page: u32,
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
    total_results: u32,
    current_page: u32,
    page_size: u32,
    base_url: &str,
) -> PaginationResponse<T> {
    let mut links = PaginationLinks {
        previous: None,
        next: None,
    };

    // Calculate total number of pages
    let total_pages = (total_results as f64 / page_size as f64).ceil() as u32;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagination_response() {
        let results = vec![1, 2, 3];
        let total_results = 3;
        let current_page = 1;
        let page_size = 10;
        let base_url = "https://example.com";

        let pagination_response = generate_pagination_response(
            results.clone(),
            total_results,
            current_page,
            page_size,
            base_url,
        );

        assert_eq!(pagination_response.metadata.total_results, total_results);
        assert_eq!(pagination_response.metadata.page_size, page_size);
        assert_eq!(pagination_response.metadata.current_page, current_page);
        assert_eq!(pagination_response.results, results);
        assert_eq!(
            pagination_response.links.previous, None,
            "There should not be a previous page for the first page"
        );
        assert_eq!(
            pagination_response.links.next, None,
            "There should not be a next page for the first page"
        );
    }

    #[test]
    fn test_pagination_response_with_multiple_pages() {
        let results = vec![1, 2, 3];
        let total_results = 20;
        let current_page = 2;
        let page_size = 5;
        let base_url = "https://example.com";

        let pagination_response = generate_pagination_response(
            results.clone(),
            total_results,
            current_page,
            page_size,
            base_url,
        );

        assert_eq!(pagination_response.metadata.total_results, total_results);
        assert_eq!(pagination_response.metadata.page_size, page_size);
        assert_eq!(pagination_response.metadata.current_page, current_page);
        assert_eq!(pagination_response.results, results);
        assert_eq!(
            pagination_response.links.previous,
            Some("https://example.com?page=1&size=5".to_string()),
            "There should be a previous page for the second page"
        );
        assert_eq!(
            pagination_response.links.next,
            Some("https://example.com?page=3&size=5".to_string()),
            "There should be a next page for the second page"
        );
    }

    #[test]
    fn test_pagination_response_with_last_page() {
        let results = vec![1, 2, 3];
        let total_results = 20;
        let current_page = 4;
        let page_size = 5;
        let base_url = "https://example.com";

        let pagination_response = generate_pagination_response(
            results.clone(),
            total_results,
            current_page,
            page_size,
            base_url,
        );

        assert_eq!(pagination_response.metadata.total_results, total_results);
        assert_eq!(pagination_response.metadata.page_size, page_size);
        assert_eq!(pagination_response.metadata.current_page, current_page);
        assert_eq!(pagination_response.results, results);
        assert_eq!(
            pagination_response.links.previous,
            Some("https://example.com?page=3&size=5".to_string()),
            "There should be a previous page for the last page"
        );
        assert_eq!(
            pagination_response.links.next, None,
            "There should not be a next page for the last page"
        );
    }
}
