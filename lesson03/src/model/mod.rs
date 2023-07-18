use std::collections::HashMap;
use std::fmt::{Display, Formatter, Write};
use std::ops::Deref;
use std::str::FromStr;

use rocket::serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use schemars::_private::NoSerialize;
use serde::{Deserializer, Serializer};
use serde_json::Value;
use sqlx::types::Uuid;

pub use req::*;
pub use res::*;
pub use ret::*;
pub use entity::*;
pub use table::*;

use crate::resp::Error;
use crate::resp::Error::ParamsError;

mod deser;
mod entity;
mod req;
mod res;
mod ret;
mod table;

