use crate::parse_dwr;

pub async fn get_all_rest() {
    let mut current_code = 1;

    loop {
        let Some(name) = parse_dwr::get_restaurant_name(current_code).await else {
            break;
        };
        println!("{current_code}: {name}");
        current_code += 1
    }
}
