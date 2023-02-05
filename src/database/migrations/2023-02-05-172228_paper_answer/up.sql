CREATE TABLE IF NOT EXISTS paper_answers (
  id SERIAL PRIMARY KEY,
  paper_id INTEGER NOT NULL,
  answer_id INTEGER NOT NULL,
  CONSTRAINT fk_pa_paper_id FOREIGN KEY(paper_id) REFERENCES papers (id) ON DELETE CASCADE,
  CONSTRAINT fk_pa_answer_id FOREIGN KEY(answer_id) REFERENCES answers (id) ON DELETE CASCADE
);

INSERT INTO paper_answers (paper_id, answer_id) VALUES
  (1, 1),
  (1, 2),
  (1, 3),
  (2, 4),
  (2, 5),
  (2, 6),
  (3, 7),
  (3, 8),
  (3, 9)
  ;
