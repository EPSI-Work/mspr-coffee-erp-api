use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub details: Option<Detail>,
    pub stock: i64,
    #[serde(with = "firestore::serialize_as_timestamp")]
    pub created_at: DateTime<Utc>,
}

// impl PartialEq for Product {
//     fn eq(&self, other: &Self) -> bool {
//         self.id == other.id
//             && self.name == other.name
//             && self.details == other.details
//             && self.stock == other.stock
//             && self.created_at == other.created_at
//     }
// }

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, PartialOrd)]
pub struct Detail {
    pub price: f64,
    pub description: String,
    pub color: String,
}

impl Eq for Detail {}

impl Ord for Detail {
    fn cmp(&self, other: &Self) -> Ordering {
        self.price
            .partial_cmp(&other.price)
            .unwrap_or(Ordering::Equal)
            .then_with(|| self.description.cmp(&other.description))
            .then_with(|| self.color.cmp(&other.color))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detail_cmp() {
        let detail1 = Detail {
            price: 5.0,
            description: String::from("Detail 1"),
            color: String::from("red"),
        };
        let detail2 = Detail {
            price: 5.0,
            description: String::from("Detail 2"),
            color: String::from("blue"),
        };
        let detail3 = Detail {
            price: 5.0,
            description: String::from("Detail 1"),
            color: String::from("red"),
        };
        let detail4 = Detail {
            price: f64::MAX,
            description: String::from("Detail 3"),
            color: String::from("green"),
        };

        assert!(detail1 != detail2, "Only the price should be the same");
        assert!(detail1 == detail3, "All values should be the same");
        assert_eq!(
            detail2.cmp(&detail4),
            Ordering::Less,
            "Detail with the greatest price should be greater"
        );
    }
}
