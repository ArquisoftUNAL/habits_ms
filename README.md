# Habitus: Habitus Microservice

## Description
This microservice provides endpoints for saving habits, assigning categories,
managing habits recurrences (times and frequency they are repeated within a certain
time period) and tracking habit progress by collecting whether Y/N or quantitative data
provided by the user

## Endpoints
There is a Postman collection with all of enabled and function endpoints in /docs/ folder

However here is an example of endpoints and their usage

### Create a category

Path: /api/v1/categories/
Method: POST
Body: 
```json
{
    name: "<Your category name goes here>"
}
```
Expected result: 201 (OK)

### Create an habit

Path: /api/v1/habits/
Method: POST
Body: 
```json
{
    [String] name: "<Your habit name goes here>",
    [String] description: "<Your habit description goes here>",
    [UUIDv4] category: "<Your category id goes here>"
    [Boolean] is_favorite: "<Whether your habit is favorite or not>"
    [String] color: "<Habit display color>"
    [String] kind: "<Type of habit (YN or ME)>"
    [String] units: "<Your measurement units>"
    [String (24)] user_id: "<User id (24 characters identifier)>"
}
```
Expected result: 201 (OK)

## Installation

You can run the commands describen in the Makefile to run the project locally (Suggestion: Run them in the same order as they appear in the Makefile)

Example (From project's root folder):
```bash
cd database
make docker-build-db
make docker-run-db
cd ..
make docker-build-ms
make docker-run-ms
```

## Requirements

Docker Desktop

If you want to build everything from scratch without using docker containers:
    - Postgres
    - Cargo
    - Rust
    - Diesel CLI