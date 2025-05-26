#![cfg(feature = "redis")]
//! Submodule implementing the `ToRedisArgs` trait for UUIDs.

impl redis::ToRedisArgs for crate::Uuid {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite,
    {
        out.write_arg(self.0.as_bytes());
    }
}
