CREATE TABLE IF NOT EXISTS usuarios (
  id SERIAL PRIMARY KEY,
  email VARCHAR NOT NULL UNIQUE,
  token VARCHAR NOT NULL UNIQUE
);

INSERT INTO usuarios (email, token) VALUES
  ('bla@asflkjd.es', '1234567890'),
  ('lsdfj@pwoei.es', '1iasd0f929'),
  ('ldsfj@pwoei.es', '1iadf920s9'),
  ('sdflj@pwoei.es', '1iasdf9209'),
  ('jlsdf@pwoei.es', '1i9adfs209')

