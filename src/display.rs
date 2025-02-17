use crate::types::{MenuType, MenusCache, RestaurantCode};
use chrono::Weekday;

/// Dias da semana
const WEEKDAYS: [Weekday; 5] = [
    Weekday::Mon,
    Weekday::Tue,
    Weekday::Wed,
    Weekday::Thu,
    Weekday::Fri,
];

/// Nome dos dias da semana
const WEEKDAY_NAMES: [&str; 7] = [
    "Segunda-feira",
    "Terça-feira",
    "Quarta-feira",
    "Quinta-feira",
    "Sexta-feira",
    "Sábado",
    "Domingo",
];

/// Hierarquia de cores para os títulos
///
/// Essas cores são usadas para diferenciar os níveis de hierarquia dos títulos
/// na função display::display_title.
const COLOR_HIERARCHY: [u8; 3] = [147, 183, 219];

/// Cria e mostra a logo do Bandex
///
/// Ref.: <https://patorjk.com/software/taag/#p=display&f=Doom&t=Bandex>
fn display_logo(with_colors: bool) {
    let reset = "\x1b[0m";
    let version = env!("CARGO_PKG_VERSION");

    let (bg, c1, c2, c3, c4, c5, c6) = if with_colors {
        (
            format!("\x1b[48;5;{}m", 0),
            format!("\x1b[38;5;{}m", 167),
            format!("\x1b[38;5;{}m", 185),
            format!("\x1b[38;5;{}m", 77),
            format!("\x1b[38;5;{}m", 068),
            format!("\x1b[38;5;{}m", 134),
            format!("\x1b[38;5;{}m", 170),
        )
    } else {
        (
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
        )
    };

    // Print logo with colors
    println!(
        "\
{reset}{bg}{c1}  _____                    {c4}_             {reset}
{bg}{c1} | ___ \\                  {c4}| |            {reset}
{bg}{c1} | |_/ /  {c2}__ _  {c3}_ __    {c4}__| |  {c5}___ {c6}__  __{reset}
{bg}{c1} | ___ \\ {c2}/ _  |{c3}|  _ \\  {c4}/ _  | {c5}/ _ \\{c6}\\ \\/ /{reset}
{bg}{c1} | |_/ /{c2}| (_| |{c3}| | | |{c4}| (_| |{c5}|  __/ {c6}>  < {reset}
{bg}{c1} |____/  {c2}\\__,_|{c3}|_| |_| {c4}\\__,_| {c5}\\___/{c6}/_/\\_\\ {version}{reset}\n"
    );
}

/// Gera e mostra um título colorido e posicionado a partir do seu "nível"
///
/// * `title`: Título a ser exibido.
/// * `level`: Nível de hierarquia do título, um número inteiro entre [0, 2].
fn print_title(title: String, level: usize) {
    let color = format!("\x1b[38;5;{}m", COLOR_HIERARCHY[level as usize]);
    let reset = "\x1b[0m";
    let title = format!(" {title} ");
    let header_len: usize = (50 - 2 * level) as usize;

    println!(
        "{color}{0:level$}# {title:~^header_len$} #{0:level$}{reset}",
        ""
    );
}

/// Mostra uma refeição de um dia da semana específico.
/// * `menus_cache`: Cache dos cardápios.
/// * `restaurant_codes`: Lista dos restaurantes pedidos.
/// * `menu_type`: Tipo de refeição a ser exibido.
/// * `weekday`: Dia da semana a ser exibido.
async fn display_menus_by_type(
    menus_cache: &mut MenusCache,
    restaurant_codes: &Vec<RestaurantCode>,
    menu_type: &MenuType,
    weekday: Weekday,
) {
    print_title(menu_type.to_string(), 1);

    for code in restaurant_codes {
        let (restaurant_name, menu) = match menus_cache
            .get_name_and_menu(*code, menu_type, weekday)
            .await
        {
            Some((name, menu)) => (name, menu),
            None => {
                println!("   Erro: Não foi possível carregar esse cardápio (Rest {code})");
                continue;
            }
        };

        print_title(restaurant_name.clone(), 2);

        if menu.content == "Fechado" {
            println!("   ✘ Fechado");
        } else {
            println!("\n   ➤  {}", menu.content.replace("\n", "\n   ➤  "));
            if let Some(calorific_value) = menu.calorific_value {
                println!("\n     Valor energético: {} kcal", calorific_value);
            }
            println!("\n### Observação: {} ###", menu.observation);
        }
        println!();
    }
}

/// Mostra todas as refeições para um dia da semana específico.
/// * `menus_cache`: Cache dos cardápios.
/// * `restaurant_codes`: Lista dos restaurantes pedidos.
/// * `menu_type`: Tipo de refeição a ser exibido.
///     * `Some(menu_type)`: Mostra apenas as refeições do tipo especificado.
///     * `None`: Mostra todas as refeições.
/// * `weekday`: Dia da semana a ser exibido.
async fn display_menus_of_day(
    menus_cache: &mut MenusCache,
    restaurant_codes: &Vec<RestaurantCode>,
    menu_type: &Option<MenuType>,
    weekday: Weekday,
) {
    let weekday_name = WEEKDAY_NAMES[weekday.num_days_from_monday() as usize];

    print_title(weekday_name.to_string(), 0);

    if let Some(menu_type) = menu_type {
        display_menus_by_type(menus_cache, restaurant_codes, menu_type, weekday).await;
    } else {
        display_menus_by_type(menus_cache, restaurant_codes, &MenuType::Lunch, weekday).await;
        display_menus_by_type(menus_cache, restaurant_codes, &MenuType::Dinner, weekday).await;
    }
}

/// Mostra todas as refeições para os dias da semana pedidos:
/// - `restaurant_codes`: Lista dos restaurantes pedidos.
/// - `menu_type`: Tipo de refeição a ser exibido.
///     - `Some(menu_type)`: Mostra apenas as refeições do tipo especificado.
///     - `None`: Mostra todas as refeições.
/// - `weekday`: Dia da semana a ser exibido.
///     - `Some(weekday)`: Mostra apenas as refeições do dia especificado.
///     - `None`: Mostra todas as refeições da semana.
pub async fn display_all_menus(
    restaurant_codes: Vec<RestaurantCode>,
    weekday: Option<Weekday>,
    menu_type: Option<MenuType>,
) {
    let mut menus_cache = MenusCache::new();

    display_logo(true);

    if let Some(weekday) = weekday {
        display_menus_of_day(&mut menus_cache, &restaurant_codes, &menu_type, weekday).await;
    } else {
        for weekday in WEEKDAYS {
            display_menus_of_day(&mut menus_cache, &restaurant_codes, &menu_type, weekday).await;
        }
    }
}
