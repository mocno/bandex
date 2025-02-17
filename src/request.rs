use crate::types::RestaurantCode;
use reqwest;

/// URL para obter o nome do restaurante
const GET_RESTAURANT_NAME_URL: &str = "https://uspdigital.usp.br/rucard/dwr/call/plaincall/CardapioControleDWR.obterRestauranteUsp.dwr";

/// URL para obter o cardápio do restaurante
const GET_MENU_URL: &str = "https://uspdigital.usp.br/rucard/dwr/call/plaincall/CardapioControleDWR.obterCardapioRestUSP.dwr";

/// Função faz a requisição para obterRestauranteUsp
pub async fn request_rest_name(code: RestaurantCode) -> Result<String, reqwest::Error> {
    let c0_param0 = format!("string:{code}");
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
pub async fn request_menu(code: RestaurantCode) -> Result<String, reqwest::Error> {
    let c0_param0 = format!("string:{code}");
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
