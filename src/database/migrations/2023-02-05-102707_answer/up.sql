CREATE TABLE IF NOT EXISTS answers (
  id SERIAL PRIMARY KEY,
  question_id SERIAL NOT NULL,
  answer VARCHAR(10) NOT NULL,
  CONSTRAINT fk_question FOREIGN KEY(question_id) REFERENCES questions(id) ON DELETE CASCADE
);

INSERT INTO answers (question_id, answer) VALUES
  (1, 'A'),
  (2, 'B'),
  (3, 'C'),
  (1, 'D'),
  (2, 'E'),
  (3, 'F'),
  (1, 'G'),
  (2, 'H'),
  (3, 'I'),
  (1, 'J'),
  (2, 'K'),
  (3, 'L')
  ;
