CREATE TABLE IF NOT EXISTS form_answers (
  id SERIAL PRIMARY KEY,
  user_id SERIAL NOT NULL,
  form_id SERIAL NOT NULL,
  answer_id SERIAL NOT NULL,
  CONSTRAINT fk_user_id FOREIGN KEY (user_id)     REFERENCES usuarios(id) ON DELETE CASCADE,
  CONSTRAINT fk_form_id FOREIGN KEY (form_id)     REFERENCES forms(id) ON DELETE CASCADE,
  CONSTRAINT fk_answer_id FOREIGN KEY (answer_id) REFERENCES answers(id) ON DELETE CASCADE
);

INSERT INTO form_answers (user_id, form_id, answer_id) VALUES
  (1, 1, 1),
  (1, 1, 2),
  (1, 1, 3),
  (1, 2, 4),
  (1, 2, 5),
  (1, 2, 6),
  (2, 1, 7),
  (2, 1, 8),
  (2, 1, 9),
  (2, 2, 10),
  (2, 2, 11),
  (2, 2, 12)
  ;
