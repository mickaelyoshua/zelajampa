CREATE EXTENSION IF NOT EXISTS postgis;

CREATE TABLE reports (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    category VARCHAR(50) NOT NULL,
    location GEOMETRY(Point, 4326) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Índice GIST para acelerar buscas espaciais/geohashing
CREATE INDEX reports_location_idx ON reports USING GIST (location);
