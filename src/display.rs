use crate::types::{Menu, MenuType, MenusCache, RestaurantCode};
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

/// Estrutura para controlar a exibição das informações do Bandex.
pub struct Display {
    /// Cache dos menus e dos nomes dos restaurantes, de modo a evitar requisições desnecessárias.
    menus_cache: MenusCache,
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
    fn logo(with_colors: bool) {
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
    {bg}{c1}      _____                    {c4}_             {reset}
    {bg}{c1} | ___ \\                  {c4}| |            {reset}
    {bg}{c1} | |_/ /{c2}  __ _ {c3} _ __  {c4}  __| |{c5}  ___ {c6}__  __{reset}
    {bg}{c1} | ___ \\{c2} / _  |{c3}|  _ \\ {c4} / _  |{c5} / _ \\{c6}\\ \\/ /{reset}
    {bg}{c1} | |_/ /{c2}| (_| |{c3}| | | |{c4}| (_| |{c5}|  __/ {c6}>  < {reset}
    {bg}{c1} |____/ {c2} \\__,_|{c3}|_| |_|{c4} \\__,_|{c5} \\___/{c6}/_/\\_\\ {version}{reset}\n"
        );
    }

    /// Mostra um título adicionando cor e posicionamento a partir do seu nível, que pode variar de 0 a 2.
    fn title(title: String, level: usize) {
        let color = format!("\x1b[38;5;{}m", COLOR_HIERARCHY[level as usize]);
        let reset = "\x1b[0m";
        let title = format!(" {title} ");
        let header_len: usize = (50 - 2 * level) as usize;

        println!(
            "{color}{0:level$}# {title:~^header_len$} #{0:level$}{reset}",
            ""
        );
    }

    /// Mostra uma mensagem simples de erro formatada.
    fn error_message(msg: String) {
        println!("   Erro: {}", msg);
    }

    /// Mostra um cardápio a partir de uma instancia de `Menu`.
    fn menu(menu: Menu) {
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

    /// Mostra uma refeição de um tipo específico (almoço ou jantar) para cada restaurante em `restaurant_codes`.
    async fn menus_by_type(
        &mut self,
        restaurant_codes: &Vec<RestaurantCode>,
        menu_type: &MenuType,
        weekday: Weekday,
    ) {
        Display::title(menu_type.to_string(), 1);

        for code in restaurant_codes {
            if let Some((restaurant_name, menu)) = self
                .menus_cache
                .get_name_and_menu(*code, menu_type, weekday)
                .await
            {
                Display::title(restaurant_name, 2);
                Display::menu(menu);
            } else {
                Display::error_message(format!(
                    "Não foi possível carregar dados desse restaurante (Rest {code})"
                ));
            }
        }
    }

    /// Mostra todas as refeições de todos os restaurantes do `restaurant_codes` para o dia da semana específico definido pelo `weekday`.
    /// O tipo da refeição segue o seguinte padrão:
    /// * `Some(menu_type)`: Mostra apenas as refeições de um tipo especificado.
    /// * `None`: Mostra todas as refeições.
    async fn menus_by_day(
        &mut self,
        restaurant_codes: &Vec<RestaurantCode>,
        menu_type: &Option<MenuType>,
        weekday: Weekday,
    ) {
        let weekday_name = WEEKDAY_NAMES[weekday.num_days_from_monday() as usize];

        Display::title(weekday_name.to_string(), 0);

        if let Some(menu_type) = menu_type {
            self.menus_by_type(restaurant_codes, menu_type, weekday)
                .await;
        } else {
            self.menus_by_type(restaurant_codes, &MenuType::Lunch, weekday)
                .await;
            self.menus_by_type(restaurant_codes, &MenuType::Dinner, weekday)
                .await;
        }
    }

    /// Mostra todas as refeições dos restaurantes em `restaurant_codes` para os dias da semana pedidos.
    /// Os parâmetros `menu_type` e `weekday` seguem o seguinte padrão:
    /// * `menu_type`: Tipo de refeição a ser exibido.
    ///     * `Some(menu_type)`: Mostra apenas as refeições do tipo especificado.
    ///     * `None`: Mostra todas as refeições.
    /// * `weekday`: Dia da semana a ser exibido.
    ///     * `Some(weekday)`: Mostra apenas as refeições do dia especificado.
    ///     * `None`: Mostra todas as refeições da semana.
    pub async fn all_menus(
        &mut self,
        restaurant_codes: Vec<RestaurantCode>,
        weekday: Option<Weekday>,
        menu_type: Option<MenuType>,
    ) {
        Display::logo(true);

        if let Some(weekday) = weekday {
            self.menus_by_day(&restaurant_codes, &menu_type, weekday)
                .await;
        } else {
            for weekday in WEEKDAYS {
                self.menus_by_day(&restaurant_codes, &menu_type, weekday)
                    .await;
            }
        }
    }
}
