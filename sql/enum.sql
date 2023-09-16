DROP TABLE IF EXISTS valid_todo_state;
CREATE TABLE valid_todo_state (
    valid_state TEXT PRIMARY KEY
);

INSERT INTO valid_todo_state(valid_state) VALUES
('ON_HOLD'),
('IN_PROGRESS'),
('DONE');