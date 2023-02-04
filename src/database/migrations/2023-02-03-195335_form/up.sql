CREATE TABLE IF NOT EXISTS forms (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  desciption VARCHAR NOT NULL
);

INSERT INTO forms (title, desciption) VALUES
  ('titulo 1', 'Questionnaire 1 of 9'),
  ('titulo 2', 'Questionnaire 2 of 9'),
  ('titulo 3', 'Questionnaire 3 of 9'),
  ('titulo 4', 'Questionnaire 4 of 9'),
  ('titulo 5', 'Questionnaire 5 of 9'),
  ('titulo 6', 'Questionnaire 6 of 9'),
  ('titulo 7', 'Questionnaire 7 of 9'),
  ('titulo 8', 'Questionnaire 8 of 9'),
  ('titulo 9', 'Questionnaire 9 of 9');
