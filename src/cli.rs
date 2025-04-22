/*!
CLI do Bandex

Este arquivo define a interface de linha de comando para o aplicativo.

O CLI processa argumentos como:
- `-a`: Para visualizar apenas almoços
- `-j`: Para visualizar apenas jantares
- `-w`: Para especificar o dia da semana
- `-e`: Para mostrar todos os cardápios da semana
- `-c`: Para especificar um arquivo de configuração personalizado
*/

use std::{
    io::{Error, ErrorKind},
    path::PathBuf,
};

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
/// Veja as refeições de qualquer bandeijão de acordo com o seu horário.
///
/// Esse projeto é publico, sinta-se a vontade para contribuir!
/// O repositorio do projeto se encontra em <https://github.com/mocno/bandex>
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

    /// Arquivo de configuração do bandex
    ///
    /// Esse arquivo, em YAML, configura os restaurantes que deseja ver e com quais cores.
    /// Para referencia, há exemplos no repositório do projeto.
    #[arg(short, long)]
    config: Option<PathBuf>,
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
pub fn cli() -> Result<(Option<MenuType>, Option<Weekday>, Option<PathBuf>), Error> {
    let cli = Cli::parse();

    if cli.weekday != None && cli.everything {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Escolha mostrar um dia especifico (-w <WEEKDAY>) ou todos os dias (-E)",
        ));
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

    Ok((menu_type, weekday, cli.config))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_weekday() {
        for (weekday_str, expected_weekday) in [
            ("1", Weekday::Mon),
            ("2", Weekday::Tue),
            ("3", Weekday::Wed),
            ("4", Weekday::Thu),
            ("5", Weekday::Fri),
            ("6", Weekday::Sat),
            ("7", Weekday::Sun),
        ] {
            let result = parse_weekday(weekday_str);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), expected_weekday);
        }

        let weekday_str = "0";
        let result = parse_weekday(weekday_str);
        assert!(result.is_err());

        let weekday_str = "8";
        let result = parse_weekday(weekday_str);
        assert!(result.is_err());
    }

    #[test]
    fn test_cli_get_right_info() {
        let cli = Cli::try_parse_from(vec!["bandex", "-a", "-w", "2", "-c", "config.yaml"]);
        assert!(cli.is_ok());

        let cli = cli.unwrap();

        assert!(
            cli.lunch,
            "CLI parses \"-a -w 2 -c config.yaml\": lunch must be true"
        );
        assert!(
            !cli.dinner,
            "CLI parses \"-a -w 2 -c config.yaml\": dinner must be false"
        );
        assert!(
            cli.weekday.is_some() && cli.weekday.unwrap() == Weekday::Tue,
            "CLI parses \"-a -w 2 -c config.yaml\": weekday must be Tuesday"
        );
        assert!(
            cli.config.is_some() && cli.config.unwrap() == PathBuf::from("config.yaml"),
            "CLI parses \"-a -w 2 -c config.yaml\": config file must be \"config.yaml\""
        );

        let cli = Cli::try_parse_from(vec!["bandex", "-aj", "-e"]);
        assert!(cli.is_ok());

        let cli = cli.unwrap();

        assert!(cli.lunch, "CLI parses \"-aj -e\": lunch must be true");
        assert!(cli.dinner, "CLI parses \"-aj -e\": dinner must be true");
        assert!(
            cli.everything,
            "CLI parses \"-aj -e\": everything must be true"
        );
        assert!(
            cli.config.is_none(),
            "CLI parses \"-aj -e\": config file must be None"
        );

        let cli = Cli::try_parse_from(vec!["bandex", "-w", "0"]);
        assert!(cli.is_err());

        let cli = Cli::try_parse_from(vec!["bandex", "-w", "8"]);
        assert!(cli.is_err());
    }

    #[test]
    fn test_get_menu_type_by_datetime() {
        let menu_type = get_menu_type_by_datetime(NaiveTime::from_hms_opt(5, 31, 47).unwrap());
        assert!(menu_type.is_none());

        let menu_type = get_menu_type_by_datetime(NaiveTime::from_hms_opt(6, 11, 32).unwrap());
        assert!(menu_type.is_some() && menu_type.unwrap() == MenuType::Lunch);

        let menu_type = get_menu_type_by_datetime(NaiveTime::from_hms_opt(13, 52, 19).unwrap());
        assert!(menu_type.is_some() && menu_type.unwrap() == MenuType::Lunch);

        let menu_type = get_menu_type_by_datetime(NaiveTime::from_hms_opt(14, 35, 12).unwrap());
        assert!(menu_type.is_some() && menu_type.unwrap() == MenuType::Dinner);

        let menu_type = get_menu_type_by_datetime(NaiveTime::from_hms_opt(19, 49, 41).unwrap());
        assert!(menu_type.is_some() && menu_type.unwrap() == MenuType::Dinner);

        let menu_type = get_menu_type_by_datetime(NaiveTime::from_hms_opt(20, 12, 19).unwrap());
        assert!(menu_type.is_none());
    }
}
