
pub static EXPR_CACHE: Lazy<Mutex<LruCache<u64, (Expr, String)>>> = Lazy::new(|| {
    Mutex::new(LruCache::new(std::num::NonZeroUsize::new(1024).unwrap())) // Cache capacity of 1024
});
