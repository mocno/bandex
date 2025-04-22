/*!
Funções para exibir informações do Bandex.

Este módulo contém estruturas e funções para exibir informações do Bandex.
 */

use crate::{
    config::Config,
    types::{Menu, MenuType, MenusCache},
};
use chrono::Weekday;
use colored::{Color, Colorize};

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

/// Cor para os titulos de dia da semana
const COLOR_WEEK_DAY: Color = Color::TrueColor {
    r: 153,
    g: 153,
    b: 255,
};

/// Cor para os titulos de tipo de menu (almoço ou jantar)
const COLOR_MENU_TYPE: Color = Color::TrueColor {
    r: 204,
    g: 153,
    b: 255,
};

/// Estrutura para controlar a exibição das informações do Bandex.
pub struct Display {
    /// Cache dos menus e dos nomes dos restaurantes, de modo a evitar requisições desnecessárias.
    menus_cache: MenusCache,
}

/// Cria 3 tipos de titulos que podem ser coloridos.
macro_rules! print_header {
    (H3, $title:expr, $color:expr) => {{
        let content = format!("  # {:~^46} #  ", format!(" {} ", $title)).color($color);
        println!("{}", content);
    }};

    (H2, $title:expr, $color:expr) => {{
        let content = format!(" # {:~^48} # ", format!(" {} ", $title)).color($color);
        println!("{}", content);
    }};

    (H1, $title:expr, $color:expr) => {{
        let content = format!("# {:~^50} #", format!(" {} ", $title)).color($color);
        println!("{}", content);
    }};
}

impl Display {
    /// Reseta a cor do texto no terminal.
    const RESET: &str = "\x1b[0m";
    /// Cores da logo do Bandex, o primeiro para a cor de fundo e os demais para cada letra.
    const LOGO_COLORS: (&str, &str, &str, &str, &str, &str, &str) = (
        "\x1b[48;5;0m",
        "\x1b[38;5;167m",
        "\x1b[38;5;185m",
        "\x1b[38;5;77m",
        "\x1b[38;5;68m",
        "\x1b[38;5;134m",
        "\x1b[38;5;170m",
    );

    /// Cria uma instância vazia do Display (sem cache).
    pub fn new() -> Self {
        Self {
            menus_cache: MenusCache::new(),
        }
    }

    /// Mostra a logo do Bandex com ou sem cor.
    ///
    /// Referência da fonte: <https://patorjk.com/software/taag/#p=display&f=Doom&t=Bandex>.
    pub fn show_logo(with_colors: bool) {
        let reset = Self::RESET;
        let version = env!("CARGO_PKG_VERSION");

        let (bg, c1, c2, c3, c4, c5, c6) = if with_colors {
            Display::LOGO_COLORS
        } else {
            ("", "", "", "", "", "", "")
        };

        // Print logo with colors
        println!(
            "{reset}\
{bg}{c1}  _____                    {c4}_             {reset}
{bg}{c1} | ___ \\                  {c4}| |            {reset}
{bg}{c1} | |_/ /{c2}  __ _ {c3} _ __  {c4}  __| |{c5}  ___ {c6}__  __{reset}
{bg}{c1} | ___ \\{c2} / _  |{c3}|  _ \\ {c4} / _  |{c5} / _ \\{c6}\\ \\/ /{reset}
{bg}{c1} | |_/ /{c2}| (_| |{c3}| | | |{c4}| (_| |{c5}|  __/ {c6}>  < {reset}
{bg}{c1} |____/ {c2} \\__,_|{c3}|_| |_|{c4} \\__,_|{c5} \\___/{c6}/_/\\_\\ {version}{reset}\n"
        );
    }

    /// Mostra uma mensagem simples de erro formatada.
    pub fn error_message(msg: String) {
        println!("   Erro: {}", msg);
    }

    /// Mostra um cardápio a partir de uma instancia de `Menu`.
    fn show_menu(menu: Menu) {
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

    /// Mostra uma refeição de um tipo específico `menu_type` (almoço ou jantar) de um dia específico `weekday`.
    ///
    /// O parâmetro de configuração `config` define quais restaurantes devem ser exibidos (e com quais cores).
    async fn show_menus_by_type(
        &mut self,
        menu_type: &MenuType,
        weekday: Weekday,
        config: &Config,
    ) {
        print_header!(H2, menu_type.to_string(), COLOR_MENU_TYPE);

        for restaurant in config.restaurants.iter() {
            if let Some((restaurant_name, menu)) = self
                .menus_cache
                .get_name_and_menu(restaurant.id, menu_type, weekday)
                .await
            {
                print_header!(H3, restaurant_name, restaurant.color);
                Display::show_menu(menu);
            } else {
                Display::error_message(format!(
                    "Não foi possível carregar dados desse restaurante (Rest {})",
                    restaurant.id
                ));
            }
        }
    }

    /// Mostra todas as refeições para um dia da semana específico, definido pelo `weekday`.
    /// O tipo da refeição segue o seguinte padrão:
    /// * `Some(menu_type)`: Mostra apenas as refeições de um tipo especificado.
    /// * `None`: Mostra todas as refeições.
    ///
    /// O parâmetro de configuração `config` define quais restaurantes devem ser exibidos (e com quais cores).
    async fn show_menus_by_day(
        &mut self,
        menu_type: &Option<MenuType>,
        weekday: Weekday,
        config: &Config,
    ) {
        let weekday_name = WEEKDAY_NAMES[weekday.num_days_from_monday() as usize];

        print_header!(H1, weekday_name, COLOR_WEEK_DAY);

        if let Some(menu_type) = menu_type {
            self.show_menus_by_type(menu_type, weekday, config).await;
        } else {
            self.show_menus_by_type(&MenuType::Lunch, weekday, config)
                .await;
            self.show_menus_by_type(&MenuType::Dinner, weekday, config)
                .await;
        }
    }

    /// Mostra todas os cardápios que devem ser exibidos a partir dos parâmetros.
    /// Os parâmetros `menu_type` e `weekday` seguem o seguinte padrão:
    /// * `menu_type`: Tipo de refeição a ser exibido.
    ///     * `Some(menu_type)`: Mostra apenas as refeições do tipo especificado.
    ///     * `None`: Mostra todas as refeições.
    /// * `weekday`: Dia da semana a ser exibido.
    ///     * `Some(weekday)`: Mostra apenas as refeições do dia especificado.
    ///     * `None`: Mostra todas as refeições da semana.
    ///
    /// O parâmetro de configuração `config` define quais restaurantes devem ser exibidos (e com quais corer).
    pub async fn show_menus(
        &mut self,
        weekday: Option<Weekday>,
        menu_type: Option<MenuType>,
        config: &Config,
    ) {
        if let Some(weekday) = weekday {
            self.show_menus_by_day(&menu_type, weekday, config).await;
        } else {
            for weekday in WEEKDAYS {
                self.show_menus_by_day(&menu_type, weekday, config).await;
            }
        }
    }
}
