use sqlx::PgPool;
use crate::model::Order;
use log::info;

pub(crate) async fn fetch_order_by_id_from_db(pool: &PgPool, order_uid: String) -> Result<Option<Order>, sqlx::Error> {
    info!("Fetching order by id from the database...");

    let order = sqlx::query_as!(
        Order,
        r#"
        SELECT
            order_uid,
            track_number,
            entry,
            delivery_name,
            delivery_phone,
            delivery_zip,
            delivery_city,
            delivery_address,
            delivery_region,
            delivery_email,
            payment_transaction,
            payment_request_id,
            payment_currency,
            payment_provider,
            payment_amount,
            payment_payment_dt,
            payment_bank,
            payment_delivery_cost,
            payment_goods_total,
            payment_custom_fee,
            items_chrt_id,
            items_track_number,
            items_price,
            items_rid,
            items_name,
            items_sale,
            items_size,
            items_total_price,
            items_nm_id,
            items_brand,
            items_status,
            locale,
            internal_signature,
            customer_id,
            delivery_service,
            shardkey,
            sm_id,
            date_created,
            oof_shard
        FROM orders
        WHERE order_uid = $1
        "#,
        order_uid
    )
        .fetch_optional(pool)
        .await?;

    Ok(order)
}