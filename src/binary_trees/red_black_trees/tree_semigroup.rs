pub trait TreeSemigroup<K, V> {
    fn leaf_val() -> Self;
    fn op(key: &K, value: &V, left: &Self, right: &Self) -> Self;
}
