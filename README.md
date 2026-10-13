# Da Vinci CRD hook payload — Rust

Small library and CLI that sanity-check **Clinical Decision Support (CRD)** hook request JSON before it reaches your CDS service: **`hook`** name (e.g. **`order-sign`**) and a **`prefetch`** object defining requested FHIR queries.

Synthetic fixtures only. Not ONC certification, not a substitute for Da Vinci CRD conformance tools or Inferno, and not legal/clinical advice.

Maintained by [Klarity Labs](https://klaritylabs.ai/) as open reference code for U.S. healthcare FHIR interoperability work.

**Exhibit 17** — [FHIR exhibit catalog](https://github.com/klaritylabs-us/klarity-fhir-exhibits) · [klaritylabs-us](https://github.com/klaritylabs-us)


## How this relates to other tools

| Tool class | Role |
|------------|------|
| **HL7 FHIR Validator**, **HAPI**, **Firely** | Full validation, servers, SDKs, IG packages |
| **Inferno**, **Touchstone**, vendor cert suites | **Conformance** against test plans |
| **This exhibit** | **Pre-flight construction** on synthetic fixtures—catch shape mistakes before wire traffic |

Conformance tools test whether your **API behaves**; exhibits focus on whether your **payloads and pipes are built** to survive real partners.

## Specifications cited

- [Da Vinci CRD](http://hl7.org/fhir/us/davinci-crd/)

## What it checks

- Warning when **`hook`** is missing or not **`order-sign`** (common CRD entry point in examples)
- **`prefetch`** must be present (CRD requests define prefetch keys for the EHR to resolve)

| Problem | What breaks |
|--------|-------------|
| **No prefetch block** | EHR cannot hydrate cards; empty CDS response |
| **Unknown hook name** | CRD service ignores the invocation |
| **Prefetch keys without matching queries** | Cards render with missing data (runtime issue; shape caught here) |

## Run it

Needs [Rust](https://www.rust-lang.org/tools/install).

```bash
cargo test
cargo run --bin validate-crd -- examples/valid-crd.json
cargo run --bin validate-crd -- examples/invalid-crd.json
```

Exit `0` if no errors, `1` if validation failed, `2` for bad usage or JSON.

## Sample output

One command:

```bash
cargo run --bin validate-crd -- examples/valid-crd.json
```

**Pass** (`examples/valid-crd.json`) — exit code `0`:

```json
[]
```

**Fail** (`examples/invalid-crd.json`) — exit code `1` (issues array; more entries may appear):

```json
[
  {
    "severity": "error",
    "path": "prefetch",
    "detail": "CRD request should define prefetch keys",
    "rule_id": "crd-prefetch"
  }
]
```


## Layout

- `src/validate.rs` — rules
- `examples/` — valid and broken CRD hook payloads for tests and demos

## Inferno vs this repo

CRD suites exercise **hook execution** in vendor EHR sandboxes. This repo is **hook request JSON** — the POST body your middleware forwards before the CDS app runs.

## License

MIT — see [LICENSE](./LICENSE).

**Need this on your payloads or API?** 20-minute FHIR Scan — sk@klaritylabs.ai · [klaritylabs.ai](https://klaritylabs.ai/)

## Contact

sk@klaritylabs.ai · [klaritylabs.ai](https://klaritylabs.ai/)
