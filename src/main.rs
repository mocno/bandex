use std::{env, io::Error, path::PathBuf};

use cli::cli;
use config::Config;
use display::Display;

mod cli;
mod config;
mod display;
mod parse_dwr;
mod request;
mod types;

const ENV_VAR_BANDEX_CONFIG: &str = "BANDEX_CONFIG_FILE";

/// Função que orquestra a execução do programa
///
/// - Recebe dados enviados no CLI, como quais cardápios devem ser exibidos
/// - Lê, se existir, o arquivo de configurações, que define que restaurantes devem ser exibidos
/// - Mostra os dados dos cardápios considerando as configurações
#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut display = Display::new();

    Display::show_logo(true);

    let (menu_type, weekday, config_filepath) = cli()?;

    let config_filepath = config_filepath.or_else(|| {
        env::var(ENV_VAR_BANDEX_CONFIG)
            .ok()
            .map(|path| PathBuf::from(path))
    });

    let config = match config_filepath {
        None => Config::default(),
        Some(filepath) => Config::from_file(filepath).unwrap_or_else(|message| {
            Display::error_message(format!(
                "Erro ao ler o arquivo de configurações: {}",
                message
            ));
            Config::default()
        }),
    };

    display.show_menus(weekday, menu_type, &config).await;
    Ok(())
}
