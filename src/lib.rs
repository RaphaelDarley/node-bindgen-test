mod opt;

use node_bindgen::core::val::JsObject;
use node_bindgen::derive::node_bindgen;
use serde;
use std::collections::VecDeque;
use surrealdb::engine::any::Any;
use surrealdb::opt::auth::Database;
use surrealdb::opt::auth::Namespace;
use surrealdb::opt::auth::Root;
use surrealdb::opt::auth::Scope;
use surrealdb::opt::PatchOp;
use surrealdb::opt::Resource;
use surrealdb::sql::Range;
use surrealdb::sql::Value;

use node_bindgen::core::val::JsNapiValue;

pub struct Surreal {
    db: surrealdb::Surreal<Any>,
}

#[node_bindgen]
impl Surreal {
    #[node_bindgen(constructor)]
    pub fn init() -> Surreal {
        Surreal {
            db: surrealdb::Surreal::init(),
        }
    }

    #[node_bindgen]
    pub async fn connect(&self, endpoint: String, opts: JsObject) -> Result<(), Error> {
        let connect = match from_value::<Option<opt::endpoint::Options>>(opts)? {
            Some(opts) => {
                let connect = match opts.strict {
                    #[cfg(any(feature = "kv-indxdb", feature = "kv-mem"))]
                    Some(true) => self.db.connect((endpoint, surrealdb::opt::Strict)),
                    _ => self.db.connect(endpoint),
                };
                match opts.capacity {
                    Some(capacity) => connect.with_capacity(capacity),
                    None => connect,
                }
            }
            None => self.db.connect(endpoint),
        };
        connect.await.map_err(Into::into)
    }
}
