CREATE TABLE crates (
    id SERIAL PRIMARY KEY,
    rustacean_id INTEGER NOT NULL REFERENCES rustaceans(id),
    code VARCHAR(60) NOT NULL,
    name VARCHAR(120) NOT NULL,
    version VARCHAR(60) NOT NULL,
    description TEXT,
    created_at TIMESTAMP DEFAULT NOW() NOT NULL
)
