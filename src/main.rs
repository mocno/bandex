use cli::cli;
use display::display_all_menus;
use types::RestaurantCode;

mod cli;
mod display;
mod parse_dwr;
mod request;
mod types;

const RESTAURANT_FISICA: RestaurantCode = 8;
const RESTAURANT_QUIMICA: RestaurantCode = 9;
const RESTAURANT_PREF: RestaurantCode = 7;
const RESTAURANT_CENTRAL: RestaurantCode = 6;

/// Função que orquestra a execução do programa, lendo a CLI e mostrando os menus pedidos
#[tokio::main]
async fn main() -> Result<(), &'static str> {
    let restaurant_codes = [
        RESTAURANT_CENTRAL,
        RESTAURANT_QUIMICA,
        RESTAURANT_FISICA,
        RESTAURANT_PREF,
    ]
    .to_vec();

    match cli() {
        Ok((menu_type, weekday)) => {
            display_all_menus(restaurant_codes, weekday, menu_type).await;
            Ok(())
        }
        Err(err) => Err(err),
    }
}
