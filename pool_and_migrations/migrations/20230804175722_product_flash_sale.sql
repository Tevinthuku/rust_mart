-- Add migration script here

CREATE TABLE IF NOT EXISTS product_flash_sale (
    id uuid PRIMARY KEY DEFAULT public.uuid_generate_v4() NOT NULL,
    product_id uuid NOT NULL,
    price INTEGER CHECK (price >= 0),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    start_date TIMESTAMPTZ NOT NULL,
    end_date TIMESTAMPTZ NOT NULL,
    CONSTRAINT fk_product_id
    FOREIGN KEY (product_id)
    REFERENCES product (id)
)
