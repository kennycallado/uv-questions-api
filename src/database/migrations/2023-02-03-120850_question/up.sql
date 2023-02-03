CREATE TABLE IF NOT EXISTS questions (
  id SERIAL PRIMARY KEY,
  q_type VARCHAR NOT NULL,
  question VARCHAR NOT NULL,
  answer VARCHAR
);

INSERT INTO questions (q_type, question, answer) VALUES
  ('range', '¿Bla bla bla bla bla bla?', NULL),
  ('range', '¿Bla bla bla bla bla?', NULL),
  ('range', '¿Bla bla bla bla?', NULL),
  ('range', '¿Bla bla bla?', NULL),
  ('range', '¿Bla bla?', NULL);
