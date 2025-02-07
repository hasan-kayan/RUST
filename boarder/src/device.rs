#[derive(Debug)]
pub struct Device {
    pub id: String,
    pub vendor_id: u16,
    pub product_id: u16,
    pub description: Option<String>,
}
