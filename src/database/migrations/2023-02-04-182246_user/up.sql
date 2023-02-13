CREATE TABLE IF NOT EXISTS usuarios (
  id SERIAL PRIMARY KEY,
  email VARCHAR NOT NULL UNIQUE,
  token VARCHAR NOT NULL UNIQUE,
  fmc_token VARCHAR
);

INSERT INTO usuarios (email, token) VALUES
  ('admin@admin.es', 'admin'),
  ('user1@test.es', '1234567890'),
  ('user2@test.es', '123456789a'),
  ('user3@test.es', '123456789b'),
  ('user4@test.es', '123456789c')
  ;
