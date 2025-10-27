use crate::entities::merchants::Merchant;
use sqlx::PgPool;

pub async fn upsert_merchants(pool: &PgPool, merchants: &[Merchant]) -> Result<(), sqlx::Error> {
    if merchants.is_empty() {
        return Ok(());
    }

    let mut query_builder = sqlx::QueryBuilder::new(
        "INSERT INTO merchants (id, merchant_reference, live_on, disbursement_frequency, minimum_monthly_fee) "
    );

    query_builder.push_values(merchants, |mut b, merchant| {
        b.push_bind(merchant.id)
            .push_bind(&merchant.merchant_reference)
            .push_bind(merchant.live_on)
            .push_bind(merchant.disbursement_frequency.to_string())
            .push_bind(merchant.minimum_monthly_fee);
    });

    query_builder.push(
        " ON CONFLICT (merchant_reference) DO UPDATE SET \
         live_on = EXCLUDED.live_on, \
         disbursement_frequency = EXCLUDED.disbursement_frequency, \
         minimum_monthly_fee = EXCLUDED.minimum_monthly_fee"
    );

    let query = query_builder.build();
    query.execute(pool).await?;

    Ok(())
}
