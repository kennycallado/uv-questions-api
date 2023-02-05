CREATE TABLE IF NOT EXISTS papers (
  id SERIAL PRIMARY KEY,
  user_id SERIAL NOT NULL,
  form_id SERIAL NOT NULL,
  CONSTRAINT fk_paper_user FOREIGN KEY (user_id) REFERENCES usuarios(id) ON DELETE CASCADE,
  CONSTRAINT fk_paper_form FOREIGN KEY (form_id) REFERENCES forms(id) ON DELETE CASCADE
);

INSERT INTO papers (user_id, form_id) VALUES
  (1, 1),
  (1, 2),
  (1, 3),
  (2, 1),
  (2, 2),
  (2, 3),
  (3, 1),
  (3, 2),
  (3, 3)
  ;
