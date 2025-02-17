/*! Este arquivo contém funções para extrair dados do sistema da USP.

Os dados do sistema da USP são extraídos a partir de ~~✘uma API RESTful fornecida pela USP✘~~ uma
página ajax gerada pelo Framework DWR, esse ajax é usada, por exemplo, na página <https://uspdigital.usp.br/rucard/Jsp/cardapioSAS.jsp?codrtn=1>.
Assim, temos acesso a algumas informações do sistema da USP, divididas em duas funções:

* `obterRestauranteUsp`: Apresenta informações sobre um restaurante da USP.
* `obterCardapioRestUSP`: Obtém o cardápio de um restaurante da USP.

# Dados extraídos do CardapioControleDWR

Todas as colunas de dados então presentes nas duas funções (R e C), mas algumas só são definidas em uma das funções:

### Definidas nas duas funções:
- `codrtn`: ID do restaurante

### Definidas apenas na `obterRestauranteUsp`:
- `nomrtn`: Nome do restaurante
- `codddd1`: DDD do restaurante
- `numtel1`: Número de telefone do restaurante, pasmem, em ponto flutuante, por exemplo: 3.0913318E7 - kk

### Definidas apenas na `obterCardapioRestUSP`:
- `cdpdia`: Texto do cardápio
- `diames`: Dia da refeição
- `diasemana`: Dia da semana da refeição (Domingo: 1, Segunda: 2, ..., Sabado: 7)
- `dtainismncdp`: Data (dia, mês e ano) da refeição
- `dtarfi`: O mesmo que "dtainismncdp" (?, parece que "dtainismncdp" é data de inicio e "dtarfi", de fim)
- `obscdp`: observações do cardápio para refeição (so foi encontrado " " ou null, o "obscdpsmn" é mais util)
- `obscdpsmn`: Observações do cardápio para a semana
- `tiprfi`: Tipo da refeição ("A" de almoço e "J" de janta)
- `vlrclorfi`: Calor calórico da refeição
*/

use crate::types::{Menu, MenuType, RestaurantCode};
use chrono::Weekday;
use html_escape::decode_html_entities;
use regex::Regex;
use unescape::unescape;

trait FromDWR<T = Self> {
    fn from_dwr(value: &str) -> Option<T>;
}

impl FromDWR for MenuType {
    fn from_dwr(value: &str) -> Option<MenuType> {
        match value.trim_matches('"') {
            "A" => Some(MenuType::Lunch),
            "J" => Some(MenuType::Dinner),
            _ => None,
        }
    }
}

impl FromDWR for Weekday {
    fn from_dwr(value: &str) -> Option<Weekday> {
        // No DWR, o domingo é 1, segunda é 2, ... e sábado é 7
        // Na função Weekday::try_from, o segunda é 0, terça é 1, ... e domingo é 6
        // Dessa forma, (weekday + 5) % 7 transforma o valor do DWR em um valor válido para Weekday::try_from
        let weekday = value.parse::<u8>().ok()?;
        let weekday = (weekday + 5) % 7;
        Weekday::try_from(weekday).ok()
    }
}

impl FromDWR for Menu {
    fn from_dwr(object: &str) -> Option<Menu> {
        let content = get_value_in_dwr_object(object, KEY_MENU)?;
        let content = format_text_dwr_value(content)?;

        let menu_type = get_value_in_dwr_object(object, KEY_MENU_TYPE)?;
        let menu_type = MenuType::from_dwr(menu_type.as_str())?;

        let weekday = get_value_in_dwr_object(object, KEY_WEEKDAY_MENU)?;
        let weekday = Weekday::from_dwr(weekday.as_str())?;

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

/// Chave do objeto DWR: Nome do restaurante
const KEY_NAME_RESTAURANT: &str = "nomrtn";

/// Chave do objeto DWR: Conteúdo da refeição
const KEY_MENU: &str = "cdpdia";

/// Chave do objeto DWR: Dia da semana da refeição (Domingo: 1, Segunda: 2, ..., Sabado: 7)
const KEY_WEEKDAY_MENU: &str = "diasemana";

/// Chave do objeto DWR: Observações do refeição
const KEY_OBS_MENU: &str = "obscdpsmn";

/// Chave do objeto DWR: Tipo da refeição, por exemplo: janta ou almoço
const KEY_MENU_TYPE: &str = "tiprfi";

/// Chave do objeto DWR: Valor Calórico da refeição
const KEY_CALORIFIC_VALUE: &str = "vlrclorfi";

/// Corta apenas o objeto dwr, começando com `[{` e terminando com `}]`
fn slice_dwr_objects(body: &String) -> Option<&str> {
    let start_object = body.find("[{");
    let end_object = body.rfind("}]");

    Some(&body[start_object? + 2..end_object?])
}

/// Formata o valor do texto do objeto dwr arrumando caracteres especiais, removendo <br>, etc.
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

/// Obtém o valor de uma chave em um objeto dwr usando regex
fn get_value_in_dwr_object(object: &str, key: &str) -> Option<String> {
    let re_get_value = Regex::new(format!(".*,?{key}:(?<value>.+?)(,.*|$)").as_str()).unwrap();
    let capture = re_get_value.captures(object)?;

    Some(String::from(&capture["value"]))
}

/// Extrai o nome do restaurante usando o código do restaurante
pub async fn get_restaurant_name(code: RestaurantCode) -> Option<String> {
    let Ok(response) = crate::request::request_rest_name(code).await else {
        return None;
    };

    let object = slice_dwr_objects(&response)?;
    let name = get_value_in_dwr_object(&object, KEY_NAME_RESTAURANT)?;
    let name = format_text_dwr_value(name)?;

    Some(name)
}

/// Extrai os cardápios da semana usando o código de um restaurante
pub async fn get_menus(code: RestaurantCode) -> Option<Vec<Menu>> {
    let Ok(response) = crate::request::request_menu(code).await else {
        return None;
    };

    let objects = slice_dwr_objects(&response)?;

    let mut menus: Vec<Menu> = Vec::with_capacity(14);

    for object in objects.split("},{") {
        if let Some(menu) = Menu::from_dwr(object) {
            menus.push(menu);
        }
        // else { return Err!("Unexpected DWR object - ignored menu"); }
    }

    Some(menus)
}
