#![allow(non_camel_case_types)]
use std::sync::{Arc, RwLock};
use std::task::Poll;

use crate::salesforce::SalesForce;
use crate::{analytics::Analytics, appstate::AppState};
use actix_web::{rt, Error, FromRequest, HttpRequest, HttpResponse, Responder, get, web};
use async_graphql::futures_util::Stream;
use async_graphql::http::GraphiQLSource;
use async_graphql::types::Json;
use async_graphql::*;
use async_graphql_actix_web::{GraphQLBatchRequest, GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use base64::prelude::*;
use graphql_client::Response;
use serde::{Deserialize, Serialize};

#[derive(serde::Deserialize, Debug, Clone)]
pub struct OAuth {
    pub access_token: String,
    pub signature: String,
    pub scope: String,
    pub instance_url: String,
    pub id: String,
    pub token_type: String,
    pub issued_at: String,
    pub api_instance_url: String,
}

pub async fn graphiql_index() -> Result<HttpResponse, Box<dyn std::error::Error>> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(GraphiQLSource::build().endpoint("/").finish()))
}

pub async fn graphql_ws(
    schema: web::Data<Schema<Query, EmptyMutation, EmptySubscription>>,
    req: HttpRequest,
    payload: web::Payload,
) -> Result<HttpResponse, Error> {
let (res, mut session, stream) = actix_ws::handle(&req, stream)?;

let mut stream = stream
    .aggregate_continuations()
    // aggregate continuation frames up to 1MiB
    .max_continuation_size(2_usize.pow(20));

// start task but don't wait for it
rt::spawn(async move {
    // receive messages from websocket
    while let Some(msg) = stream.next().await {
      match msg {
        _ => {}
      }
}

async fn echo(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let (res, mut session, stream) = actix_ws::handle(&req, stream)?;

    let mut stream = stream
        .aggregate_continuations()
        // aggregate continuation frames up to 1MiB
        .max_continuation_size(2_usize.pow(20));

    // start task but don't wait for it
    rt::spawn(async move {
        // receive messages from websocket
        while let Some(msg) = stream.next().await {
            match msg {
                Ok(AggregatedMessage::Text(text)) => {
                    // echo text message
                    session.text(text).await.unwrap();
                }

                Ok(AggregatedMessage::Binary(bin)) => {
                    // echo binary message
                    session.binary(bin).await.unwrap();
                }

                Ok(AggregatedMessage::Ping(msg)) => {
                    // respond to PING frame with PONG frame
                    session.pong(&msg).await.unwrap();
                }

                _ => {}
            }
        }
    });

    // respond immediately with response connected to WS session
    Ok(res)
}

// type __Schema {
//   description: String
//   types: [__Type!]!
//   queryType: __Type!
//   mutationType: __Type
//   subscriptionType: __Type
//   directives: [__Directive!]!
// }

// type __Type {
//   kind: __TypeKind!
//   name: String
//   description: String
//   # may be non-null for custom SCALAR, otherwise null.
//   specifiedByURL: String
//   # must be non-null for OBJECT and INTERFACE, otherwise null.
//   fields(includeDeprecated: Boolean! = false): [__Field!]
//   # must be non-null for OBJECT and INTERFACE, otherwise null.
//   interfaces: [__Type!]
//   # must be non-null for INTERFACE and UNION, otherwise null.
//   possibleTypes: [__Type!]
//   # must be non-null for ENUM, otherwise null.
//   enumValues(includeDeprecated: Boolean! = false): [__EnumValue!]
//   # must be non-null for INPUT_OBJECT, otherwise null.
//   inputFields(includeDeprecated: Boolean! = false): [__InputValue!]
//   # must be non-null for NON_NULL and LIST, otherwise null.
//   ofType: __Type
//   # must be non-null for INPUT_OBJECT, otherwise null.
//   isOneOf: Boolean
// }

// enum __TypeKind {
//   SCALAR
//   OBJECT
//   INTERFACE
//   UNION
//   ENUM
//   INPUT_OBJECT
//   LIST
//   NON_NULL
// }

// type __Field {
//   name: String!
//   description: String
//   args(includeDeprecated: Boolean! = false): [__InputValue!]!
//   type: __Type!
//   isDeprecated: Boolean!
//   deprecationReason: String
// }

// type __InputValue {
//   name: String!
//   description: String
//   type: __Type!
//   defaultValue: String
//   isDeprecated: Boolean!
//   deprecationReason: String
// }

// type __EnumValue {
//   name: String!
//   description: String
//   isDeprecated: Boolean!
//   deprecationReason: String
// }

// type __Directive {
//   name: String!
//   description: String
//   isRepeatable: Boolean!
//   locations: [__DirectiveLocation!]!
//   args(includeDeprecated: Boolean! = false): [__InputValue!]!
// }

// enum __DirectiveLocation {
//   QUERY
//   MUTATION
//   SUBSCRIPTION
//   FIELD
//   FRAGMENT_DEFINITION
//   FRAGMENT_SPREAD
//   INLINE_FRAGMENT
//   VARIABLE_DEFINITION
//   SCHEMA
//   SCALAR
//   OBJECT
//   FIELD_DEFINITION
//   ARGUMENT_DEFINITION
//   INTERFACE
//   UNION
//   ENUM
//   ENUM_VALUE
//   INPUT_OBJECT
//   INPUT_FIELD_DEFINITION
// }

#[derive(Serialize, Deserialize, Clone, Debug)]
struct __Schema {
    description: String,
    types: Vec<Type>,
    queryType: Type,
    mutationType: Type,
    subscriptionType: Type,
    directives: Vec<Directive>,
}

#[derive(Serialize, Deserialize, Clone, Debug, SimpleObject)]
#[graphql(complex)]
struct Type {
    kind: TypeKind,
    name: String,
    description: String,
    //   # may be non-null for custom SCALAR, otherwise null.
    specifiedByURL: String,
    //   # must be non-null for OBJECT and INTERFACE, otherwise null.
    //   # must be non-null for OBJECT and INTERFACE, otherwise null.
    interfaces: Vec<Type>, //   # must be non-null for INTERFACE and UNION, otherwise null.
    possibleTypes: Vec<Type>, //   # must be non-null for ENUM, otherwise null.
    _fields: Vec<Field>
                           //   # must be non-null for INPUT_OBJECT, otherwise null.
                           //   inputFields(includeDeprecated: Boolean! = false): [__InputValue!]
                           //   # must be non-null for NON_NULL and LIST, otherwise null.
                           //   ofType: __Type
                           //   # must be non-null for INPUT_OBJECT, otherwise null.
                           //   isOneOf: Boolean
                           // }
}

#[ComplexObject]
impl Type {
    pub async fn fields(
        &self,
        ctx: &Context<'_>,
        #[graphql(default = false)] includeDeprecated: bool,
    ) -> Vec<Field> {
        let data = ctx.data::<AppState>().unwrap();
        let client = data.client;
        let url = data.salesforce_graphql_endpoint;

        let res = client.post(url).

        // self.fields.filter(|field| if includeDeprecated { true } else { !field.is_deprecated })
    }

    pub async fn enumValues(&self, ctx: &Context<'_>) -> Vec<EnumValue> {

    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Directive {}

#[derive(Enum, Serialize, Deserialize, Clone, Debug, Copy, PartialEq, Eq)]
enum TypeKind {
    SCALAR,
    OBJECT,
    INTERFACE,
    UNION,
    ENUM,
    INPUT_OBJECT,
    LIST,
    NON_NULL,
}

#[derive(Serialize, Deserialize, Clone, Debug, SimpleObject)]
#[graphql(complex)]
struct Field {
    pub name: String,
    pub isDeprecated: bool,

    //   type: __Type!
    //   isDeprecated: Boolean!
    //   deprecationReason: String
}

#[ComplexObject]
impl Field {
  fn args(&self, #[graphql(default = false)] includeDeprecated: bool) -> Json<Vec<InputValue>> {

  }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct EnumValue {}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct InputValue {
  name: String,
    description: String,
    type_:Type,
    defaultValue: String,
    isDeprecated: bool,
    deprecationReason: String
}

#[derive(Default, SimpleObject)]
#[graphql(complex)]
pub struct Query {
    analytics: Analytics,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct SalesforceType {
    name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct SalesforceSchema {
    types: Vec<SalesforceType>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct SalesforceData {
    __schema: SalesforceSchema,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct SalesforceQuery {
    data: SalesforceData,
}

// #[derive(Clone, Debug, Deserialize, Serialize)]
// struct Field {
//     name: String,
// }

#[ComplexObject]
impl Query {
    pub async fn salesforce<'ctx>(&self, ctx: &Context<'ctx>) -> Json<SalesforceQuery> {
        println!("ctx: {:#?}", ctx.data::<AppState>());
        let data = ctx.data::<AppState>().unwrap();
        let AppState {
            salesforce_initial_access_token,
            salesforce_consumer_key,
            salesforce_consumer_secret,
            salesforce_login_url,
            salesforce_graphql_endpoint,
            client,
            salesforce_access_token
        } = data;
        let mut token_request_url = String::from(&*salesforce_login_url.clone());
        token_request_url.push_str("/services/oauth2/token");
        let id = salesforce_consumer_key.clone();
        let secret = salesforce_consumer_secret.clone();
        // let mut b64 = String::default();
        // BASE64_STANDARD.encode_string(format!("{}:{}", id, secret), &mut b64);


        let mut url = String::from(&*salesforce_login_url.clone());
        url.push_str(&*salesforce_graphql_endpoint);
        let req = client
            .post(url)
            // .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("X-Chatter-Entity-Encoding", "false")
            .header("Authorization", format!("Bearer {access_token}").as_str())
            .body("{\"query\": \"query { __schema { types { name } } }\"}")
            .build()
            .unwrap();
        let res = client.execute(req).await;
        let res = res;
        println!("{:#?}", res);
        let res = res.unwrap();
        let json = res.json::<SalesforceQuery>().await.unwrap();
        println!("{:#?}", json);
        Json(json)

        // json.parse().unwrap()
    }
}

#[derive(Default)]
pub struct Mutation {
    id: u64,
}

#[Object]
impl Mutation {
    pub async fn id(&self) -> u64 {
        self.id
    }
}

#[derive(Default)]
pub struct QuerySubscription;

#[Subscription]
impl QuerySubscription {
    async fn stream(&self) -> impl Stream<Item = u8> {
        Streamer::default()
    }
}

#[derive(Default)]
struct Streamer;

impl Stream for Streamer {
    type Item = u8;
    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        Poll::Ready(Some(50u8))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (1, Some(512))
    }
}
