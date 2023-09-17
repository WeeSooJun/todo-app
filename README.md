# Todo App
Quick and dirty todo app to try out Axum and PostgreSQL integration in rust.

### Setup PostgreSQL (MacOS - Homebrew)
##### Install
`brew install postgresql@15`

##### Start the PostgreSQL server
`brew services start postgresql@15`

##### Create a superuser called postgres
`createuser -s postgres`

##### Create database
`createdb todo-app`

##### Show all databases
`psql -U postgres -l`

##### Access the todo-app database
`psql todo-app`

##### Run sql file against database
`psql -U <user> -d <database> -f <file>`

##### Show tables (while in psql)
`\dt`