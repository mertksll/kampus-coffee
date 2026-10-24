use serde::Serialize;
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
static SEQUENCE: AtomicU32 = AtomicU32::new(0);
#[derive(Debug, Serialize)]
pub struct OrderCode { pub code: String, pub quantity: u32 }
#[derive(Debug, Serialize)]
#[serde(tag = "code")]
pub enum OrderError {
 #[serde(rename = "gecersiz_adet")]
 GecersizAdet { message: String },
}
pub fn create_order(quantity: u32) -> Result<OrderCode, OrderError> {
 if quantity == 0 || quantity > 9900 {
  return Err(OrderError::GecersizAdet {message:"Adet 1-9900 arası tam sayı olmalıdır.".into()});
 }
 let nanos=SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_nanos();
 let sequence=SEQUENCE.fetch_add(1,Ordering::Relaxed);
 Ok(OrderCode {code:format!("KMP-{:08X}",(nanos as u32).wrapping_add(sequence)),quantity})
}
#[cfg(test)]
mod tests {
 use super::*;
 #[test] fn valid_order() { let result=create_order(5).unwrap();assert_eq!(result.quantity,5);assert_eq!(result.code.len(),12);assert!(result.code.starts_with("KMP-"));assert!(result.code[4..].chars().all(|c| c.is_ascii_hexdigit())); }
 #[test] fn rejects_zero() {assert!(matches!(create_order(0),Err(OrderError::GecersizAdet{..})));}
 #[test] fn rejects_over_limit() {assert!(create_order(9901).is_err());}
}
