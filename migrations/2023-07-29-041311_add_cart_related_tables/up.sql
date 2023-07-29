-- Your SQL goes here

CREATE TABLE IF NOT EXISTS cart (
    id uuid PRIMARY KEY DEFAULT public.uuid_generate_v4() NOT NULL
);


CREATE TABLE IF NOT EXISTS cart_item (
    id uuid PRIMARY KEY DEFAULT public.uuid_generate_v4() NOT NULL,
    cart_id uuid NOT NULL,
    product_id uuid NOT NULL,
    quantity INTEGER CHECK (quantity >= 0) NOT NULL DEFAULT 0,
    added_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT fk_cart_id
    FOREIGN KEY (cart_id) 
    REFERENCES cart (id),
    CONSTRAINT fk_product_id
    FOREIGN KEY (product_id)
    REFERENCES product (id)
)
