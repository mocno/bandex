use crate::types::MenuType;
use chrono::{Datelike, Local, NaiveTime, Weekday};
use clap::Parser;

/// Ler dia da semana a partir de um numero na string (segunda: 1, terça: 2, ..., domingo: 7)
fn parse_weekday(s: &str) -> Result<Weekday, String> {
    match s {
        "1" => Ok(Weekday::Mon),
        "2" => Ok(Weekday::Tue),
        "3" => Ok(Weekday::Wed),
        "4" => Ok(Weekday::Thu),
        "5" => Ok(Weekday::Fri),
        "6" => Ok(Weekday::Sat),
        "7" => Ok(Weekday::Sun),
        _ => Err("O dia de semada é um inteiro entre 1 e 7".to_owned()),
    }
}

/// Mostra o cardápio dos restaurantes da USP
///
/// Veja as refeições dos bandeijões de acordo com o seu horário
#[derive(Debug, Parser)]
#[command(
    version,
    about,
    after_help = "Se deseja consultar o cardápio do almoço e da janta, pode-se colocar os dois argumentos \"-j -a\" ou \"-aj\"."
)]
struct Cli {
    /// Mostra apenas os almoços
    #[arg(short = 'a')]
    lunch: bool,
    /// Mostra apenas os jantares
    #[arg(short = 'j')]
    dinner: bool,
    /// Mostra as refeições do dia escolhido, por padrão, o dia atual (Segunda = 1, Terça = 2, etc)
    #[arg(short, long, value_parser=parse_weekday)]
    weekday: Option<Weekday>,
    /// Mostra todas as refeições da semana!
    #[arg(short, long)]
    everything: bool,
}

/// Retorna o tipo da refeição com base na hora atual.
///
/// O valor de retorno depende do horário atual:
/// - Almoço: retorna `Some(MenuType::Lunch)` entre  6:00 e 14:00
/// - Janta: retorna `Some(MenuType::Dinner)` entre 14:00 e 20:00
/// - Todos: retorna `None` entre 20:00 e  6:00
fn get_menu_type_by_datetime(time: NaiveTime) -> Option<MenuType> {
    let lunch_start = NaiveTime::from_hms_opt(6, 0, 0)?;
    let lunch_end_and_dinner_start = NaiveTime::from_hms_opt(14, 0, 0)?;
    let dinner_end = NaiveTime::from_hms_opt(20, 0, 0)?;

    if lunch_start < time && time < lunch_end_and_dinner_start {
        Some(MenuType::Lunch)
    } else if lunch_end_and_dinner_start <= time && time < dinner_end {
        Some(MenuType::Dinner)
    } else {
        None
    }
}

/// Lê os parâmetros de linha de comando e os intepreta extraindo:
/// - O horário da refeição que se busca
///     - `Some(time)`: se o usuário especificou um horário específico
///     - `None`: se o usuário não escolheu um horário (ou seja, todos os horários)
/// - O dia da semana escolhido:
///     - `Some(weekday)`: se o usuário especificou um dia
///     - `None`: se o usuário não quer todos os dias da semana
pub fn cli() -> Result<(Option<MenuType>, Option<Weekday>), &'static str> {
    let cli = Cli::parse();

    if cli.weekday != None && cli.everything {
        return Err("Escolha mostrar um dia especifico (-d <WEEKDAY>) ou todos os dias (-E)");
    }

    let current_time = Local::now().time();

    let menu_type = match (cli.everything, cli.lunch, cli.dinner) {
        (_, true, false) => Some(MenuType::Lunch),
        (_, false, true) => Some(MenuType::Dinner),
        (false, false, false) => get_menu_type_by_datetime(current_time),
        _ => None,
    };

    let weekday = match (cli.everything, cli.weekday) {
        (false, Some(weekday)) => Some(weekday),
        (false, None) => Some(Local::now().naive_local().weekday()),
        (true, _) => None,
    };

    Ok((menu_type, weekday))
}
