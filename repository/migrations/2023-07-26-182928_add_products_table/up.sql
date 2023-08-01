-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp" WITH SCHEMA "public";

CREATE TABLE IF NOT EXISTS product (
    id uuid PRIMARY KEY DEFAULT public.uuid_generate_v4() NOT NULL,
    name text NOT NULL,
    description text NOT NULL,
    quantity_on_hand INTEGER CHECK (quantity_on_hand >= 0) NOT NULL DEFAULT 0,
    quantity_available INTEGER CHECK (quantity_available >= 0) NOT NULL DEFAULT 0,
    price INTEGER CHECK (price >= 0),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
)
