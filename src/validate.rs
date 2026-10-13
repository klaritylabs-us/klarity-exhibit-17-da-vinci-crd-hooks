use serde_json::Value;
use crate::outcome::{ValidationIssue, ValidationReport};

pub fn validate_crd_json(json: &str) -> Result<ValidationReport, String> {
    let v: Value = serde_json::from_str(json).map_err(|e| format!("JSON: {e}"))?;
    let mut r = ValidationReport::ok();
    if v.get("hook").and_then(|x| x.as_str()) != Some("order-sign") && v.get("hook").is_none() {
        r.push(ValidationIssue::warning("hook", "CRD hooks include hook name (e.g. order-sign)", "crd-hook"));
    }
    if v.get("prefetch").is_none() {
        r.push(ValidationIssue::error("prefetch", "CRD request should define prefetch keys", "crd-prefetch"));
    }
    Ok(r)
}
