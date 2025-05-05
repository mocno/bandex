/*!
Funções de requisições para o USP Digital

Este módulo fornece funções para realizar requisições HTTP aos serviços da USP Digital
para obter informações sobre restaurantes universitários.

As funções disponíveis permitem:
- Obter o nome de um restaurante pelo seu código
- Obter o cardápio de um restaurante pelo seu código

Todas as funções retornam o conteúdo da requisição ou um erro de requisição `reqwest::Error`.
*/

use crate::types::RestaurantID;
use reqwest;

/// URL para obter o nome do restaurante
const GET_RESTAURANT_NAME_URL: &str = "https://uspdigital.usp.br/rucard/dwr/call/plaincall/CardapioControleDWR.obterRestauranteUsp.dwr";

/// URL para obter o cardápio do restaurante
const GET_MENU_URL: &str = "https://uspdigital.usp.br/rucard/dwr/call/plaincall/CardapioControleDWR.obterCardapioRestUSP.dwr";

/// Função faz a requisição para obterRestauranteUsp
pub async fn request_rest_name(restaurant_id: RestaurantID) -> Result<String, reqwest::Error> {
    let c0_param0 = format!("string:{restaurant_id}");
    let params = [
        ("page", ""),
        ("windowName", ""),
        ("c0-id", "a"),
        ("batchId", "0"),
        ("callCount", "1"),
        ("instanceId", "0"),
        ("c0-param0", c0_param0.as_str()),
        ("c0-scriptName", "CardapioControleDWR"),
        ("c0-methodName", "obterRestauranteUsp"),
        (
            "scriptSessionId",
            "$$cHGUA$xN69qjKpKBPg$r4l5bn/pM7m5bn-HStgR4BS4",
        ),
    ];
    let client = reqwest::Client::new();
    let response = client
        .post(GET_RESTAURANT_NAME_URL)
        .form(&params)
        .send()
        .await?
        .text()
        .await?;

    return Ok(response);
}

/// Função faz a requisição para obterCardapioRestUSP
pub async fn request_menu(restaurant_id: RestaurantID) -> Result<String, reqwest::Error> {
    let c0_param0 = format!("string:{restaurant_id}");
    let params = [
        ("page", ""),
        ("windowName", ""),
        ("c0-id", "a"),
        ("batchId", "0"),
        ("callCount", "1"),
        ("instanceId", "0"),
        ("c0-param0", c0_param0.as_str()),
        ("c0-scriptName", "CardapioControleDWR"),
        ("c0-methodName", "obterCardapioRestUSP"),
        (
            "scriptSessionId",
            "$$cHGUA$xN69qjKpKBPg$r4l5bn/pM7m5bn-HStgR4BS4",
        ),
    ];
    let client = reqwest::Client::new();
    let response = client
        .post(GET_MENU_URL)
        .form(&params)
        .send()
        .await?
        .text()
        .await?;

    return Ok(response);
}

#[cfg(test)]
mod test {
    /// ID do restaurante Central
    const RESTAURANT_CENTRAL: RestaurantID = 6;

    use super::*;

    #[tokio::test]
    async fn test_request_rest_name() {
        let response = request_rest_name(RESTAURANT_CENTRAL).await.unwrap();
        assert!(response.contains("Restaurante Central"));
    }

    #[tokio::test]
    async fn test_request_menu() {
        let response = request_menu(RESTAURANT_CENTRAL).await.unwrap();
        assert!(response.contains("cdpdia"));
    }
}
