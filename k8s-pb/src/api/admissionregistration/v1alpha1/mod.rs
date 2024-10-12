// This file is @generated by prost-build.
/// AuditAnnotation describes how to produce an audit annotation for an API request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuditAnnotation {
    /// key specifies the audit annotation key. The audit annotation keys of
    /// a ValidatingAdmissionPolicy must be unique. The key must be a qualified
    /// name ([A-Za-z0-9][-A-Za-z0-9_.]*) no more than 63 bytes in length.
    ///
    /// The key is combined with the resource name of the
    /// ValidatingAdmissionPolicy to construct an audit annotation key:
    /// "{ValidatingAdmissionPolicy name}/{key}".
    ///
    /// If an admission webhook uses the same resource name as this ValidatingAdmissionPolicy
    /// and the same audit annotation key, the annotation key will be identical.
    /// In this case, the first annotation written with the key will be included
    /// in the audit event and all subsequent annotations with the same key
    /// will be discarded.
    ///
    /// Required.
    #[prost(string, optional, tag = "1")]
    pub key: ::core::option::Option<::prost::alloc::string::String>,
    /// valueExpression represents the expression which is evaluated by CEL to
    /// produce an audit annotation value. The expression must evaluate to either
    /// a string or null value. If the expression evaluates to a string, the
    /// audit annotation is included with the string value. If the expression
    /// evaluates to null or empty string the audit annotation will be omitted.
    /// The valueExpression may be no longer than 5kb in length.
    /// If the result of the valueExpression is more than 10kb in length, it
    /// will be truncated to 10kb.
    ///
    /// If multiple ValidatingAdmissionPolicyBinding resources match an
    /// API request, then the valueExpression will be evaluated for
    /// each binding. All unique values produced by the valueExpressions
    /// will be joined together in a comma-separated list.
    ///
    /// Required.
    #[prost(string, optional, tag = "2")]
    pub value_expression: ::core::option::Option<::prost::alloc::string::String>,
}
/// ExpressionWarning is a warning information that targets a specific expression.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExpressionWarning {
    /// The path to the field that refers the expression.
    /// For example, the reference to the expression of the first item of
    /// validations is "spec.validations\[0\].expression"
    #[prost(string, optional, tag = "2")]
    pub field_ref: ::core::option::Option<::prost::alloc::string::String>,
    /// The content of type checking information in a human-readable form.
    /// Each line of the warning contains the type that the expression is checked
    /// against, followed by the type check error from the compiler.
    #[prost(string, optional, tag = "3")]
    pub warning: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MatchCondition {
    /// Name is an identifier for this match condition, used for strategic merging of MatchConditions,
    /// as well as providing an identifier for logging purposes. A good name should be descriptive of
    /// the associated expression.
    /// Name must be a qualified name consisting of alphanumeric characters, '-', '_' or '.', and
    /// must start and end with an alphanumeric character (e.g. 'MyName',  or 'my.name',  or
    /// '123-abc', regex used for validation is '([A-Za-z0-9][-A-Za-z0-9_.]*)?\[A-Za-z0-9\]') with an
    /// optional DNS subdomain prefix and '/' (e.g. 'example.com/MyName')
    ///
    /// Required.
    #[prost(string, optional, tag = "1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// Expression represents the expression which will be evaluated by CEL. Must evaluate to bool.
    /// CEL expressions have access to the contents of the AdmissionRequest and Authorizer, organized into CEL variables:
    ///
    /// 'object' - The object from the incoming request. The value is null for DELETE requests.
    /// 'oldObject' - The existing object. The value is null for CREATE requests.
    /// 'request' - Attributes of the admission request(/pkg/apis/admission/types.go#AdmissionRequest).
    /// 'authorizer' - A CEL Authorizer. May be used to perform authorization checks for the principal (user or service account) of the request.
    ///    See <https://pkg.go.dev/k8s.io/apiserver/pkg/cel/library#Authz>
    /// 'authorizer.requestResource' - A CEL ResourceCheck constructed from the 'authorizer' and configured with the
    ///    request resource.
    /// Documentation on CEL: <https://kubernetes.io/docs/reference/using-api/cel/>
    ///
    /// Required.
    #[prost(string, optional, tag = "2")]
    pub expression: ::core::option::Option<::prost::alloc::string::String>,
}
/// MatchResources decides whether to run the admission control policy on an object based
/// on whether it meets the match criteria.
/// The exclude rules take precedence over include rules (if a resource matches both, it is excluded)
/// +structType=atomic
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MatchResources {
    /// NamespaceSelector decides whether to run the admission control policy on an object based
    /// on whether the namespace for that object matches the selector. If the
    /// object itself is a namespace, the matching is performed on
    /// object.metadata.labels. If the object is another cluster scoped resource,
    /// it never skips the policy.
    ///
    /// For example, to run the webhook on any objects whose namespace is not
    /// associated with "runlevel" of "0" or "1";  you will set the selector as
    /// follows:
    /// "namespaceSelector": {
    ///    "matchExpressions": [
    ///      {
    ///        "key": "runlevel",
    ///        "operator": "NotIn",
    ///        "values": [
    ///          "0",
    ///          "1"
    ///        ]
    ///      }
    ///    ]
    /// }
    ///
    /// If instead you want to only run the policy on any objects whose
    /// namespace is associated with the "environment" of "prod" or "staging";
    /// you will set the selector as follows:
    /// "namespaceSelector": {
    ///    "matchExpressions": [
    ///      {
    ///        "key": "environment",
    ///        "operator": "In",
    ///        "values": [
    ///          "prod",
    ///          "staging"
    ///        ]
    ///      }
    ///    ]
    /// }
    ///
    /// See
    /// <https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/>
    /// for more examples of label selectors.
    ///
    /// Default to the empty LabelSelector, which matches everything.
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub namespace_selector:
        ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::LabelSelector>,
    /// ObjectSelector decides whether to run the validation based on if the
    /// object has matching labels. objectSelector is evaluated against both
    /// the oldObject and newObject that would be sent to the cel validation, and
    /// is considered to match if either object matches the selector. A null
    /// object (oldObject in the case of create, or newObject in the case of
    /// delete) or an object that cannot have labels (like a
    /// DeploymentRollback or a PodProxyOptions object) is not considered to
    /// match.
    /// Use the object selector only if the webhook is opt-in, because end
    /// users may skip the admission webhook by setting the labels.
    /// Default to the empty LabelSelector, which matches everything.
    /// +optional
    #[prost(message, optional, tag = "2")]
    pub object_selector:
        ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::LabelSelector>,
    /// ResourceRules describes what operations on what resources/subresources the ValidatingAdmissionPolicy matches.
    /// The policy cares about an operation if it matches _any_ Rule.
    /// +listType=atomic
    /// +optional
    #[prost(message, repeated, tag = "3")]
    pub resource_rules: ::prost::alloc::vec::Vec<NamedRuleWithOperations>,
    /// ExcludeResourceRules describes what operations on what resources/subresources the ValidatingAdmissionPolicy should not care about.
    /// The exclude rules take precedence over include rules (if a resource matches both, it is excluded)
    /// +listType=atomic
    /// +optional
    #[prost(message, repeated, tag = "4")]
    pub exclude_resource_rules: ::prost::alloc::vec::Vec<NamedRuleWithOperations>,
    /// matchPolicy defines how the "MatchResources" list is used to match incoming requests.
    /// Allowed values are "Exact" or "Equivalent".
    ///
    /// - Exact: match a request only if it exactly matches a specified rule.
    /// For example, if deployments can be modified via apps/v1, apps/v1beta1, and extensions/v1beta1,
    /// but "rules" only included `apiGroups:\["apps"\], apiVersions:\["v1"\], resources: \["deployments"\]`,
    /// a request to apps/v1beta1 or extensions/v1beta1 would not be sent to the ValidatingAdmissionPolicy.
    ///
    /// - Equivalent: match a request if modifies a resource listed in rules, even via another API group or version.
    /// For example, if deployments can be modified via apps/v1, apps/v1beta1, and extensions/v1beta1,
    /// and "rules" only included `apiGroups:\["apps"\], apiVersions:\["v1"\], resources: \["deployments"\]`,
    /// a request to apps/v1beta1 or extensions/v1beta1 would be converted to apps/v1 and sent to the ValidatingAdmissionPolicy.
    ///
    /// Defaults to "Equivalent"
    /// +optional
    #[prost(string, optional, tag = "7")]
    pub match_policy: ::core::option::Option<::prost::alloc::string::String>,
}
/// NamedRuleWithOperations is a tuple of Operations and Resources with ResourceNames.
/// +structType=atomic
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NamedRuleWithOperations {
    /// ResourceNames is an optional white list of names that the rule applies to.  An empty set means that everything is allowed.
    /// +listType=atomic
    /// +optional
    #[prost(string, repeated, tag = "1")]
    pub resource_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// RuleWithOperations is a tuple of Operations and Resources.
    #[prost(message, optional, tag = "2")]
    pub rule_with_operations: ::core::option::Option<super::v1::RuleWithOperations>,
}
/// ParamKind is a tuple of Group Kind and Version.
/// +structType=atomic
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParamKind {
    /// APIVersion is the API group version the resources belong to.
    /// In format of "group/version".
    /// Required.
    #[prost(string, optional, tag = "1")]
    pub api_version: ::core::option::Option<::prost::alloc::string::String>,
    /// Kind is the API kind the resources belong to.
    /// Required.
    #[prost(string, optional, tag = "2")]
    pub kind: ::core::option::Option<::prost::alloc::string::String>,
}
/// ParamRef describes how to locate the params to be used as input to
/// expressions of rules applied by a policy binding.
/// +structType=atomic
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParamRef {
    /// `name` is the name of the resource being referenced.
    ///
    /// `name` and `selector` are mutually exclusive properties. If one is set,
    /// the other must be unset.
    ///
    /// +optional
    #[prost(string, optional, tag = "1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// namespace is the namespace of the referenced resource. Allows limiting
    /// the search for params to a specific namespace. Applies to both `name` and
    /// `selector` fields.
    ///
    /// A per-namespace parameter may be used by specifying a namespace-scoped
    /// `paramKind` in the policy and leaving this field empty.
    ///
    /// - If `paramKind` is cluster-scoped, this field MUST be unset. Setting this
    /// field results in a configuration error.
    ///
    /// - If `paramKind` is namespace-scoped, the namespace of the object being
    /// evaluated for admission will be used when this field is left unset. Take
    /// care that if this is left empty the binding must not match any cluster-scoped
    /// resources, which will result in an error.
    ///
    /// +optional
    #[prost(string, optional, tag = "2")]
    pub namespace: ::core::option::Option<::prost::alloc::string::String>,
    /// selector can be used to match multiple param objects based on their labels.
    /// Supply selector: {} to match all resources of the ParamKind.
    ///
    /// If multiple params are found, they are all evaluated with the policy expressions
    /// and the results are ANDed together.
    ///
    /// One of `name` or `selector` must be set, but `name` and `selector` are
    /// mutually exclusive properties. If one is set, the other must be unset.
    ///
    /// +optional
    #[prost(message, optional, tag = "3")]
    pub selector:
        ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::LabelSelector>,
    /// `parameterNotFoundAction` controls the behavior of the binding when the resource
    /// exists, and name or selector is valid, but there are no parameters
    /// matched by the binding. If the value is set to `Allow`, then no
    /// matched parameters will be treated as successful validation by the binding.
    /// If set to `Deny`, then no matched parameters will be subject to the
    /// `failurePolicy` of the policy.
    ///
    /// Allowed values are `Allow` or `Deny`
    /// Default to `Deny`
    /// +optional
    #[prost(string, optional, tag = "4")]
    pub parameter_not_found_action: ::core::option::Option<::prost::alloc::string::String>,
}
/// TypeChecking contains results of type checking the expressions in the
/// ValidatingAdmissionPolicy
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TypeChecking {
    /// The type checking warnings for each expression.
    /// +optional
    /// +listType=atomic
    #[prost(message, repeated, tag = "1")]
    pub expression_warnings: ::prost::alloc::vec::Vec<ExpressionWarning>,
}
/// ValidatingAdmissionPolicy describes the definition of an admission validation policy that accepts or rejects an object without changing it.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatingAdmissionPolicy {
    /// Standard object metadata; More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata.>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// Specification of the desired behavior of the ValidatingAdmissionPolicy.
    #[prost(message, optional, tag = "2")]
    pub spec: ::core::option::Option<ValidatingAdmissionPolicySpec>,
    /// The status of the ValidatingAdmissionPolicy, including warnings that are useful to determine if the policy
    /// behaves in the expected way.
    /// Populated by the system.
    /// Read-only.
    /// +optional
    #[prost(message, optional, tag = "3")]
    pub status: ::core::option::Option<ValidatingAdmissionPolicyStatus>,
}
/// ValidatingAdmissionPolicyBinding binds the ValidatingAdmissionPolicy with paramerized resources.
/// ValidatingAdmissionPolicyBinding and parameter CRDs together define how cluster administrators configure policies for clusters.
///
/// For a given admission request, each binding will cause its policy to be
/// evaluated N times, where N is 1 for policies/bindings that don't use
/// params, otherwise N is the number of parameters selected by the binding.
///
/// The CEL expressions of a policy must have a computed CEL cost below the maximum
/// CEL budget. Each evaluation of the policy is given an independent CEL cost budget.
/// Adding/removing policies, bindings, or params can not affect whether a
/// given (policy, binding, param) combination is within its own CEL budget.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatingAdmissionPolicyBinding {
    /// Standard object metadata; More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata.>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// Specification of the desired behavior of the ValidatingAdmissionPolicyBinding.
    #[prost(message, optional, tag = "2")]
    pub spec: ::core::option::Option<ValidatingAdmissionPolicyBindingSpec>,
}
/// ValidatingAdmissionPolicyBindingList is a list of ValidatingAdmissionPolicyBinding.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatingAdmissionPolicyBindingList {
    /// Standard list metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// List of PolicyBinding.
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<ValidatingAdmissionPolicyBinding>,
}
/// ValidatingAdmissionPolicyBindingSpec is the specification of the ValidatingAdmissionPolicyBinding.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatingAdmissionPolicyBindingSpec {
    /// PolicyName references a ValidatingAdmissionPolicy name which the ValidatingAdmissionPolicyBinding binds to.
    /// If the referenced resource does not exist, this binding is considered invalid and will be ignored
    /// Required.
    #[prost(string, optional, tag = "1")]
    pub policy_name: ::core::option::Option<::prost::alloc::string::String>,
    /// paramRef specifies the parameter resource used to configure the admission control policy.
    /// It should point to a resource of the type specified in ParamKind of the bound ValidatingAdmissionPolicy.
    /// If the policy specifies a ParamKind and the resource referred to by ParamRef does not exist, this binding is considered mis-configured and the FailurePolicy of the ValidatingAdmissionPolicy applied.
    /// If the policy does not specify a ParamKind then this field is ignored, and the rules are evaluated without a param.
    /// +optional
    #[prost(message, optional, tag = "2")]
    pub param_ref: ::core::option::Option<ParamRef>,
    /// MatchResources declares what resources match this binding and will be validated by it.
    /// Note that this is intersected with the policy's matchConstraints, so only requests that are matched by the policy can be selected by this.
    /// If this is unset, all resources matched by the policy are validated by this binding
    /// When resourceRules is unset, it does not constrain resource matching. If a resource is matched by the other fields of this object, it will be validated.
    /// Note that this is differs from ValidatingAdmissionPolicy matchConstraints, where resourceRules are required.
    /// +optional
    #[prost(message, optional, tag = "3")]
    pub match_resources: ::core::option::Option<MatchResources>,
    /// validationActions declares how Validations of the referenced ValidatingAdmissionPolicy are enforced.
    /// If a validation evaluates to false it is always enforced according to these actions.
    ///
    /// Failures defined by the ValidatingAdmissionPolicy's FailurePolicy are enforced according
    /// to these actions only if the FailurePolicy is set to Fail, otherwise the failures are
    /// ignored. This includes compilation errors, runtime errors and misconfigurations of the policy.
    ///
    /// validationActions is declared as a set of action values. Order does
    /// not matter. validationActions may not contain duplicates of the same action.
    ///
    /// The supported actions values are:
    ///
    /// "Deny" specifies that a validation failure results in a denied request.
    ///
    /// "Warn" specifies that a validation failure is reported to the request client
    /// in HTTP Warning headers, with a warning code of 299. Warnings can be sent
    /// both for allowed or denied admission responses.
    ///
    /// "Audit" specifies that a validation failure is included in the published
    /// audit event for the request. The audit event will contain a
    /// `validation.policy.admission.k8s.io/validation_failure` audit annotation
    /// with a value containing the details of the validation failures, formatted as
    /// a JSON list of objects, each with the following fields:
    /// - message: The validation failure message string
    /// - policy: The resource name of the ValidatingAdmissionPolicy
    /// - binding: The resource name of the ValidatingAdmissionPolicyBinding
    /// - expressionIndex: The index of the failed validations in the ValidatingAdmissionPolicy
    /// - validationActions: The enforcement actions enacted for the validation failure
    /// Example audit annotation:
    /// `"validation.policy.admission.k8s.io/validation_failure": "\[{\"message\": \"Invalid value\", {\"policy\": \"policy.example.com\", {\"binding\": \"policybinding.example.com\", {\"expressionIndex\": \"1\", {\"validationActions\": [\"Audit\"\]}]"`
    ///
    /// Clients should expect to handle additional values by ignoring
    /// any values not recognized.
    ///
    /// "Deny" and "Warn" may not be used together since this combination
    /// needlessly duplicates the validation failure both in the
    /// API response body and the HTTP warning headers.
    ///
    /// Required.
    /// +listType=set
    #[prost(string, repeated, tag = "4")]
    pub validation_actions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// ValidatingAdmissionPolicyList is a list of ValidatingAdmissionPolicy.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatingAdmissionPolicyList {
    /// Standard list metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// List of ValidatingAdmissionPolicy.
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<ValidatingAdmissionPolicy>,
}
/// ValidatingAdmissionPolicySpec is the specification of the desired behavior of the AdmissionPolicy.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatingAdmissionPolicySpec {
    /// ParamKind specifies the kind of resources used to parameterize this policy.
    /// If absent, there are no parameters for this policy and the param CEL variable will not be provided to validation expressions.
    /// If ParamKind refers to a non-existent kind, this policy definition is mis-configured and the FailurePolicy is applied.
    /// If paramKind is specified but paramRef is unset in ValidatingAdmissionPolicyBinding, the params variable will be null.
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub param_kind: ::core::option::Option<ParamKind>,
    /// MatchConstraints specifies what resources this policy is designed to validate.
    /// The AdmissionPolicy cares about a request if it matches _all_ Constraints.
    /// However, in order to prevent clusters from being put into an unstable state that cannot be recovered from via the API
    /// ValidatingAdmissionPolicy cannot match ValidatingAdmissionPolicy and ValidatingAdmissionPolicyBinding.
    /// Required.
    #[prost(message, optional, tag = "2")]
    pub match_constraints: ::core::option::Option<MatchResources>,
    /// Validations contain CEL expressions which is used to apply the validation.
    /// Validations and AuditAnnotations may not both be empty; a minimum of one Validations or AuditAnnotations is
    /// required.
    /// +listType=atomic
    /// +optional
    #[prost(message, repeated, tag = "3")]
    pub validations: ::prost::alloc::vec::Vec<Validation>,
    /// failurePolicy defines how to handle failures for the admission policy. Failures can
    /// occur from CEL expression parse errors, type check errors, runtime errors and invalid
    /// or mis-configured policy definitions or bindings.
    ///
    /// A policy is invalid if spec.paramKind refers to a non-existent Kind.
    /// A binding is invalid if spec.paramRef.name refers to a non-existent resource.
    ///
    /// failurePolicy does not define how validations that evaluate to false are handled.
    ///
    /// When failurePolicy is set to Fail, ValidatingAdmissionPolicyBinding validationActions
    /// define how failures are enforced.
    ///
    /// Allowed values are Ignore or Fail. Defaults to Fail.
    /// +optional
    #[prost(string, optional, tag = "4")]
    pub failure_policy: ::core::option::Option<::prost::alloc::string::String>,
    /// auditAnnotations contains CEL expressions which are used to produce audit
    /// annotations for the audit event of the API request.
    /// validations and auditAnnotations may not both be empty; a least one of validations or auditAnnotations is
    /// required.
    /// +listType=atomic
    /// +optional
    #[prost(message, repeated, tag = "5")]
    pub audit_annotations: ::prost::alloc::vec::Vec<AuditAnnotation>,
    /// MatchConditions is a list of conditions that must be met for a request to be validated.
    /// Match conditions filter requests that have already been matched by the rules,
    /// namespaceSelector, and objectSelector. An empty list of matchConditions matches all requests.
    /// There are a maximum of 64 match conditions allowed.
    ///
    /// If a parameter object is provided, it can be accessed via the `params` handle in the same
    /// manner as validation expressions.
    ///
    /// The exact matching logic is (in order):
    ///    1. If ANY matchCondition evaluates to FALSE, the policy is skipped.
    ///    2. If ALL matchConditions evaluate to TRUE, the policy is evaluated.
    ///    3. If any matchCondition evaluates to an error (but none are FALSE):
    ///       - If failurePolicy=Fail, reject the request
    ///       - If failurePolicy=Ignore, the policy is skipped
    ///
    /// +patchMergeKey=name
    /// +patchStrategy=merge
    /// +listType=map
    /// +listMapKey=name
    /// +optional
    #[prost(message, repeated, tag = "6")]
    pub match_conditions: ::prost::alloc::vec::Vec<MatchCondition>,
    /// Variables contain definitions of variables that can be used in composition of other expressions.
    /// Each variable is defined as a named CEL expression.
    /// The variables defined here will be available under `variables` in other expressions of the policy
    /// except MatchConditions because MatchConditions are evaluated before the rest of the policy.
    ///
    /// The expression of a variable can refer to other variables defined earlier in the list but not those after.
    /// Thus, Variables must be sorted by the order of first appearance and acyclic.
    /// +patchMergeKey=name
    /// +patchStrategy=merge
    /// +listType=map
    /// +listMapKey=name
    /// +optional
    #[prost(message, repeated, tag = "7")]
    pub variables: ::prost::alloc::vec::Vec<Variable>,
}
/// ValidatingAdmissionPolicyStatus represents the status of a ValidatingAdmissionPolicy.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatingAdmissionPolicyStatus {
    /// The generation observed by the controller.
    /// +optional
    #[prost(int64, optional, tag = "1")]
    pub observed_generation: ::core::option::Option<i64>,
    /// The results of type checking for each expression.
    /// Presence of this field indicates the completion of the type checking.
    /// +optional
    #[prost(message, optional, tag = "2")]
    pub type_checking: ::core::option::Option<TypeChecking>,
    /// The conditions represent the latest available observations of a policy's current state.
    /// +optional
    /// +listType=map
    /// +listMapKey=type
    #[prost(message, repeated, tag = "3")]
    pub conditions:
        ::prost::alloc::vec::Vec<super::super::super::apimachinery::pkg::apis::meta::v1::Condition>,
}
/// Validation specifies the CEL expression which is used to apply the validation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Validation {
    /// Expression represents the expression which will be evaluated by CEL.
    /// ref: <https://github.com/google/cel-spec>
    /// CEL expressions have access to the contents of the API request/response, organized into CEL variables as well as some other useful variables:
    ///
    /// - 'object' - The object from the incoming request. The value is null for DELETE requests.
    /// - 'oldObject' - The existing object. The value is null for CREATE requests.
    /// - 'request' - Attributes of the API request([ref](/pkg/apis/admission/types.go#AdmissionRequest)).
    /// - 'params' - Parameter resource referred to by the policy binding being evaluated. Only populated if the policy has a ParamKind.
    /// - 'namespaceObject' - The namespace object that the incoming object belongs to. The value is null for cluster-scoped resources.
    /// - 'variables' - Map of composited variables, from its name to its lazily evaluated value.
    ///    For example, a variable named 'foo' can be accessed as 'variables.foo'.
    /// - 'authorizer' - A CEL Authorizer. May be used to perform authorization checks for the principal (user or service account) of the request.
    ///    See <https://pkg.go.dev/k8s.io/apiserver/pkg/cel/library#Authz>
    /// - 'authorizer.requestResource' - A CEL ResourceCheck constructed from the 'authorizer' and configured with the
    ///    request resource.
    ///
    /// The `apiVersion`, `kind`, `metadata.name` and `metadata.generateName` are always accessible from the root of the
    /// object. No other metadata properties are accessible.
    ///
    /// Only property names of the form `[a-zA-Z_.-/][a-zA-Z0-9_.-/]*` are accessible.
    /// Accessible property names are escaped according to the following rules when accessed in the expression:
    /// - '__' escapes to '__underscores__'
    /// - '.' escapes to '__dot__'
    /// - '-' escapes to '__dash__'
    /// - '/' escapes to '__slash__'
    /// - Property names that exactly match a CEL RESERVED keyword escape to '__{keyword}__'. The keywords are:
    /// 	  "true", "false", "null", "in", "as", "break", "const", "continue", "else", "for", "function", "if",
    /// 	  "import", "let", "loop", "package", "namespace", "return".
    /// Examples:
    ///    - Expression accessing a property named "namespace": {"Expression": "object.__namespace__ > 0"}
    ///    - Expression accessing a property named "x-prop": {"Expression": "object.x__dash__prop > 0"}
    ///    - Expression accessing a property named "redact__d": {"Expression": "object.redact__underscores__d > 0"}
    ///
    /// Equality on arrays with list type of 'set' or 'map' ignores element order, i.e. \[1, 2\] == \[2, 1\].
    /// Concatenation on arrays with x-kubernetes-list-type use the semantics of the list type:
    ///    - 'set': `X + Y` performs a union where the array positions of all elements in `X` are preserved and
    ///      non-intersecting elements in `Y` are appended, retaining their partial order.
    ///    - 'map': `X + Y` performs a merge where the array positions of all keys in `X` are preserved but the values
    ///      are overwritten by values in `Y` when the key sets of `X` and `Y` intersect. Elements in `Y` with
    ///      non-intersecting keys are appended, retaining their partial order.
    /// Required.
    #[prost(string, optional, tag = "1")]
    pub expression: ::core::option::Option<::prost::alloc::string::String>,
    /// Message represents the message displayed when validation fails. The message is required if the Expression contains
    /// line breaks. The message must not contain line breaks.
    /// If unset, the message is "failed rule: {Rule}".
    /// e.g. "must be a URL with the host matching spec.host"
    /// If the Expression contains line breaks. Message is required.
    /// The message must not contain line breaks.
    /// If unset, the message is "failed Expression: {Expression}".
    /// +optional
    #[prost(string, optional, tag = "2")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
    /// Reason represents a machine-readable description of why this validation failed.
    /// If this is the first validation in the list to fail, this reason, as well as the
    /// corresponding HTTP response code, are used in the
    /// HTTP response to the client.
    /// The currently supported reasons are: "Unauthorized", "Forbidden", "Invalid", "RequestEntityTooLarge".
    /// If not set, StatusReasonInvalid is used in the response to the client.
    /// +optional
    #[prost(string, optional, tag = "3")]
    pub reason: ::core::option::Option<::prost::alloc::string::String>,
    /// messageExpression declares a CEL expression that evaluates to the validation failure message that is returned when this rule fails.
    /// Since messageExpression is used as a failure message, it must evaluate to a string.
    /// If both message and messageExpression are present on a validation, then messageExpression will be used if validation fails.
    /// If messageExpression results in a runtime error, the runtime error is logged, and the validation failure message is produced
    /// as if the messageExpression field were unset. If messageExpression evaluates to an empty string, a string with only spaces, or a string
    /// that contains line breaks, then the validation failure message will also be produced as if the messageExpression field were unset, and
    /// the fact that messageExpression produced an empty string/string with only spaces/string with line breaks will be logged.
    /// messageExpression has access to all the same variables as the `expression` except for 'authorizer' and 'authorizer.requestResource'.
    /// Example:
    /// "object.x must be less than max ("+string(params.max)+")"
    /// +optional
    #[prost(string, optional, tag = "4")]
    pub message_expression: ::core::option::Option<::prost::alloc::string::String>,
}
/// Variable is the definition of a variable that is used for composition.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Variable {
    /// Name is the name of the variable. The name must be a valid CEL identifier and unique among all variables.
    /// The variable can be accessed in other expressions through `variables`
    /// For example, if name is "foo", the variable will be available as `variables.foo`
    #[prost(string, optional, tag = "1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// Expression is the expression that will be evaluated as the value of the variable.
    /// The CEL expression has access to the same identifiers as the CEL expressions in Validation.
    #[prost(string, optional, tag = "2")]
    pub expression: ::core::option::Option<::prost::alloc::string::String>,
}

impl crate::Resource for ValidatingAdmissionPolicy {
    const API_VERSION: &'static str = "admissionregistration.k8s.io/v1alpha1";
    const GROUP: &'static str = "admissionregistration.k8s.io";
    const VERSION: &'static str = "v1alpha1";
    const KIND: &'static str = "ValidatingAdmissionPolicy";
    const URL_PATH_SEGMENT: &'static str = "validatingadmissionpolicies";
    type Scope = crate::ClusterResourceScope;
}
impl crate::HasMetadata for ValidatingAdmissionPolicy {
    type Metadata = crate::apimachinery::pkg::apis::meta::v1::ObjectMeta;
    fn metadata(&self) -> Option<&<Self as crate::HasMetadata>::Metadata> {
        self.metadata.as_ref()
    }
    fn metadata_mut(&mut self) -> Option<&mut <Self as crate::HasMetadata>::Metadata> {
        self.metadata.as_mut()
    }
}
impl crate::HasSpec for ValidatingAdmissionPolicy {
    type Spec = crate::api::admissionregistration::v1alpha1::ValidatingAdmissionPolicySpec;
    fn spec(&self) -> Option<&<Self as crate::HasSpec>::Spec> {
        self.spec.as_ref()
    }
    fn spec_mut(&mut self) -> Option<&mut <Self as crate::HasSpec>::Spec> {
        self.spec.as_mut()
    }
}
impl crate::HasStatus for ValidatingAdmissionPolicy {
    type Status = crate::api::admissionregistration::v1alpha1::ValidatingAdmissionPolicyStatus;
    fn status(&self) -> Option<&<Self as crate::HasStatus>::Status> {
        self.status.as_ref()
    }
    fn status_mut(&mut self) -> Option<&mut <Self as crate::HasStatus>::Status> {
        self.status.as_mut()
    }
}
impl crate::HasConditions for ValidatingAdmissionPolicy {
    type Condition = crate::apimachinery::pkg::apis::meta::v1::Condition;
    fn conditions(&self) -> Option<&[<Self as crate::HasConditions>::Condition]> {
        self.status.as_ref().map(|s| s.conditions.as_slice())
    }
    fn conditions_mut(&mut self) -> Option<&mut Vec<<Self as crate::HasConditions>::Condition>> {
        self.status.as_mut().and_then(|s| Some(s.conditions.as_mut()))
    }
}

impl crate::Resource for ValidatingAdmissionPolicyBinding {
    const API_VERSION: &'static str = "admissionregistration.k8s.io/v1alpha1";
    const GROUP: &'static str = "admissionregistration.k8s.io";
    const VERSION: &'static str = "v1alpha1";
    const KIND: &'static str = "ValidatingAdmissionPolicyBinding";
    const URL_PATH_SEGMENT: &'static str = "validatingadmissionpolicybindings";
    type Scope = crate::ClusterResourceScope;
}
impl crate::HasMetadata for ValidatingAdmissionPolicyBinding {
    type Metadata = crate::apimachinery::pkg::apis::meta::v1::ObjectMeta;
    fn metadata(&self) -> Option<&<Self as crate::HasMetadata>::Metadata> {
        self.metadata.as_ref()
    }
    fn metadata_mut(&mut self) -> Option<&mut <Self as crate::HasMetadata>::Metadata> {
        self.metadata.as_mut()
    }
}
impl crate::HasSpec for ValidatingAdmissionPolicyBinding {
    type Spec = crate::api::admissionregistration::v1alpha1::ValidatingAdmissionPolicyBindingSpec;
    fn spec(&self) -> Option<&<Self as crate::HasSpec>::Spec> {
        self.spec.as_ref()
    }
    fn spec_mut(&mut self) -> Option<&mut <Self as crate::HasSpec>::Spec> {
        self.spec.as_mut()
    }
}
