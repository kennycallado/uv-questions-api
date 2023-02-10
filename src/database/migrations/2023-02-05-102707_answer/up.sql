CREATE TABLE IF NOT EXISTS answers (
  id SERIAL PRIMARY KEY,
  question_id SERIAL NOT NULL,
  answer VARCHAR(10) NOT NULL,
  CONSTRAINT fk_answer_question FOREIGN KEY(question_id) REFERENCES questions(id) ON DELETE CASCADE
);

INSERT INTO answers (question_id, answer) VALUES
  (1, '1'),
  (2, '4'),
  (3, '2'),
  (4, '4'),
  (1, '1'),
  (2, '7'),
  (3, '0'),
  (4, '0'),
  (1, '1'),
  (2, '6'),
  (3, '1'),
  (4, '2')
  ;
