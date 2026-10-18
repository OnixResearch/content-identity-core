# Seaglass Cairn policy overlay.
#
# r[impl lifecycle-authoring.repository-traceability-profile]
# r[verify lifecycle-authoring.repository-traceability-profile.scenario.source-evidence]
#
# Refresh flow:
# 1. update generated/cairn-policy.base.json from the locked Cairn input;
# 2. run `cairn policy export --source cairn-policy/default.ncl --output cairn-policy/generated/cairn-policy.json`;
# 3. run the flake's `cairn-traceability` check.

let Base = import "generated/cairn-policy.base.json" in
let Contracts = import "contracts.ncl" in
let ValidatedWorkflowSchemas = Base.schemas | Array Contracts.WorkflowSchema in

let SeaglassTraceabilityProfile = {
  id = "seaglass-workspace",
  requirement_sources = [
    { root = "cairn/specs", source_kind = "accepted_specs", extensions = ["md"] },
  ],
  evidence_sources = [
    { root = "docking-layout", source_kind = "rust_source", extensions = ["rs", "ncl"] },
    { root = "tiered-cache", source_kind = "rust_source", extensions = ["rs", "ncl"] },
    { root = "seaglass-constants", source_kind = "rust_source", extensions = ["rs", "ncl"] },
    { root = "seaglass-kernel", source_kind = "rust_source", extensions = ["rs", "ncl"] },
    { root = "seaglass-ui-model", source_kind = "rust_source", extensions = ["rs", "ncl"] },
    { root = "seaglass-registry-model", source_kind = "rust_source", extensions = ["rs", "ncl"] },
    { root = "seaglass-content-identity", source_kind = "rust_source", extensions = ["rs", "ncl"] },
    { root = "seaglass-preserves", source_kind = "rust_source", extensions = ["rs", "ncl"] },
    { root = "seaglass-plugin-sdk", source_kind = "rust_source", extensions = ["rs", "ncl"] },
    { root = "seaglass-plugin-host", source_kind = "rust_source", extensions = ["rs", "ncl"] },
    { root = "seaglass-transform-registry", source_kind = "rust_source", extensions = ["rs", "ncl"] },
    { root = "seaglass-transform-polars", source_kind = "rust_source", extensions = ["rs", "ncl"] },
    { root = "seaglass-dataframe-model", source_kind = "rust_source", extensions = ["rs", "ncl"] },
    { root = "seaglass-types", source_kind = "rust_source", extensions = ["rs", "ncl"] },
    { root = "seaglass-transformations", source_kind = "rust_source", extensions = ["rs", "ncl"] },
    { root = "seaglass-data-io", source_kind = "rust_source", extensions = ["rs", "ncl"] },
    { root = "seaglass-pipeline-io", source_kind = "rust_source", extensions = ["rs", "ncl"] },
    { root = "seaglass-fileio", source_kind = "rust_source", extensions = ["rs", "ncl"] },
    { root = "seaglass-mapping-core", source_kind = "rust_source", extensions = ["rs", "ncl"] },
    { root = "seaglass-mapping-runtime", source_kind = "rust_source", extensions = ["rs", "ncl"] },
    { root = "seaglass-mapping", source_kind = "rust_source", extensions = ["rs", "ncl"] },
    { root = "seaglass-command-model", source_kind = "rust_source", extensions = ["rs", "ncl"] },
    { root = "seaglass-agent-session", source_kind = "rust_source", extensions = ["rs", "ncl"] },
    { root = "seaglass-display-model", source_kind = "rust_source", extensions = ["rs", "ncl"] },
    { root = "seaglass-validation-core", source_kind = "rust_source", extensions = ["rs", "ncl"] },
    { root = "seaglass-validation-document", source_kind = "rust_source", extensions = ["rs", "ncl"] },
    { root = "seaglass-validation-io", source_kind = "rust_source", extensions = ["rs", "ncl"] },
    { root = "seaglass-validation-viewmodel", source_kind = "rust_source", extensions = ["rs", "ncl"] },
    { root = "seaglass-cli", source_kind = "rust_source", extensions = ["rs", "ncl"] },
    { root = "seaglass-web", source_kind = "rust_source", extensions = ["rs", "ncl"] },
    { root = "seaglass-ui", source_kind = "rust_source", extensions = ["rs", "ncl"] },
    { root = "seaglass-sql", source_kind = "rust_source", extensions = ["rs", "ncl"] },
    { root = "seaglass-pipeline", source_kind = "rust_source", extensions = ["rs", "ncl"] },
    { root = "seaglass-viewmodel", source_kind = "rust_source", extensions = ["rs", "ncl"] },
    { root = "seaglass-platform-core", source_kind = "rust_source", extensions = ["rs", "ncl"] },
    { root = "seaglass-platform", source_kind = "rust_source", extensions = ["rs", "ncl"] },
    { root = "seaglass-validation", source_kind = "rust_source", extensions = ["rs", "ncl"] },
    { root = "seaglass-nickel-eval", source_kind = "rust_source", extensions = ["rs", "ncl"] },
    { root = "seaglass-config-nickel", source_kind = "rust_source", extensions = ["rs", "ncl"] },
    { root = "seaglass-nickel-codegen", source_kind = "rust_source", extensions = ["rs", "ncl"] },
    { root = "seaglass-nickel", source_kind = "rust_source", extensions = ["rs", "ncl"] },
    { root = "seaglass-services", source_kind = "rust_source", extensions = ["rs", "ncl"] },
    { root = "seaglass-sample-data-provider", source_kind = "rust_source", extensions = ["rs", "ncl"] },
    { root = "seaglass-document", source_kind = "rust_source", extensions = ["rs", "ncl"] },
    { root = "seaglass-test-harness", source_kind = "rust_source", extensions = ["rs", "ncl"] },
    { root = "seaglass-script-tools", source_kind = "rust_source", extensions = ["rs", "ncl"] },
    { root = "seaglass-daemon", source_kind = "rust_source", extensions = ["rs", "ncl"] },
    { root = "seaglass-h3-proxy", source_kind = "rust_source", extensions = ["rs", "ncl"] },
    { root = "seaglass-runtime", source_kind = "rust_source", extensions = ["rs", "ncl"] },
    { root = "seaglass-steel", source_kind = "rust_source", extensions = ["rs", "ncl"] },
    { root = "advisory-review", source_kind = "tool_source", extensions = ["rs", "ncl"] },
    { root = "cairn-policy", source_kind = "nickel_source", extensions = ["ncl"] },
    { root = "flake.nix", source_kind = "tool_source", extensions = ["nix"] },
    { root = "scripts", source_kind = "tool_source", extensions = ["rs", "ncl"] },
    { root = "test-support", source_kind = "rust_source", extensions = ["rs", "ncl"] },
    { root = "tests", source_kind = "rust_source", extensions = ["rs", "ncl"] },
  ],
  marker_verbs = ["impl", "verify", "depends", "related"],
} | Contracts.TraceabilityProfile in

(Base & {
  schemas | force = ValidatedWorkflowSchemas,
  traceability_policy | force = {
    default_profile = SeaglassTraceabilityProfile.id,
    profiles = Base.traceability_policy.profiles @ [SeaglassTraceabilityProfile],
  } | Contracts.TraceabilityPolicy,
})
