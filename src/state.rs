use crate::models::Order;
use std::sync::Mutex;

pub struct AppState {
    pub orders: Mutex<Vec<Order>>,
}

pub fn example_order() -> Order {
    Order {
        order_uid: "b563feb7b2b84b6test".to_string(),
        track_number: "WBILMTESTTRACK".to_string(),
        entry: "WBIL".to_string(),
        delivery: crate::models::Delivery {
            name: "Test Testov".to_string(),
            phone: "+9720000000".to_string(),
            zip: "2639809".to_string(),
            city: "Kiryat Mozkin".to_string(),
            address: "Ploshad Mira 15".to_string(),
            region: "Kraiot".to_string(),
            email: "test@gmail.com".to_string(),
        },
        payment: crate::models::Payment {
            transaction: "b563feb7b2b84b6test".to_string(),
            request_id: "".to_string(),
            currency: "USD".to_string(),
            provider: "wbpay".to_string(),
            amount: 1817,
            payment_dt: 1637907727,
            bank: "alpha".to_string(),
            delivery_cost: 1500,
            goods_total: 317,
            custom_fee: 0,
        },
        items: vec![crate::models::Item {
            chrt_id: 9934930,
            track_number: "WBILMTESTTRACK".to_string(),
            price: 453,
            rid: "ab4219087a764ae0btest".to_string(),
            name: "Mascaras".to_string(),
            sale: 30,
            size: "0".to_string(),
            total_price: 317,
            nm_id: 2389212,
            brand: "Vivienne Sabo".to_string(),
            status: 202,
        }],
        locale: "en".to_string(),
        internal_signature: "".to_string(),
        customer_id: "test".to_string(),
        delivery_service: "meest".to_string(),
        shardkey: "9".to_string(),
        sm_id: 99,
        date_created: "2021-11-26T06:22:19Z".to_string(),
        oof_shard: "1".to_string(),
    }
}
