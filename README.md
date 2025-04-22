# Bandex

O Bandex é uma interface em linha de comando para consultar os cardápios dos restaurantes da USP, os bandejões.
Permitindo ver as refeições dos bandeijões de acordo com o seu horário e permitindo ver os cardápios da semana.

## Exemplos de uso

Para ler os cardápios da próxima refeição do dia atual, execute:

```sh
bandex
```

Se deseja ler apenas os cardápios de almoço use o parametro "--lunch" ou "-a", se deseja ler apenas os cardápios
dos jantares, use o parametro "--dinner" ou "-j". Se deseja forçar que apareça os dois, o almoço e o jantar,
adicione os dois parametros ("-ja").

Agora, para ver os cardápios de um dia específico, adicione o parâmetro `-w` ou `--weekday`, assim, para
consultar os cardápios da segunda-feira (segunda-feira é 1, terça-feira é 2, ..., domingo é 7):

```sh
bandex -w 1
```

Por outro lado, para ver os cardápios de todos os dias da semana, adicione o parâmetro `-e` ou `--everything`:

```sh
bandex -e
```

Para adicionar configurações, use o parâmetro `-c` ou `--config` seguido do caminho para o arquivo de configuração:

```sh
bandex -c ./configs-examples/bandex-config.yaml
```

Alguns exemplos de configurações então disponiveis no diretório `configs-examples`.

Sinta-se livre para misturar os parâmetros.
Por fim, para uma ajuda rápida busque pela ajuda completa:

```sh
bandex --help
```

## Instalação

Se deseja executar o projeto a partir do código fonte, execute o seguinte comando:

```sh
cargo run
```

Nota importante: Se deseja adicionar um argumento, execute o seguinte comando:

```sh
cargo run -- <argumentos>
```

Muitas vezes é melhor gerar um executável do projeto para facilitar a execução,
nesse caso, execute o seguinte comando para criar um arquivo executável em `./target/release/bandex`:

```sh
cargo build --release
```

## Documentação

Pode-se gerar a documentação do projeto usando o comando:

```sh
cargo doc
```

## Licença MIT

Sinta-se livre para modificar e distribuir o projeto, desde que mantenha os créditos dos autores.

## Contribuição

Sinta-se livre e incentivado para contribuir com o projeto.
Esse projeto é aberto e feito para receber contribuições de qualquer pessoa que queira ajudar a melhorar o projeto.
