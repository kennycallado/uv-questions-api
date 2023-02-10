CREATE TABLE IF NOT EXISTS paper_answers (
  id SERIAL PRIMARY KEY,
  paper_id SERIAL NOT NULL,
  answer_id SERIAL NOT NULL,
  CONSTRAINT fk_pa_paper_id FOREIGN KEY(paper_id) REFERENCES papers (id) ON DELETE CASCADE,
  CONSTRAINT fk_pa_answer_id FOREIGN KEY(answer_id) REFERENCES answers (id) ON DELETE CASCADE
);

INSERT INTO paper_answers (paper_id, answer_id) VALUES
  (1, 1),
  (1, 2),
  (1, 3),
  (1, 4),
  (2, 5),
  (2, 6),
  (2, 7),
  (2, 8),
  (3, 9),
  (3, 10),
  (3, 11),
  (3, 12)
  ;
