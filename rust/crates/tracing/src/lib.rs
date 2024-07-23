#[tracing::instrument]
pub fn sum(a: u32, b: u32) -> u32 {
    tracing::debug!("zzz");
    a + b
}
