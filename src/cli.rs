use crate::types::MenuType;

use chrono::{Local, NaiveTime, Weekday};
use clap::Parser;

// Custom parser function for chrono::Weekday
fn parse_weekday(s: &str) -> Result<Weekday, String> {
    match s.to_lowercase().as_str() {
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

pub async fn cli() -> (Option<MenuType>, bool, Option<Weekday>) {
    let cli = Cli::parse();

    if cli.weekday != None && cli.everything {
        panic!("Escolha mostrar um dia especifico (-d <WEEKDAY>) ou todos os dias (-E)");
    }

    let current_time = Local::now().time();

    let menu_type = match (cli.everything, cli.lunch, cli.dinner) {
        (_, true, false) => Some(MenuType::Lunch),
        (_, false, true) => Some(MenuType::Dinner),
        (false, false, false) => get_menu_type_by_datetime(current_time),
        _ => None,
    };

    (menu_type, cli.everything, cli.weekday)
}
