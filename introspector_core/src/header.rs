use std::collections::BTreeSet;
use std::hash::{DefaultHasher, Hash, Hasher};
use proc_macro2; // Move this import to the top
use quote; // Needed for quote::ToTokens

use lru::LruCache; // New import
use once_cell::sync::Lazy; // New import
use std::sync::Mutex; // New import
