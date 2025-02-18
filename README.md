# Bandex
O Bandex é uma interface em linha de comando para consultar os cardápios dos restaurantes da USP, os bandejões.
Permitindo ver as refeições dos bandeijões de acordo com o seu horário e permitindo ver os cardápios da semana.

## Exemplos de uso

Para ler os cardápios da próxima refeição do dia atual, execute:
```sh
bandex
```
Para ler apenas os cardápios do almoço do dia atual, execute:
```sh
bandex -a
```
De forma parecida, para ler apenas os cardápios do janta do dia atual, execute:
```sh
bandex -j
```
Se deseja que apareça os dois, o almoço e a janta, basta executar:
```sh
bandex -ja
```
Agora, para ver os cardápios de um dia específico, adicione o parâmetro `-w` ou `--weekday`,
assim, para consultar os cardápios da segunda-feira (segunda-feira é 1, terça-feira é 2, ..., domingo é 7):
```sh
bandex -w 1
```
Por outro lado, para ver os cardápios de todos os dias da semana, adicione o parâmetro `-e` ou `--everything`:
```sh
bandex -e
```
Sinta-se livre para misturar os parâmetros mostrados.
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
cargo run -- <argumento>
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
