use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Rating {
    pub rate: f64,
    pub count: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Product {
	pub id: u32,
	pub title: String,
	pub price: f64,
	pub description: String,
	pub category: String,
	pub image: String,
	pub rating: Rating
}
