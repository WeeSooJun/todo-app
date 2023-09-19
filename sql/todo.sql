DROP TABLE IF EXISTS todo;
CREATE TABLE todo (
    id SERIAL,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    end_date TIMESTAMP,
    todo_state TEXT NOT NULL REFERENCES valid_todo_state(valid_state) ON UPDATE CASCADE
);