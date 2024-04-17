use rocket::serde::{Deserialize, Serialize};
use rocket::log;
use rocket::serde::json::to_string;
use rocket::tokio;
use bambangshop::REQWEST_CLIENT;
use crate::model::notification::Notification;

impl Subscriber {
    #[tokio::main]
    pub async fn update(&self, payload: Notification){
        REQWEST_CLIENT
            .post(&self.url)
            .header("Content-type", "JSON")
            .body(to_string(&payload).unwrap())
            .send().await.ok();
        log::warn_!("Sent {} notification of: [{}] {}, to: {}",
            payload.status, payload.product_type, payload.product_title, self.url);
    }

    pub fn notify(&self, product_type:&str, status:&str, product:Product){
        let mut payload: Notification = Notification{
            product_title : product.clone().title,
            product_type: String::from(product_type),
            product_url: product.clone().get_url(),
            subscriber_name: String::from(""),
            status: String::from(status)
        };

        let subscribers: Vec<Subscriber> = SubscriberRepository::list_all(product_type);
        for subscriber in subscribers{
            payload.subscriber_name = subscriber.clone().name;
            let subscriber_clone = subscriber.clone();
            let payload_clone = payload.clone();
            thread::spawn(move|| subscriber_clone.update(payload_clone));
        }
    }
}