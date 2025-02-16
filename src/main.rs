use display::display_all_menus;
use types::RestaurantCode;

mod cli;
mod display;
mod parse_dwr;
mod request;
mod types;
mod utils;

const RESTAURANT_FISICA: RestaurantCode = 8;
const RESTAURANT_QUIMICA: RestaurantCode = 9;
const RESTAURANT_PREF: RestaurantCode = 7;
const RESTAURANT_CENTRAL: RestaurantCode = 6;

#[tokio::main]
async fn main() {
    let (menu_type, everything, weekday) = cli::cli().await;

    let restaurant_codes = [
        RESTAURANT_CENTRAL,
        RESTAURANT_QUIMICA,
        RESTAURANT_FISICA,
        RESTAURANT_PREF,
    ]
    .to_vec();

    display_all_menus(restaurant_codes, everything, weekday, menu_type).await;
}
