/*!
Leitura do arquivo de configuração

Este módulo contém as estruturas e funções responsáveis pela leitura e
interpretação do arquivo de configuração YAML do Bandex.

O arquivo de configuração permite personalizar os restaurantes, definindo seus
IDs e cores para visualização. Se nenhum arquivo for fornecido ou se o arquivo
estiver inválido, serão utilizadas as configurações padrão.
*/

use std::{
    env, fs,
    io::{Error, ErrorKind},
    path::{Path, PathBuf},
    vec::Vec,
};

use colored::Color;
use yaml_rust::{yaml, Yaml, YamlLoader};

use crate::types::RestaurantID;

/// Padrão para configuração dos restaurantes, definido com os bandeijões da USP São Paulo.
/// * 8: Restaurante da Física
/// * 7: Restaurante das Química
/// * 9: Restaurante da Prefeitura
/// * 6: Restaurante Central
const DEFAULT_RESTAURANTS: [(RestaurantID, Color); 4] = [
    (8, Color::Blue),
    (7, Color::Blue),
    (9, Color::Blue),
    (6, Color::Blue),
];

/// Cor padrão dos restaurantes.
const DEFAULT_COLOR: Color = Color::White;

/// Variável de ambiente para o arquivo de configuração.
const ENV_VAR_BANDEX_CONFIG: &str = "BANDEX_CONFIG_FILE";

/// Macro para converter um valor em uma string YAML.
macro_rules! to_yaml_str {
    ( $value:expr ) => {{
        &Yaml::String($value.to_string())
    }};
}

/// Configurações de um restaurante como cor e ID.
#[derive(Debug, PartialEq)]
pub struct RestaurantConfig {
    pub id: RestaurantID,
    pub color: Color,
}

impl RestaurantConfig {
    pub fn new(id: RestaurantID, color: Color) -> Self {
        RestaurantConfig { id, color }
    }
}

impl TryFrom<&Yaml> for RestaurantConfig {
    type Error = Error;

    // Tenta ler as configurações dos restaurantes a partir de um YAML
    fn try_from(value: &Yaml) -> Result<Self, Self::Error> {
        let Some(restaurant_config) = value.as_hash() else {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Não foi possível ler a configuração do restaurante",
            ));
        };

        let Some(id) = restaurant_config
            .get(to_yaml_str!("id"))
            .and_then(|yaml| yaml.as_i64())
        else {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Todo restaurante deve ter um ID válido",
            ));
        };

        let color = match restaurant_config.get(to_yaml_str!("color")) {
            Some(Yaml::String(color)) => color.parse().ok(),
            Some(Yaml::Array(values)) if values.len() == 3 => {
                let r = values[0].as_i64().unwrap_or(0) as u8;
                let g = values[1].as_i64().unwrap_or(0) as u8;
                let b = values[2].as_i64().unwrap_or(0) as u8;

                Some(Color::TrueColor { r, g, b })
            }
            None => Some(DEFAULT_COLOR),
            _ => None,
        };

        let Some(color) = color else {
            return Err(Error::new(ErrorKind::InvalidData, "Cor inválida"));
        };

        Ok(RestaurantConfig::new(id as usize, color))
    }
}

/// Representa uma configuração de uma comida - relacionando uma comida com um
/// ou alguns restaurantes
#[derive(Debug)]
pub struct FoodConfig {
    pub name: String,
    pub restaurants: Option<Vec<u64>>,
}

impl FoodConfig {
    pub fn new(name: String, restaurants: Option<Vec<u64>>) -> Self {
        FoodConfig { name, restaurants }
    }
}

impl TryFrom<&Yaml> for FoodConfig {
    type Error = Error;

    // Tenta ler as configurações das comidas a partir de um YAML
    fn try_from(yaml: &Yaml) -> Result<Self, Self::Error> {
        match yaml {
            Yaml::String(value) => Ok(FoodConfig::new(value.clone(), None)),
            Yaml::Hash(value) if value.len() == 1 => {
                let (key, value) = value.iter().next().unwrap();

                let Some(key) = key.as_str() else {
                    return Err(Error::new(ErrorKind::InvalidData, "Comida inválida"));
                };
                let Some(restaurants) = value.as_vec() else {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        "Restaurante das comidas inválida",
                    ));
                };
                let mut restaurants_ids = Vec::new();
                for restaurant in restaurants {
                    let Some(restaurant_id) = restaurant.as_i64() else {
                        return Err(Error::new(
                            ErrorKind::InvalidData,
                            "Restaurante das comidas inválida",
                        ));
                    };
                    restaurants_ids.push(restaurant_id as u64);
                }

                Ok(FoodConfig::new(key.to_lowercase(), Some(restaurants_ids)))
            }
            _ => Err(Error::new(
                ErrorKind::InvalidData,
                "Comida com formato inválido",
            )),
        }
    }
}

impl FoodConfig {
    /// Verifica se uma linha contém o nome da comida.
    pub fn check_line(&self, line: &str, restaurant_id: u64) -> bool {
        let restaurant_match = if let Some(restaurants) = &self.restaurants {
            restaurants.contains(&restaurant_id)
        } else {
            true
        };

        let line_match = line.to_lowercase().contains(&self.name);

        restaurant_match && line_match
    }
}

/// Configurações do Bandex - um objeto criado a partir de um arquivo YAML.
///
/// Essa estrutura contém as configurações do Bandex, incluindo informações sobre os restaurantes.
#[derive(Debug)]
pub struct Config {
    /// Lista dos restaurantes que serão apresentados.
    pub restaurants: Vec<RestaurantConfig>,

    /// Lista das comidas favoritas.
    pub liked_foods: Vec<FoodConfig>,

    /// Lista das comidas não gostadas.
    pub disliked_foods: Vec<FoodConfig>,
}

/// Extrai objetos YAML a partir do conteúdo do arquivo YAML.
fn parse_yaml_from_content(contents: &str) -> Result<Vec<Yaml>, Error> {
    let docs = YamlLoader::load_from_str(contents)
        .map_err(|err| Error::new(ErrorKind::InvalidData, err.to_string()))?;

    Ok(docs)
}

impl Default for Config {
    /// Cria uma nova instância de Config com as configurações padrão definidas em `DEFAULT_RESTAURANTS`.
    fn default() -> Self {
        let restaurants = DEFAULT_RESTAURANTS
            .iter()
            .map(|(id, color)| RestaurantConfig {
                id: *id,
                color: *color,
            })
            .collect();

        Config {
            restaurants,
            liked_foods: vec![],
            disliked_foods: vec![],
        }
    }
}

impl Config {
    /// Extrai a configuração do bandex do documento YAML.
    fn get_bandex_yaml(doc: &Yaml) -> Option<&yaml::Hash> {
        doc.as_hash()?.get(to_yaml_str!("bandex"))?.as_hash()
    }

    /// Extrai os restaurantes, em yaml, da configuração do bandex.
    fn get_restaurants_yaml(bandex_config: &yaml::Hash) -> Option<&yaml::Array> {
        bandex_config.get(to_yaml_str!("restaurants"))?.as_vec()
    }

    /// Extrai os alimentos, em yaml, da configuração do bandex.
    fn get_foods_yaml(bandex_config: &yaml::Hash) -> Option<&yaml::Hash> {
        bandex_config.get(to_yaml_str!("foods"))?.as_hash()
    }

    /// Extrai os alimentos favoritos, em yaml, da configuração do bandex.
    fn get_liked_foods_yaml(foods_config: &yaml::Hash) -> Option<&yaml::Array> {
        foods_config.get(to_yaml_str!("liked"))?.as_vec()
    }

    /// Extrai os alimentos não gostados, em yaml, da configuração do bandex.
    fn get_disliked_foods_yaml(foods_config: &yaml::Hash) -> Option<&yaml::Array> {
        foods_config.get(to_yaml_str!("disliked"))?.as_vec()
    }

    /// Extrai as configurações a partir do conteúdo do arquivo YAML.
    pub fn from_file_content(contents: &str) -> Result<Config, Error> {
        let docs = parse_yaml_from_content(contents)?;

        if docs.is_empty() {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Não foi encontrado nada para ser lido no arquivo de configurações",
            ));
        }

        let mut restaurants = Vec::new();
        let mut liked_foods = Vec::new();
        let mut disliked_foods = Vec::new();

        for doc in docs {
            let Some(bandex_config) = Self::get_bandex_yaml(&doc) else {
                continue;
            };

            if let Some(restaurants_config) = Self::get_restaurants_yaml(bandex_config) {
                for restaurant_yaml in restaurants_config {
                    if let Ok(restaurant) = RestaurantConfig::try_from(restaurant_yaml) {
                        restaurants.push(restaurant);
                    }
                }
            }

            if let Some(foods_config) = Self::get_foods_yaml(bandex_config) {
                if let Some(liked) = Self::get_liked_foods_yaml(foods_config) {
                    for food_yaml in liked {
                        if let Ok(food) = FoodConfig::try_from(food_yaml) {
                            liked_foods.push(food);
                        }
                    }
                }
                if let Some(disliked) = Self::get_disliked_foods_yaml(foods_config) {
                    for food_yaml in disliked {
                        if let Ok(food) = FoodConfig::try_from(food_yaml) {
                            disliked_foods.push(food);
                        }
                    }
                }
            }
        }

        if restaurants.is_empty() {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Nenhum restaurante encontrado, verifique se o arquivo de configurações está correto",
            ));
        }

        Ok(Config {
            restaurants,
            liked_foods,
            disliked_foods,
        })
    }

    /// Extrai a configuração do bandex a partir de um arquivo YAML.
    pub fn from_file<P>(file_path: P) -> Result<Config, Error>
    where
        P: AsRef<Path>,
    {
        let contents = fs::read_to_string(file_path)?;

        Config::from_file_content(&contents)
    }
}

/// Lê o caminho de configuração do bandex a partir de uma variável de ambiente.
pub fn read_env_config_filepath() -> Option<PathBuf> {
    env::var(ENV_VAR_BANDEX_CONFIG)
        .ok()
        .map(|path| PathBuf::from(path))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_yaml_str() {
        let name = to_yaml_str!("name");
        let name = name.as_str();
        assert!(name.is_some() && name.unwrap() == "name");
    }

    #[test]
    fn test_new_restaurant_config() {
        let id: RestaurantID = 338;
        let color = Color::TrueColor {
            r: 187,
            g: 35,
            b: 51,
        };
        let restaurant = RestaurantConfig::new(id, color);

        assert_eq!(restaurant.id, id);
        assert_eq!(restaurant.color, color);
    }

    #[test]
    fn test_restaurant_config_parse() {
        // without color
        let values = parse_yaml_from_content("id: 2").unwrap();
        let value = values.first().unwrap();
        let restaurant = RestaurantConfig::try_from(value);

        assert!(restaurant.is_ok());

        let restaurant = restaurant.unwrap();
        assert_eq!(restaurant.id, 2);
        assert_eq!(restaurant.color, DEFAULT_COLOR);

        // name color
        let values = parse_yaml_from_content("color: BluE\nid: 6").unwrap();
        let value = values.first().unwrap();
        let restaurant = RestaurantConfig::try_from(value);

        assert!(restaurant.is_ok());

        let restaurant = restaurant.unwrap();
        assert_eq!(restaurant.id, 6);
        assert_eq!(restaurant.color, Color::Blue);

        // RGB color
        let values = parse_yaml_from_content("id: 12\ncolor: [187, 35, 51]").unwrap();
        let value = values.first().unwrap();
        let restaurant = RestaurantConfig::try_from(value);

        assert!(restaurant.is_ok());

        let restaurant = restaurant.unwrap();
        assert_eq!(restaurant.id, 12);
        assert_eq!(
            restaurant.color,
            Color::TrueColor {
                r: 187,
                g: 35,
                b: 51
            }
        );

        // Test Errors
        let values = parse_yaml_from_content("color: [187, 35, 51]").unwrap();
        let value = values.first().unwrap();
        let restaurant = RestaurantConfig::try_from(value);

        assert!(restaurant.is_err());

        let values = parse_yaml_from_content("id: \"12\"\ncolor: blue").unwrap();
        let value = values.first().unwrap();
        let restaurant = RestaurantConfig::try_from(value);

        assert!(restaurant.is_err());

        let values = parse_yaml_from_content("id: 89\ncolor: not a color").unwrap();
        let value = values.first().unwrap();
        let restaurant = RestaurantConfig::try_from(value);

        assert!(restaurant.is_err());
    }

    #[test]
    fn test_new_food_config() {
        let food_config = FoodConfig::new("pizza".to_string(), Some(vec![1, 2, 3]));
        assert_eq!(food_config.name, "pizza");
        assert_eq!(food_config.restaurants, Some(vec![1, 2, 3]));

        let food_config = FoodConfig::new("feijoada".to_string(), None);
        assert_eq!(food_config.name, "feijoada");
        assert_eq!(food_config.restaurants, None);
    }

    #[test]
    fn test_food_config_from_yaml() {
        let values = parse_yaml_from_content("pizza").unwrap();
        let value = values.first().unwrap();
        let food_config = FoodConfig::try_from(value);

        assert!(food_config.is_ok());

        let food_config = food_config.unwrap();
        assert_eq!(food_config.name, "pizza");
        assert_eq!(food_config.restaurants, None);

        let values = parse_yaml_from_content("feijoada: [1, 2, 3]").unwrap();
        let value = values.first().unwrap();
        let food_config = FoodConfig::try_from(value);

        assert!(food_config.is_ok());

        let food_config = food_config.unwrap();
        assert_eq!(food_config.name, "feijoada");
        assert_eq!(food_config.restaurants, Some(vec![1, 2, 3]));
    }

    #[test]
    fn test_check_line_config() {
        let food_config = FoodConfig::new("pizza".to_string(), None);

        assert!(food_config.check_line("pizza de feijoada com arroz", 12));
        assert!(!food_config.check_line("feijoada com arroz", 11));

        let food_config = FoodConfig::new("feijoada".to_string(), Some(vec![1, 2, 3]));

        assert!(!food_config.check_line("feijoada com arroz", 4));
        assert!(food_config.check_line("feijoada com arroz", 1));
    }

    #[test]
    fn test_parse_yaml_from_content() {
        let docs = parse_yaml_from_content("key: value");
        assert!(docs.is_ok());

        let docs = docs.unwrap();
        assert!(docs.len() == 1);

        let doc = docs[0].as_hash();
        assert!(doc.is_some());

        let doc = doc.unwrap();
        let value = doc.get(to_yaml_str!("key"));
        assert!(value.is_some());

        let value = value.unwrap().as_str();
        assert!(value.is_some() && value.unwrap() == "value");
    }

    #[test]
    fn test_default_config() {
        let config = Config::default();

        assert!(!config.restaurants.is_empty());
        assert_eq!(config.restaurants.len(), DEFAULT_RESTAURANTS.len());
    }

    #[test]
    fn test_get_bandex_yaml() {
        let values = parse_yaml_from_content("bandex: {restaurants: [{id: 1}]}").unwrap();
        let value = values.first().unwrap();
        let config = Config::get_bandex_yaml(value);

        assert!(config.is_some());
        assert!(config.unwrap().get(to_yaml_str!("restaurants")).is_some());

        let values = parse_yaml_from_content("nao_bandex: 42").unwrap();
        let value = values.first().unwrap();
        let config = Config::get_bandex_yaml(value);

        assert_eq!(config, None);
    }

    #[test]
    fn test_get_restaurants_yaml() {
        let values = parse_yaml_from_content("restaurants:\n  - id: 1\n  - id: 2").unwrap();
        let value = values.first().unwrap().as_hash().unwrap();
        let restaurant = Config::get_restaurants_yaml(value);

        assert!(restaurant.is_some());
        assert_eq!(restaurant.unwrap().len(), 2);

        let values = parse_yaml_from_content("restaurants:\n  1").unwrap();
        let value = values.first().unwrap().as_hash().unwrap();
        let restaurant = Config::get_restaurants_yaml(value);

        assert_eq!(restaurant, None);
    }

    #[test]
    fn test_get_foods_yaml() {
        let values = parse_yaml_from_content("foods:\n  liked: [bom]\n  disliked: [ruim]").unwrap();
        let value = values.first().unwrap().as_hash().unwrap();
        let config = Config::get_foods_yaml(value);

        assert!(config.is_some());
        assert_eq!(config.unwrap().len(), 2);

        let values = parse_yaml_from_content("restaurants:\n  - id: 1").unwrap();
        let value = values.first().unwrap().as_hash().unwrap();
        let config = Config::get_foods_yaml(value);

        assert_eq!(config, None);
    }

    #[test]
    fn test_get_liked_foods() {
        let values = parse_yaml_from_content("liked: [bom, muito bom]\ndisliked: [ruim]").unwrap();
        let value = values.first().unwrap().as_hash().unwrap();
        let config = Config::get_liked_foods_yaml(value);

        assert!(config.is_some());
        assert_eq!(config.unwrap().len(), 2);

        let values = parse_yaml_from_content("disliked: [ruim]").unwrap();
        let value = values.first().unwrap().as_hash().unwrap();
        let config = Config::get_liked_foods_yaml(value);

        assert_eq!(config, None);
    }

    #[test]
    fn test_get_disliked_foods() {
        let values = parse_yaml_from_content("liked: [bom]\ndisliked: [ruim, muito_ruim]").unwrap();
        let value = values.first().unwrap().as_hash().unwrap();
        let config = Config::get_disliked_foods_yaml(value);

        assert!(config.is_some());
        assert_eq!(config.unwrap().len(), 2);

        let values = parse_yaml_from_content("liked: [bom]").unwrap();
        let value = values.first().unwrap().as_hash().unwrap();
        let config = Config::get_disliked_foods_yaml(value);

        assert_eq!(config, None);
    }

    #[test]
    fn test_config_from_file_content() {
        let config = Config::from_file_content(
            "
            bandex:
                restaurants:
                    - id: 123
                    - id: 456
                      color: red",
        )
        .unwrap();

        assert!(config.restaurants.len() == 2);
        assert_eq!(
            config.restaurants[0],
            RestaurantConfig::new(123, DEFAULT_COLOR)
        );
        assert_eq!(
            config.restaurants[1],
            RestaurantConfig::new(456, Color::Red)
        );

        let config = Config::from_file_content("");
        assert!(config.is_err());
    }
}
