use crate::types::{Menu, MenuType, RestaurantCode};
use chrono::Weekday;
use html_escape::decode_html_entities;
use regex::Regex;
use unescape::unescape;

/** # Dados do CardapioControleDWR

   Todas as colunas de dados então presentes nas duas funções, mas algumas não são nulos em apenas uma delas
   obterRestauranteUsp: R, obterCardapioRestUSP: C

   - codrtn:    (RC) ID do restaurante

   - nomrtn:    (R) Nome do restaurante
   - codddd1:   (R) DDD do restaurante
   - numtel1:   (R) Número de telefone do restaurante, pasmem, em ponto flutuante, por exemplo: 3.0913318E7 - kk

   - cdpdia:        (C) Texto do cardápio
   - diames:        (C) Dia da refeição
   - diasemana:     (C) Dia da semana da refeição (Domingo: 1, Segunda: 2, ..., Sabado: 7)
   - dtainismncdp:  (C) Data (dia, mês e ano) da refeição
   - dtarfi:        (C) O mesmo que "dtainismncdp" (?, parece que "dtainismncdp" é data de inicio e "dtarfi", de fim)
   - obscdp:        (C) observações do cardápio para refeição (so foi encontrado " " ou null, o "obscdpsmn" é mais util)
   - obscdpsmn:     (C) Observações do cardápio para a semana
   - tiprfi:        (C) Tipo da refeição ("A" de almoço e "J" de janta)
   - vlrclorfi:     (C) Calor calórico da refeição
*/

impl MenuType {
    fn from_drw_value(input: &str) -> Option<MenuType> {
        match input {
            "A" => Some(MenuType::Lunch),
            "J" => Some(MenuType::Dinner),
            _ => None,
        }
    }
}

impl Menu {
    fn from_drw_object(object: &str) -> Option<Menu> {
        let content = get_value_in_dwr_object(object, KEY_MENU)?;
        let content = format_text_dwr_value(content)?;

        let menu_type = get_value_in_dwr_object(object, KEY_MENU_TYPE)?;
        let menu_type = MenuType::from_drw_value(menu_type.trim_matches('"'))?;

        let weekday = get_value_in_dwr_object(object, KEY_WEEKDAY_MENU)?;
        let weekday = weekday.parse::<u8>().ok()?;
        // It starts on Sunday with 1 and ends on Saturday with 7,
        // but Weekday::try_from starts on monday...
        let weekday = Weekday::try_from((weekday + 5) % 7).ok()?;

        let calorific_value = get_value_in_dwr_object(object, KEY_CALORIFIC_VALUE)?;
        let calorific_value = calorific_value.parse::<usize>().ok()?;
        let calorific_value = (calorific_value != 0).then_some(calorific_value);

        let observation = get_value_in_dwr_object(object, KEY_OBS_MENU)?;
        let observation = format_text_dwr_value(observation)?;

        Some(Menu {
            content,
            menu_type,
            weekday,
            calorific_value,
            observation,
        })
    }
}

/// Nome do restaurante
const KEY_NAME_RESTAURANT: &str = "nomrtn";

/// Conteúdo da refeição
const KEY_MENU: &str = "cdpdia";

/// Dia da semana da refeição (Domingo: 1, Segunda: 2, ..., Sabado: 7)
const KEY_WEEKDAY_MENU: &str = "diasemana";

/// Observações do refeição
const KEY_OBS_MENU: &str = "obscdpsmn";

/// Tipo da refeição, por exemplo: janta ou almoço
const KEY_MENU_TYPE: &str = "tiprfi";

/// Valor Calórico da refeição
const KEY_CALORIFIC_VALUE: &str = "vlrclorfi";

/** Slice only dwr object, starting with "\[{" and ending with "}\]" */
fn slice_dwr_objects(body: &String) -> Option<&str> {
    let start_object = body.find("[{");
    let end_object = body.rfind("}]");

    Some(&body[start_object? + 2..end_object?])
}

fn format_text_dwr_value(value: String) -> Option<String> {
    let value = &value[1..value.len() - 1]
        .replace("<br>", "\n")
        .replace(" \\/ ", ", ")
        .trim()
        .to_string();
    let value = unescape(value)?;
    let value = decode_html_entities(&value).to_string();

    Some(value)
}

fn get_value_in_dwr_object(object: &str, key: &str) -> Option<String> {
    let re_get_value = Regex::new(format!(".*,?{key}:(?<value>.+?)(,.*|$)").as_str()).unwrap();
    let capture = re_get_value.captures(object)?;

    Some(String::from(&capture["value"]))
}

pub async fn get_restaurant_name(code: RestaurantCode) -> Option<String> {
    let Ok(response) = crate::request::request_rest_name(code).await else {
        return None;
    };

    let object = slice_dwr_objects(&response)?;
    let name = get_value_in_dwr_object(&object, KEY_NAME_RESTAURANT)?;
    let name = format_text_dwr_value(name)?;

    Some(name)
}

pub async fn get_menus(code: RestaurantCode) -> Option<Vec<Menu>> {
    let Ok(response) = crate::request::request_menu(code).await else {
        return None;
    };

    let objects = slice_dwr_objects(&response)?;

    let mut menus: Vec<Menu> = Vec::with_capacity(14);

    for object in objects.split("},{") {
        if let Some(menu) = Menu::from_drw_object(object) {
            menus.push(menu);
        }
        // else { print!("Unexpected DRW object - ignored menu"); }
    }

    Some(menus)
}
