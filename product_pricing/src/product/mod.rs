use crate::price::Price;
use anyhow::bail;
use anyhow::Context;
use chrono::{DateTime, Utc};
use pool_and_migrations::pool::Pool;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FlashSale {
    price: Price,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct Product {
    id: uuid::Uuid,
    active_flash_sale: Option<FlashSale>,
    current_price: Option<Price>,
}

impl Product {
    pub(crate) async fn get(id: uuid::Uuid, pool: &Pool) -> anyhow::Result<Self> {
        let active_flash_sale = sqlx::query!(
            r#"
                SELECT price, start_date, end_date  FROM product_flash_sale WHERE product_id = $1 AND NOW() BETWEEN start_date AND end_date LIMIT 1
            "#, id
        ).fetch_optional(pool.get()).await?.map(|row| {
            Price::new(row.price).map(|price| {
                FlashSale {
                    price,
                    start: row.start_date,
                    end: row.end_date
                }
            })
        }).transpose()?;

        let current_price = sqlx::query!(
            r#"
            SELECT price FROM product_price WHERE product_id = $1 ORDER BY created_at DESC LIMIT 1
            "#,
            id
        )
        .fetch_optional(pool.get())
        .await?
        .and_then(|row| row.price.map(Price::new))
        .transpose()?;

        Ok(Self {
            id,
            active_flash_sale,
            current_price,
        })
    }

    pub(crate) fn price(&self) -> Option<Price> {
        match (&self.active_flash_sale, self.current_price) {
            (Some(active_flash_sale), _) => Some(active_flash_sale.price),
            (None, Some(current_price)) => Some(current_price),
            (_, _) => None,
        }
    }
    pub(crate) async fn new_flash_sale(
        &self,
        price: Price,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        pool: &Pool,
    ) -> anyhow::Result<Self> {
        if let Some(current_flash_sale) = &self.active_flash_sale {
            if start >= current_flash_sale.start && end <= current_flash_sale.end {
                bail!("There already exists a flash sale for the set time {current_flash_sale:?}")
            }
        };

        // might not be the best way to check this, but this will do for the demo;
        let existing_flash_sale_for_period = sqlx::query!(
            r#"
            SELECT price, start_date, end_date FROM product_flash_sale WHERE product_id = $1 AND start_date <= $2 AND end_date >= $3
            "#, self.id, start, end
        ).fetch_optional(pool.get()).await.context("Failed to fetch upcoming flash_sales")?.map(|row| {
            Price::new(row.price).map(|price| {
                FlashSale {
                    price,
                    start: row.start_date,
                    end: row.end_date
                }
            })
        }).transpose()?;

        if let Some(existing_flash_sale) = existing_flash_sale_for_period {
            bail!("There already exists a flash sale for the specified period {existing_flash_sale:?}")
        }

        sqlx::query!(
            r#"
            INSERT INTO product_flash_sale (product_id, price, start_date, end_date) VALUES ($1, $2, $3, $4)
            "#,
            self.id,
            *price.as_ref(),
            start,
            end
        ).execute(pool.get()).await.context("Failed to set new flash_sale")?;

        Product::get(self.id, pool).await
    }

    pub async fn increase_price(&self, new_price: Price, pool: &Pool) -> anyhow::Result<Self> {
        if let Some(current_price) = self.current_price {
            if current_price > new_price {
                bail!("Existing price {current_price} is greater than the new price {new_price}")
            }
        }

        sqlx::query!(
            r#"
            INSERT INTO product_price (product_id, price) VALUES ($1, $2)
        "#,
            self.id,
            *new_price.as_ref()
        )
        .execute(pool.get())
        .await
        .context("Failed to set new price")?;

        Product::get(self.id, pool).await
    }
}
