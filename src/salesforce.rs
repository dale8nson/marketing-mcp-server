use std::any::Any;

use actix_web::web::Query;
use async_graphql::{
    ComplexObject
// #[derive(Default, InputObject)]
// #[graphql(complex)]
// pub struct SalesForce {
//     url: String,
// }

// #[ComplexObject]
// impl SalesForce {}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Salesforce {

}
