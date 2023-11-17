# PetClinicRUST
API em rust para consultório veterinário

## Features:
- Agendamento de consultas veterinárias
- Histórico de vacinas e tratamento para os pets

## Stack:
- Actix Web
- Sqlx
- Postgres

## Steps to run the project:
- Run the sql artifact at /database/create_postgres.sql
- Cargo run

## Incompatibilidade encontrada entre o Docker e o SQLX
- o sqlx tenta expandir as macros quando o docker compose executa o build dos containers, isso
provoca a falha no build do container
- rodando com uma instancia do POSTGRES na máquina de desenvolvimento e com o o banco já criado
é possivel fazer a compilação normalmente e rodar o projeto
