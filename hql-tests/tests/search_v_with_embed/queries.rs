// DEFAULT CODE
// use helix_db::helix_engine::traversal_core::config::Config;

// pub fn config() -> Option<Config> {
//     None
// }

use bumpalo::Bump;
use chrono::{DateTime, Utc};
use heed3::RoTxn;
use helix_db::{
    embed, embed_async, field_addition_from_old_field, field_addition_from_value, field_type_cast,
    helix_engine::{
        reranker::{
            RerankAdapter,
            fusion::{DistanceMethod, MMRReranker, RRFReranker},
        },
        traversal_core::{
            config::{Config, GraphConfig, VectorConfig},
            ops::{
                bm25::search_bm25::SearchBM25Adapter,
                g::G,
                in_::{in_::InAdapter, in_e::InEdgesAdapter, to_n::ToNAdapter, to_v::ToVAdapter},
                out::{
                    from_n::FromNAdapter, from_v::FromVAdapter, out::OutAdapter,
                    out_e::OutEdgesAdapter,
                },
                source::{
                    add_e::AddEAdapter, add_n::AddNAdapter, e_from_id::EFromIdAdapter,
                    e_from_type::EFromTypeAdapter, n_from_id::NFromIdAdapter,
                    n_from_index::NFromIndexAdapter, n_from_type::NFromTypeAdapter,
                    v_from_id::VFromIdAdapter, v_from_type::VFromTypeAdapter,
                },
                util::{
                    aggregate::AggregateAdapter,
                    count::CountAdapter,
                    dedup::DedupAdapter,
                    drop::Drop,
                    exist::Exist,
                    filter_mut::FilterMut,
                    filter_ref::FilterRefAdapter,
                    group_by::GroupByAdapter,
                    map::MapAdapter,
                    order::OrderByAdapter,
                    paths::{PathAlgorithm, ShortestPathAdapter},
                    range::RangeAdapter,
                    update::UpdateAdapter,
                    upsert::UpsertAdapter,
                },
                vectors::{
                    brute_force_search::BruteForceSearchVAdapter, insert::InsertVAdapter,
                    search::SearchVAdapter,
                },
            },
            traversal_value::TraversalValue,
        },
        types::{GraphError, SecondaryIndex},
        vector_core::vector::HVector,
    },
    helix_gateway::{
        embedding_providers::{EmbeddingModel, get_embedding_model},
        mcp::mcp::{MCPHandler, MCPHandlerSubmission, MCPToolInput},
        router::router::{HandlerInput, IoContFn},
    },
    node_matches, props,
    protocol::{
        format::Format,
        response::Response,
        value::{
            Value,
            casting::{CastType, cast},
        },
    },
    utils::{
        id::{ID, uuid_str},
        items::{Edge, Node},
        properties::ImmutablePropertiesMap,
    },
};
use helix_macros::{handler, mcp_handler, migration, tool_call};
use sonic_rs::{Deserialize, Serialize, json};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Instant;

// Re-export scalar types for generated code
type I8 = i8;
type I16 = i16;
type I32 = i32;
type I64 = i64;
type U8 = u8;
type U16 = u16;
type U32 = u32;
type U64 = u64;
type U128 = u128;
type F32 = f32;
type F64 = f64;

pub fn config() -> Option<Config> {
    return Some(Config {
        vector_config: Some(VectorConfig {
            m: Some(16),
            ef_construction: Some(128),
            ef_search: Some(768),
        }),
        graph_config: Some(GraphConfig {
            secondary_indices: None,
        }),
        db_max_size_gb: Some(10),
        mcp: Some(true),
        bm25: Some(true),
        schema: Some(
            r#"{
  "schema": {
    "nodes": [
      {
        "name": "File19",
        "properties": {
          "id": "ID",
          "age": "I32",
          "label": "String",
          "name": "String"
        }
      }
    ],
    "vectors": [
      {
        "name": "File19Vec",
        "properties": {
          "data": "Array(F64)",
          "score": "F64",
          "id": "ID",
          "name": "String",
          "age": "I32",
          "label": "String"
        }
      }
    ],
    "edges": [
      {
        "name": "Follows",
        "from": "File19",
        "to": "File19Vec",
        "properties": {}
      }
    ]
  },
  "queries": [
    {
      "name": "file19",
      "parameters": {},
      "returns": [
        "vec"
      ]
    }
  ]
}"#
            .to_string(),
        ),
        embedding_model: Some("text-embedding-ada-002".to_string()),
        graphvis_node_label: None,
    });
}
pub struct File19 {
    pub name: String,
    pub age: i32,
}

pub struct Follows {
    pub from: File19,
    pub to: File19Vec,
}

pub struct File19Vec {
    pub name: String,
    pub age: i32,
}

#[derive(Serialize)]
pub struct File19VecReturnType<'a> {
    pub id: &'a str,
    pub label: &'a str,
    pub data: &'a [f64],
    pub score: f64,
    pub age: Option<&'a Value>,
    pub name: Option<&'a Value>,
}

#[handler]
pub fn file19(input: HandlerInput) -> Result<Response, GraphError> {
    let db = Arc::clone(&input.graph.storage);
    Err(IoContFn::create_err(
        move |__internal_cont_tx, __internal_ret_chan| {
            Box::pin(async move {
                let __internal_embed_data_0 = embed_async!(db, &hello world);
                __internal_cont_tx
                    .send_async((
                        __internal_ret_chan,
                        Box::new(move || {
                            let __internal_embed_data_0: Vec<f64> = __internal_embed_data_0?;
                            let arena = Bump::new();
                            let txn = db.graph_env.read_txn().map_err(|e| {
                                GraphError::New(format!(
                                    "Failed to start read transaction: {:?}",
                                    e
                                ))
                            })?;
                            let vec = G::new(&db, &txn, &arena)
                                .n_from_type("File19")
                                .out_vec("Follows", false)
                                .brute_force_search_v(&__internal_embed_data_0, 10)
                                .collect::<Result<Vec<_>, _>>()?;
                            let response = json!({
                                "vec": vec.iter().map(|vec| File19VecReturnType {
                                    id: uuid_str(vec.id(), &arena),
                                    label: vec.label(),
                                    data: vec.data(),
                                    score: vec.score(),
                                    age: vec.get_property("age"),
                                    name: vec.get_property("name"),
                                }).collect::<Vec<_>>()
                            });
                            txn.commit().map_err(|e| {
                                GraphError::New(format!("Failed to commit transaction: {:?}", e))
                            })?;
                            Ok(input.request.out_fmt.create_response(&response))
                        }),
                    ))
                    .await
                    .expect("Cont Channel should be alive")
            })
        },
    ))
}
