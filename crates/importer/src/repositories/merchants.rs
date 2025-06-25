use crate::entities::merchants::Merchant;
use sqlx::PgPool;

pub async fn upsert_merchants(pool: &PgPool, merchants: &[Merchant]) -> Result<(), sqlx::Error> {
    if merchants.is_empty() {
        return Ok(());
    }

    let mut query_builder = sqlx::QueryBuilder::new(
        "INSERT INTO merchants (id, merchant_reference, email, live_on, disbursement_frequency, minimum_monthly_fee) "
    );

    query_builder.push_values(merchants, |mut b, merchant| {
        b.push_bind(merchant.id)
        .push_bind(&merchant.merchant_reference)
        .push_bind(&merchant.email)
        .push_bind(merchant.live_on)
        .push_bind(merchant.disbursement_frequency.to_string())
        .push_bind(merchant.minimum_monthly_fee);
    });

    query_builder.push(
        " ON CONFLICT (merchant_reference) DO UPDATE SET \
         email = EXCLUDED.email, \
         live_on = EXCLUDED.live_on, \
         disbursement_frequency = EXCLUDED.disbursement_frequency, \
         minimum_monthly_fee = EXCLUDED.minimum_monthly_fee"
    );

    let query = query_builder.build();
    query.execute(pool).await?;

    Ok(())
}

/*
    it would be nice if it's needed creating some kind of port and adapter pattern like this:

    use crate::entities::merchants::Merchant;
    use async_trait::async_trait;

    #[async_trait]
    pub trait MerchantRepository {
        async fn save_merchants(&self, merchants: &[Merchant]) -> anyhow::Result<()>;
        async fn exists(&self, id: &uuid::Uuid) -> anyhow::Result<bool>;
    }

    but i think is enough to use these "free functions" here for now.
    We just call these kind of functions with the module_path::{insert_merchant, get_existing_merchant_ids}
*/