CREATE TABLE IF NOT EXISTS pessoas (
    id UUID PRIMARY KEY,
    apelido TEXT UNIQUE NOT NULL,
    nome TEXT NOT NULL,
    nascimento TEXT NOT NULL,
    stack TEXT[]
);