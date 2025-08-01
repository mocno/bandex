/*!
Algumas objetos para funcionamento do aplicativo
*/
use crate::parse_dwr;
use chrono::Weekday;
use std::collections::HashMap;

/// ID do restaurate
pub type RestaurantID = usize;

/// Tipos de refeição
#[derive(Debug, PartialEq, Clone)]
pub enum MenuType {
    /// Almoço
    Lunch,
    /// Janta
    Dinner,
}

impl ToString for MenuType {
    fn to_string(&self) -> String {
        match self {
            MenuType::Dinner => "Jantar",
            MenuType::Lunch => "Almoço",
        }
        .to_string()
    }
}

/// Cardápio do dia
#[derive(Debug, Clone)]
pub struct Menu {
    /// Conteúdo da refeição
    pub content: String,

    /// Tipo da refeição, por exemplo: janta ou almoço
    pub menu_type: MenuType,

    /// Dia da semana da refeição (Domingo: 1, Segunda: 2, ..., Sabado: 7)
    pub weekday: Weekday,

    /// Valor Calórico da refeição
    pub calorific_value: Option<usize>,

    /// Observações do refeição
    pub observation: String,
}

/// Cache para as refeições e os nomes dos restaurantes
#[derive(Debug)]
pub struct MenusCache {
    menus: HashMap<RestaurantID, Vec<Menu>>,
    names: HashMap<RestaurantID, String>,
}

impl MenusCache {
    pub fn new() -> Self {
        MenusCache {
            names: HashMap::new(),
            menus: HashMap::new(),
        }
    }

    pub async fn search(&mut self, restaurant_id: RestaurantID) {
        if let Some(menus) = parse_dwr::get_menus(restaurant_id).await {
            if let Some(name) = parse_dwr::get_restaurant_name(restaurant_id).await {
                self.menus.insert(restaurant_id, menus);
                self.names.insert(restaurant_id, name);
            };
        };
    }

    pub async fn get_name_and_menu(
        &mut self,
        restaurant_id: RestaurantID,
        menu_type: &MenuType,
        weekday: Weekday,
    ) -> Option<(String, Menu)> {
        if !self.menus.contains_key(&restaurant_id) {
            self.search(restaurant_id).await;
        }

        let menus = self.menus.get(&restaurant_id)?;

        for menu in menus {
            if menu.weekday == weekday && &menu.menu_type == menu_type {
                let name = self.names.get(&restaurant_id)?;
                return Some((name.clone(), menu.clone()));
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    /// ID do restaurante Central
    const RESTAURANT_CENTRAL: RestaurantID = 6;

    use super::*;

    #[test]
    fn test_menu_type_to_string() {
        assert_eq!(MenuType::Lunch.to_string(), "Almoço");
        assert_eq!(MenuType::Dinner.to_string(), "Jantar");
    }

    #[tokio::test]
    async fn test_get_name_and_menu_from_menu_cache() {
        let mut cache = MenusCache::new();
        let value = cache
            .get_name_and_menu(RESTAURANT_CENTRAL, &MenuType::Lunch, Weekday::Mon)
            .await;

        assert!(value.is_some());
        let (name, menu) = value.unwrap();
        assert_eq!(name, "Restaurante Central");
        assert!(menu.weekday == Weekday::Mon);
        assert!(menu.menu_type == MenuType::Lunch);
    }
}
