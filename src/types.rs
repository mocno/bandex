use std::collections::HashMap;

use chrono::Weekday;

use crate::parse_dwr;

/// ID do restaurate
pub type RestaurantCode = usize;

/// Tipos de refeição
#[derive(Debug, PartialEq, Clone)]
pub enum MenuType {
    Dinner,
    Lunch,
}

impl ToString for MenuType {
    fn to_string(&self) -> String {
        match self {
            MenuType::Dinner => "Janta".to_string(),
            MenuType::Lunch => "Almoço".to_string(),
        }
    }
}

/// Cardápio do dia
#[derive(Debug, Clone)]
pub struct Menu {
    /// Conteúdo da refeição (cdpdia)
    pub content: String,

    /// Tipo da refeição, por exemplo: janta ou almoço (tiprfi)
    pub menu_type: MenuType,

    /// Dia da semana da refeição (Domingo: 1, Segunda: 2, ..., Sabado: 7) (diasemana)
    pub weekday: Weekday,

    /// Valor Calórico da refeição (vlrclorfi)
    pub calorific_value: Option<usize>,

    /// Observações do refeição (obscdpsmn)
    pub observation: String,
}

#[derive(Debug)]
pub struct MenusCache {
    menus: HashMap<RestaurantCode, Vec<Menu>>,
    names: HashMap<RestaurantCode, String>,
}

impl MenusCache {
    pub fn new() -> Self {
        MenusCache {
            names: HashMap::new(),
            menus: HashMap::new(),
        }
    }

    pub async fn search(&mut self, restaurant_code: RestaurantCode) {
        if let Some(menus) = parse_dwr::get_menus(restaurant_code).await {
            if let Some(name) = parse_dwr::get_restaurant_name(restaurant_code).await {
                self.menus.insert(restaurant_code, menus);
                self.names.insert(restaurant_code, name);
            };
        };
    }

    pub async fn get_name_and_menu(
        &mut self,
        restaurant_code: RestaurantCode,
        menu_type: &MenuType,
        weekday: Weekday,
    ) -> Option<(String, Menu)> {
        if !self.menus.contains_key(&restaurant_code) {
            self.search(restaurant_code).await;
        }

        let menus = self.menus.get(&restaurant_code)?;

        for menu in menus {
            if menu.weekday == weekday && &menu.menu_type == menu_type {
                let name = self.names.get(&restaurant_code)?;
                return Some((name.clone(), menu.clone()));
            }
        }

        None
    }
}
