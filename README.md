# PetClinicRUST
API em rust para consultório veterinário

![screenshot](assets/banner.png)

## Features:
- Agendamento de consultas veterinárias
- Histórico de vacinas e tratamento para os pets
- Pesquisa de satisfação e contatos através de SMS e Whatsapp

## Stack:
- Actix Web
- Sqlx
- Postgres

## Steps to run the project
- cargo sqlx prepare (para gerar a pasta com os metadados sql, se desejar suba no controle de versão)
  >  cargo sqlx prepare

  `query data written to .sqlx in the current directory; please check this into version control`

- docker compose up
- Follow the link:  http://localhost:3000/api/pets

## Steps to run the project without Docker
- Run the SQL script at  /database/create_postgres.sql
- Set DATABASE_URL in the .env file
- cargo run (verificar se o compilador conectou ao Postgres e validou o sql do projeto)
- Follow the link:  http://localhost:3000/api/pets

## Deploy the project to a Kubernetes cluster
- Run
    > kubectl apply -f deploy/petclinic.yaml
