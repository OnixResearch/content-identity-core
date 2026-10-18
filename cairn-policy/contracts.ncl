# Typed contracts for the Seaglass-owned Cairn traceability profile.
# r[impl lifecycle-authoring.cairn-policy-compatibility]

let NonEmptyString = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.length value > 0
) in

let NonEmptyArray = std.contract.from_predicate (fun value =>
  std.is_array value && std.array.length value > 0
) in

let SafeRelativePath = std.contract.from_predicate (fun value =>
  std.is_string value
  && std.string.length value > 0
  && std.string.is_match "^[A-Za-z0-9][A-Za-z0-9._/-]*$" value
  && !std.string.is_match "(^|/)[.][.](/|$)" value
  && !std.string.is_match "//" value
  && !std.string.is_match "/$" value
) in

let TraceabilityEvidenceSourceKind = std.contract.from_predicate (fun value =>
  std.is_string value
  && (value == "rust_source" || value == "nickel_source" || value == "tool_source")
) in

let TraceabilityRequirementSourceKind = std.contract.from_predicate (fun value =>
  std.is_string value
  && (value == "accepted_specs" || value == "requirements_doc")
) in

let TraceabilityExtension = std.contract.from_predicate (fun value =>
  std.is_string value
  && std.string.length value > 0
  && std.string.is_match "^[A-Za-z0-9_-]+$" value
) in

let TraceabilityMarkerVerb = std.contract.from_predicate (fun value =>
  std.is_string value
  && (value == "impl" || value == "verify" || value == "depends" || value == "related")
) in

let WorkflowPathKind = std.contract.from_predicate (fun value =>
  std.is_string value
  && (value == "file" || value == "spec_directory")
) in

{
  TraceabilityRequirementSource = {
    root | SafeRelativePath,
    source_kind | TraceabilityRequirementSourceKind,
    extensions | NonEmptyArray | Array TraceabilityExtension,
  },

  TraceabilityEvidenceSource = {
    root | SafeRelativePath,
    source_kind | TraceabilityEvidenceSourceKind,
    extensions | NonEmptyArray | Array TraceabilityExtension,
  },

  TraceabilityProfile = {
    id | NonEmptyString,
    requirement_sources | NonEmptyArray | Array TraceabilityRequirementSource,
    evidence_sources | NonEmptyArray | Array TraceabilityEvidenceSource,
    marker_verbs | NonEmptyArray | Array TraceabilityMarkerVerb,
  },

  TraceabilityPolicy = {
    default_profile | NonEmptyString,
    profiles | NonEmptyArray | Array TraceabilityProfile,
  },

  WorkflowArtifact = {
    output_path | SafeRelativePath,
    path_kind | WorkflowPathKind,
    ..
  },

  WorkflowSchema = {
    id | NonEmptyString,
    artifacts | NonEmptyArray | Array WorkflowArtifact,
    ..
  },
}
