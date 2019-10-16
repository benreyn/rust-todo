-- SQLite does not support easily removing a column from
-- a table, so we use a hack to drop and recreate the table.
-- This is not something we would want to do if this was
-- anything more than a toy app.

CREATE TEMPORARY TABLE tasks_backup(id,title);
INSERT INTO tasks_backup SELECT id,title FROM tasks;
DROP TABLE tasks;
CREATE TABLE tasks (
       id INTEGER NOT NULL,
       title TEXT NOT NULL,
       PRIMARY KEY (id)
);
INSERT INTO tasks SELECT id,title FROM tasks_backup;
DROP TABLE tasks_backup;
