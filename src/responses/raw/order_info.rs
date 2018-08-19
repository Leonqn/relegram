use responses::raw::shipping_address::ShippingAddress;

#[derive(Deserialize, Debug, Clone)]
pub struct OrderInfo {
    pub name: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub shipping_address: Option<ShippingAddress>
}