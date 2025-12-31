use async_graphql::{
    Context, EmptyMutation, EmptySubscription, Enum, Error, Interface, Object, OutputType, Result,
    Schema, SimpleObject,
    connection::{Connection, Edge, query},
};

#[derive(SimpleObject)]
pub struct SalesForce {}

pub struct QueryRoot;

impl QueryRoot {}
