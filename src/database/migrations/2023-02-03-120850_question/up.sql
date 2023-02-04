CREATE TABLE IF NOT EXISTS questions (
  id SERIAL PRIMARY KEY,
  q_type VARCHAR NOT NULL,
  question VARCHAR NOT NULL
);

INSERT INTO questions (q_type, question) VALUES
  ('range', '¿Bla bla bla bla bla bla?'),
  ('range', '¿Bla bla bla bla bla?'),
  ('range', '¿Bla bla bla bla?'),
  ('range', '¿Bla bla bla?'),
  ('range', '¿Bla bla?');
