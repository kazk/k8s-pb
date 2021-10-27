/// ControllerRevision implements an immutable snapshot of state data. Clients
/// are responsible for serializing and deserializing the objects that contain
/// their internal state.
/// Once a ControllerRevision has been successfully created, it can not be updated.
/// The API Server will fail validation of all requests that attempt to mutate
/// the Data field. ControllerRevisions may, however, be deleted. Note that, due to its use by both
/// the DaemonSet and StatefulSet controllers for update and rollback, this object is beta. However,
/// it may be subject to name and representation changes in future releases, and clients should not
/// depend on its stability. It is primarily for internal use by controllers.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ControllerRevision {
    /// Standard object's metadata.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// Data is the serialized representation of the state.
    #[prost(message, optional, tag="2")]
    pub data: ::core::option::Option<super::super::super::apimachinery::pkg::runtime::RawExtension>,
    /// Revision indicates the revision of the state represented by Data.
    #[prost(int64, optional, tag="3")]
    pub revision: ::core::option::Option<i64>,
}
/// ControllerRevisionList is a resource containing a list of ControllerRevision objects.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ControllerRevisionList {
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// Items is the list of ControllerRevisions
    #[prost(message, repeated, tag="2")]
    pub items: ::prost::alloc::vec::Vec<ControllerRevision>,
}
/// DaemonSet represents the configuration of a daemon set.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DaemonSet {
    /// Standard object's metadata.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// The desired behavior of this daemon set.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
    /// +optional
    #[prost(message, optional, tag="2")]
    pub spec: ::core::option::Option<DaemonSetSpec>,
    /// The current status of this daemon set. This data may be
    /// out of date by some window of time.
    /// Populated by the system.
    /// Read-only.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
    /// +optional
    #[prost(message, optional, tag="3")]
    pub status: ::core::option::Option<DaemonSetStatus>,
}
/// DaemonSetCondition describes the state of a DaemonSet at a certain point.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DaemonSetCondition {
    /// Type of DaemonSet condition.
    #[prost(string, optional, tag="1")]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
    /// Status of the condition, one of True, False, Unknown.
    #[prost(string, optional, tag="2")]
    pub status: ::core::option::Option<::prost::alloc::string::String>,
    /// Last time the condition transitioned from one status to another.
    /// +optional
    #[prost(message, optional, tag="3")]
    pub last_transition_time: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::Time>,
    /// The reason for the condition's last transition.
    /// +optional
    #[prost(string, optional, tag="4")]
    pub reason: ::core::option::Option<::prost::alloc::string::String>,
    /// A human readable message indicating details about the transition.
    /// +optional
    #[prost(string, optional, tag="5")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
/// DaemonSetList is a collection of daemon sets.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DaemonSetList {
    /// Standard list metadata.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// A list of daemon sets.
    #[prost(message, repeated, tag="2")]
    pub items: ::prost::alloc::vec::Vec<DaemonSet>,
}
/// DaemonSetSpec is the specification of a daemon set.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DaemonSetSpec {
    /// A label query over pods that are managed by the daemon set.
    /// Must match in order to be controlled.
    /// It must match the pod template's labels.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/#label-selectors
    #[prost(message, optional, tag="1")]
    pub selector: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::LabelSelector>,
    /// An object that describes the pod that will be created.
    /// The DaemonSet will create exactly one copy of this pod on every node
    /// that matches the template's node selector (or on every node if no node
    /// selector is specified).
    /// More info: https://kubernetes.io/docs/concepts/workloads/controllers/replicationcontroller#pod-template
    #[prost(message, optional, tag="2")]
    pub template: ::core::option::Option<super::super::core::v1::PodTemplateSpec>,
    /// An update strategy to replace existing DaemonSet pods with new pods.
    /// +optional
    #[prost(message, optional, tag="3")]
    pub update_strategy: ::core::option::Option<DaemonSetUpdateStrategy>,
    /// The minimum number of seconds for which a newly created DaemonSet pod should
    /// be ready without any of its container crashing, for it to be considered
    /// available. Defaults to 0 (pod will be considered available as soon as it
    /// is ready).
    /// +optional
    #[prost(int32, optional, tag="4")]
    pub min_ready_seconds: ::core::option::Option<i32>,
    /// The number of old history to retain to allow rollback.
    /// This is a pointer to distinguish between explicit zero and not specified.
    /// Defaults to 10.
    /// +optional
    #[prost(int32, optional, tag="6")]
    pub revision_history_limit: ::core::option::Option<i32>,
}
/// DaemonSetStatus represents the current status of a daemon set.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DaemonSetStatus {
    /// The number of nodes that are running at least 1
    /// daemon pod and are supposed to run the daemon pod.
    /// More info: https://kubernetes.io/docs/concepts/workloads/controllers/daemonset/
    #[prost(int32, optional, tag="1")]
    pub current_number_scheduled: ::core::option::Option<i32>,
    /// The number of nodes that are running the daemon pod, but are
    /// not supposed to run the daemon pod.
    /// More info: https://kubernetes.io/docs/concepts/workloads/controllers/daemonset/
    #[prost(int32, optional, tag="2")]
    pub number_misscheduled: ::core::option::Option<i32>,
    /// The total number of nodes that should be running the daemon
    /// pod (including nodes correctly running the daemon pod).
    /// More info: https://kubernetes.io/docs/concepts/workloads/controllers/daemonset/
    #[prost(int32, optional, tag="3")]
    pub desired_number_scheduled: ::core::option::Option<i32>,
    /// The number of nodes that should be running the daemon pod and have one
    /// or more of the daemon pod running and ready.
    #[prost(int32, optional, tag="4")]
    pub number_ready: ::core::option::Option<i32>,
    /// The most recent generation observed by the daemon set controller.
    /// +optional
    #[prost(int64, optional, tag="5")]
    pub observed_generation: ::core::option::Option<i64>,
    /// The total number of nodes that are running updated daemon pod
    /// +optional
    #[prost(int32, optional, tag="6")]
    pub updated_number_scheduled: ::core::option::Option<i32>,
    /// The number of nodes that should be running the
    /// daemon pod and have one or more of the daemon pod running and
    /// available (ready for at least spec.minReadySeconds)
    /// +optional
    #[prost(int32, optional, tag="7")]
    pub number_available: ::core::option::Option<i32>,
    /// The number of nodes that should be running the
    /// daemon pod and have none of the daemon pod running and available
    /// (ready for at least spec.minReadySeconds)
    /// +optional
    #[prost(int32, optional, tag="8")]
    pub number_unavailable: ::core::option::Option<i32>,
    /// Count of hash collisions for the DaemonSet. The DaemonSet controller
    /// uses this field as a collision avoidance mechanism when it needs to
    /// create the name for the newest ControllerRevision.
    /// +optional
    #[prost(int32, optional, tag="9")]
    pub collision_count: ::core::option::Option<i32>,
    /// Represents the latest available observations of a DaemonSet's current state.
    /// +optional
    /// +patchMergeKey=type
    /// +patchStrategy=merge
    #[prost(message, repeated, tag="10")]
    pub conditions: ::prost::alloc::vec::Vec<DaemonSetCondition>,
}
/// DaemonSetUpdateStrategy is a struct used to control the update strategy for a DaemonSet.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DaemonSetUpdateStrategy {
    /// Type of daemon set update. Can be "RollingUpdate" or "OnDelete". Default is RollingUpdate.
    /// +optional
    #[prost(string, optional, tag="1")]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
    /// Rolling update config params. Present only if type = "RollingUpdate".
    /// ---
    /// TODO: Update this to follow our convention for oneOf, whatever we decide it
    /// to be. Same as Deployment `strategy.rollingUpdate`.
    /// See https://github.com/kubernetes/kubernetes/issues/35345
    /// +optional
    #[prost(message, optional, tag="2")]
    pub rolling_update: ::core::option::Option<RollingUpdateDaemonSet>,
}
/// Deployment enables declarative updates for Pods and ReplicaSets.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Deployment {
    /// Standard object's metadata.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// Specification of the desired behavior of the Deployment.
    /// +optional
    #[prost(message, optional, tag="2")]
    pub spec: ::core::option::Option<DeploymentSpec>,
    /// Most recently observed status of the Deployment.
    /// +optional
    #[prost(message, optional, tag="3")]
    pub status: ::core::option::Option<DeploymentStatus>,
}
/// DeploymentCondition describes the state of a deployment at a certain point.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeploymentCondition {
    /// Type of deployment condition.
    #[prost(string, optional, tag="1")]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
    /// Status of the condition, one of True, False, Unknown.
    #[prost(string, optional, tag="2")]
    pub status: ::core::option::Option<::prost::alloc::string::String>,
    /// The last time this condition was updated.
    #[prost(message, optional, tag="6")]
    pub last_update_time: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::Time>,
    /// Last time the condition transitioned from one status to another.
    #[prost(message, optional, tag="7")]
    pub last_transition_time: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::Time>,
    /// The reason for the condition's last transition.
    #[prost(string, optional, tag="4")]
    pub reason: ::core::option::Option<::prost::alloc::string::String>,
    /// A human readable message indicating details about the transition.
    #[prost(string, optional, tag="5")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
/// DeploymentList is a list of Deployments.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeploymentList {
    /// Standard list metadata.
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// Items is the list of Deployments.
    #[prost(message, repeated, tag="2")]
    pub items: ::prost::alloc::vec::Vec<Deployment>,
}
/// DeploymentSpec is the specification of the desired behavior of the Deployment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeploymentSpec {
    /// Number of desired pods. This is a pointer to distinguish between explicit
    /// zero and not specified. Defaults to 1.
    /// +optional
    #[prost(int32, optional, tag="1")]
    pub replicas: ::core::option::Option<i32>,
    /// Label selector for pods. Existing ReplicaSets whose pods are
    /// selected by this will be the ones affected by this deployment.
    /// It must match the pod template's labels.
    #[prost(message, optional, tag="2")]
    pub selector: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::LabelSelector>,
    /// Template describes the pods that will be created.
    #[prost(message, optional, tag="3")]
    pub template: ::core::option::Option<super::super::core::v1::PodTemplateSpec>,
    /// The deployment strategy to use to replace existing pods with new ones.
    /// +optional
    /// +patchStrategy=retainKeys
    #[prost(message, optional, tag="4")]
    pub strategy: ::core::option::Option<DeploymentStrategy>,
    /// Minimum number of seconds for which a newly created pod should be ready
    /// without any of its container crashing, for it to be considered available.
    /// Defaults to 0 (pod will be considered available as soon as it is ready)
    /// +optional
    #[prost(int32, optional, tag="5")]
    pub min_ready_seconds: ::core::option::Option<i32>,
    /// The number of old ReplicaSets to retain to allow rollback.
    /// This is a pointer to distinguish between explicit zero and not specified.
    /// Defaults to 10.
    /// +optional
    #[prost(int32, optional, tag="6")]
    pub revision_history_limit: ::core::option::Option<i32>,
    /// Indicates that the deployment is paused.
    /// +optional
    #[prost(bool, optional, tag="7")]
    pub paused: ::core::option::Option<bool>,
    /// The maximum time in seconds for a deployment to make progress before it
    /// is considered to be failed. The deployment controller will continue to
    /// process failed deployments and a condition with a ProgressDeadlineExceeded
    /// reason will be surfaced in the deployment status. Note that progress will
    /// not be estimated during the time a deployment is paused. Defaults to 600s.
    #[prost(int32, optional, tag="9")]
    pub progress_deadline_seconds: ::core::option::Option<i32>,
}
/// DeploymentStatus is the most recently observed status of the Deployment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeploymentStatus {
    /// The generation observed by the deployment controller.
    /// +optional
    #[prost(int64, optional, tag="1")]
    pub observed_generation: ::core::option::Option<i64>,
    /// Total number of non-terminated pods targeted by this deployment (their labels match the selector).
    /// +optional
    #[prost(int32, optional, tag="2")]
    pub replicas: ::core::option::Option<i32>,
    /// Total number of non-terminated pods targeted by this deployment that have the desired template spec.
    /// +optional
    #[prost(int32, optional, tag="3")]
    pub updated_replicas: ::core::option::Option<i32>,
    /// Total number of ready pods targeted by this deployment.
    /// +optional
    #[prost(int32, optional, tag="7")]
    pub ready_replicas: ::core::option::Option<i32>,
    /// Total number of available pods (ready for at least minReadySeconds) targeted by this deployment.
    /// +optional
    #[prost(int32, optional, tag="4")]
    pub available_replicas: ::core::option::Option<i32>,
    /// Total number of unavailable pods targeted by this deployment. This is the total number of
    /// pods that are still required for the deployment to have 100% available capacity. They may
    /// either be pods that are running but not yet available or pods that still have not been created.
    /// +optional
    #[prost(int32, optional, tag="5")]
    pub unavailable_replicas: ::core::option::Option<i32>,
    /// Represents the latest available observations of a deployment's current state.
    /// +patchMergeKey=type
    /// +patchStrategy=merge
    #[prost(message, repeated, tag="6")]
    pub conditions: ::prost::alloc::vec::Vec<DeploymentCondition>,
    /// Count of hash collisions for the Deployment. The Deployment controller uses this
    /// field as a collision avoidance mechanism when it needs to create the name for the
    /// newest ReplicaSet.
    /// +optional
    #[prost(int32, optional, tag="8")]
    pub collision_count: ::core::option::Option<i32>,
}
/// DeploymentStrategy describes how to replace existing pods with new ones.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeploymentStrategy {
    /// Type of deployment. Can be "Recreate" or "RollingUpdate". Default is RollingUpdate.
    /// +optional
    #[prost(string, optional, tag="1")]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
    /// Rolling update config params. Present only if DeploymentStrategyType =
    /// RollingUpdate.
    /// ---
    /// TODO: Update this to follow our convention for oneOf, whatever we decide it
    /// to be.
    /// +optional
    #[prost(message, optional, tag="2")]
    pub rolling_update: ::core::option::Option<RollingUpdateDeployment>,
}
/// ReplicaSet ensures that a specified number of pod replicas are running at any given time.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplicaSet {
    /// If the Labels of a ReplicaSet are empty, they are defaulted to
    /// be the same as the Pod(s) that the ReplicaSet manages.
    /// Standard object's metadata.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// Spec defines the specification of the desired behavior of the ReplicaSet.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
    /// +optional
    #[prost(message, optional, tag="2")]
    pub spec: ::core::option::Option<ReplicaSetSpec>,
    /// Status is the most recently observed status of the ReplicaSet.
    /// This data may be out of date by some window of time.
    /// Populated by the system.
    /// Read-only.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
    /// +optional
    #[prost(message, optional, tag="3")]
    pub status: ::core::option::Option<ReplicaSetStatus>,
}
/// ReplicaSetCondition describes the state of a replica set at a certain point.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplicaSetCondition {
    /// Type of replica set condition.
    #[prost(string, optional, tag="1")]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
    /// Status of the condition, one of True, False, Unknown.
    #[prost(string, optional, tag="2")]
    pub status: ::core::option::Option<::prost::alloc::string::String>,
    /// The last time the condition transitioned from one status to another.
    /// +optional
    #[prost(message, optional, tag="3")]
    pub last_transition_time: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::Time>,
    /// The reason for the condition's last transition.
    /// +optional
    #[prost(string, optional, tag="4")]
    pub reason: ::core::option::Option<::prost::alloc::string::String>,
    /// A human readable message indicating details about the transition.
    /// +optional
    #[prost(string, optional, tag="5")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
/// ReplicaSetList is a collection of ReplicaSets.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplicaSetList {
    /// Standard list metadata.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// List of ReplicaSets.
    /// More info: https://kubernetes.io/docs/concepts/workloads/controllers/replicationcontroller
    #[prost(message, repeated, tag="2")]
    pub items: ::prost::alloc::vec::Vec<ReplicaSet>,
}
/// ReplicaSetSpec is the specification of a ReplicaSet.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplicaSetSpec {
    /// Replicas is the number of desired replicas.
    /// This is a pointer to distinguish between explicit zero and unspecified.
    /// Defaults to 1.
    /// More info: https://kubernetes.io/docs/concepts/workloads/controllers/replicationcontroller/#what-is-a-replicationcontroller
    /// +optional
    #[prost(int32, optional, tag="1")]
    pub replicas: ::core::option::Option<i32>,
    /// Minimum number of seconds for which a newly created pod should be ready
    /// without any of its container crashing, for it to be considered available.
    /// Defaults to 0 (pod will be considered available as soon as it is ready)
    /// +optional
    #[prost(int32, optional, tag="4")]
    pub min_ready_seconds: ::core::option::Option<i32>,
    /// Selector is a label query over pods that should match the replica count.
    /// Label keys and values that must match in order to be controlled by this replica set.
    /// It must match the pod template's labels.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/#label-selectors
    #[prost(message, optional, tag="2")]
    pub selector: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::LabelSelector>,
    /// Template is the object that describes the pod that will be created if
    /// insufficient replicas are detected.
    /// More info: https://kubernetes.io/docs/concepts/workloads/controllers/replicationcontroller#pod-template
    /// +optional
    #[prost(message, optional, tag="3")]
    pub template: ::core::option::Option<super::super::core::v1::PodTemplateSpec>,
}
/// ReplicaSetStatus represents the current status of a ReplicaSet.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplicaSetStatus {
    /// Replicas is the most recently oberved number of replicas.
    /// More info: https://kubernetes.io/docs/concepts/workloads/controllers/replicationcontroller/#what-is-a-replicationcontroller
    #[prost(int32, optional, tag="1")]
    pub replicas: ::core::option::Option<i32>,
    /// The number of pods that have labels matching the labels of the pod template of the replicaset.
    /// +optional
    #[prost(int32, optional, tag="2")]
    pub fully_labeled_replicas: ::core::option::Option<i32>,
    /// The number of ready replicas for this replica set.
    /// +optional
    #[prost(int32, optional, tag="4")]
    pub ready_replicas: ::core::option::Option<i32>,
    /// The number of available replicas (ready for at least minReadySeconds) for this replica set.
    /// +optional
    #[prost(int32, optional, tag="5")]
    pub available_replicas: ::core::option::Option<i32>,
    /// ObservedGeneration reflects the generation of the most recently observed ReplicaSet.
    /// +optional
    #[prost(int64, optional, tag="3")]
    pub observed_generation: ::core::option::Option<i64>,
    /// Represents the latest available observations of a replica set's current state.
    /// +optional
    /// +patchMergeKey=type
    /// +patchStrategy=merge
    #[prost(message, repeated, tag="6")]
    pub conditions: ::prost::alloc::vec::Vec<ReplicaSetCondition>,
}
/// Spec to control the desired behavior of daemon set rolling update.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RollingUpdateDaemonSet {
    /// The maximum number of DaemonSet pods that can be unavailable during the
    /// update. Value can be an absolute number (ex: 5) or a percentage of total
    /// number of DaemonSet pods at the start of the update (ex: 10%). Absolute
    /// number is calculated from percentage by rounding up.
    /// This cannot be 0 if MaxSurge is 0
    /// Default value is 1.
    /// Example: when this is set to 30%, at most 30% of the total number of nodes
    /// that should be running the daemon pod (i.e. status.desiredNumberScheduled)
    /// can have their pods stopped for an update at any given time. The update
    /// starts by stopping at most 30% of those DaemonSet pods and then brings
    /// up new DaemonSet pods in their place. Once the new pods are available,
    /// it then proceeds onto other DaemonSet pods, thus ensuring that at least
    /// 70% of original number of DaemonSet pods are available at all times during
    /// the update.
    /// +optional
    #[prost(message, optional, tag="1")]
    pub max_unavailable: ::core::option::Option<super::super::super::apimachinery::pkg::util::intstr::IntOrString>,
    /// The maximum number of nodes with an existing available DaemonSet pod that
    /// can have an updated DaemonSet pod during during an update.
    /// Value can be an absolute number (ex: 5) or a percentage of desired pods (ex: 10%).
    /// This can not be 0 if MaxUnavailable is 0.
    /// Absolute number is calculated from percentage by rounding up to a minimum of 1.
    /// Default value is 0.
    /// Example: when this is set to 30%, at most 30% of the total number of nodes
    /// that should be running the daemon pod (i.e. status.desiredNumberScheduled)
    /// can have their a new pod created before the old pod is marked as deleted.
    /// The update starts by launching new pods on 30% of nodes. Once an updated
    /// pod is available (Ready for at least minReadySeconds) the old DaemonSet pod
    /// on that node is marked deleted. If the old pod becomes unavailable for any
    /// reason (Ready transitions to false, is evicted, or is drained) an updated
    /// pod is immediatedly created on that node without considering surge limits.
    /// Allowing surge implies the possibility that the resources consumed by the
    /// daemonset on any given node can double if the readiness check fails, and
    /// so resource intensive daemonsets should take into account that they may
    /// cause evictions during disruption.
    /// This is beta field and enabled/disabled by DaemonSetUpdateSurge feature gate.
    /// +optional
    #[prost(message, optional, tag="2")]
    pub max_surge: ::core::option::Option<super::super::super::apimachinery::pkg::util::intstr::IntOrString>,
}
/// Spec to control the desired behavior of rolling update.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RollingUpdateDeployment {
    /// The maximum number of pods that can be unavailable during the update.
    /// Value can be an absolute number (ex: 5) or a percentage of desired pods (ex: 10%).
    /// Absolute number is calculated from percentage by rounding down.
    /// This can not be 0 if MaxSurge is 0.
    /// Defaults to 25%.
    /// Example: when this is set to 30%, the old ReplicaSet can be scaled down to 70% of desired pods
    /// immediately when the rolling update starts. Once new pods are ready, old ReplicaSet
    /// can be scaled down further, followed by scaling up the new ReplicaSet, ensuring
    /// that the total number of pods available at all times during the update is at
    /// least 70% of desired pods.
    /// +optional
    #[prost(message, optional, tag="1")]
    pub max_unavailable: ::core::option::Option<super::super::super::apimachinery::pkg::util::intstr::IntOrString>,
    /// The maximum number of pods that can be scheduled above the desired number of
    /// pods.
    /// Value can be an absolute number (ex: 5) or a percentage of desired pods (ex: 10%).
    /// This can not be 0 if MaxUnavailable is 0.
    /// Absolute number is calculated from percentage by rounding up.
    /// Defaults to 25%.
    /// Example: when this is set to 30%, the new ReplicaSet can be scaled up immediately when
    /// the rolling update starts, such that the total number of old and new pods do not exceed
    /// 130% of desired pods. Once old pods have been killed,
    /// new ReplicaSet can be scaled up further, ensuring that total number of pods running
    /// at any time during the update is at most 130% of desired pods.
    /// +optional
    #[prost(message, optional, tag="2")]
    pub max_surge: ::core::option::Option<super::super::super::apimachinery::pkg::util::intstr::IntOrString>,
}
/// RollingUpdateStatefulSetStrategy is used to communicate parameter for RollingUpdateStatefulSetStrategyType.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RollingUpdateStatefulSetStrategy {
    /// Partition indicates the ordinal at which the StatefulSet should be
    /// partitioned.
    /// Default value is 0.
    /// +optional
    #[prost(int32, optional, tag="1")]
    pub partition: ::core::option::Option<i32>,
}
/// StatefulSet represents a set of pods with consistent identities.
/// Identities are defined as:
///  - Network: A single stable DNS and hostname.
///  - Storage: As many VolumeClaims as requested.
/// The StatefulSet guarantees that a given network identity will always
/// map to the same storage identity.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatefulSet {
    /// Standard object's metadata.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// Spec defines the desired identities of pods in this set.
    /// +optional
    #[prost(message, optional, tag="2")]
    pub spec: ::core::option::Option<StatefulSetSpec>,
    /// Status is the current status of Pods in this StatefulSet. This data
    /// may be out of date by some window of time.
    /// +optional
    #[prost(message, optional, tag="3")]
    pub status: ::core::option::Option<StatefulSetStatus>,
}
/// StatefulSetCondition describes the state of a statefulset at a certain point.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatefulSetCondition {
    /// Type of statefulset condition.
    #[prost(string, optional, tag="1")]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
    /// Status of the condition, one of True, False, Unknown.
    #[prost(string, optional, tag="2")]
    pub status: ::core::option::Option<::prost::alloc::string::String>,
    /// Last time the condition transitioned from one status to another.
    /// +optional
    #[prost(message, optional, tag="3")]
    pub last_transition_time: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::Time>,
    /// The reason for the condition's last transition.
    /// +optional
    #[prost(string, optional, tag="4")]
    pub reason: ::core::option::Option<::prost::alloc::string::String>,
    /// A human readable message indicating details about the transition.
    /// +optional
    #[prost(string, optional, tag="5")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
/// StatefulSetList is a collection of StatefulSets.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatefulSetList {
    /// Standard list's metadata.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// Items is the list of stateful sets.
    #[prost(message, repeated, tag="2")]
    pub items: ::prost::alloc::vec::Vec<StatefulSet>,
}
/// A StatefulSetSpec is the specification of a StatefulSet.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatefulSetSpec {
    /// replicas is the desired number of replicas of the given Template.
    /// These are replicas in the sense that they are instantiations of the
    /// same Template, but individual replicas also have a consistent identity.
    /// If unspecified, defaults to 1.
    /// TODO: Consider a rename of this field.
    /// +optional
    #[prost(int32, optional, tag="1")]
    pub replicas: ::core::option::Option<i32>,
    /// selector is a label query over pods that should match the replica count.
    /// It must match the pod template's labels.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/#label-selectors
    #[prost(message, optional, tag="2")]
    pub selector: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::LabelSelector>,
    /// template is the object that describes the pod that will be created if
    /// insufficient replicas are detected. Each pod stamped out by the StatefulSet
    /// will fulfill this Template, but have a unique identity from the rest
    /// of the StatefulSet.
    #[prost(message, optional, tag="3")]
    pub template: ::core::option::Option<super::super::core::v1::PodTemplateSpec>,
    /// volumeClaimTemplates is a list of claims that pods are allowed to reference.
    /// The StatefulSet controller is responsible for mapping network identities to
    /// claims in a way that maintains the identity of a pod. Every claim in
    /// this list must have at least one matching (by name) volumeMount in one
    /// container in the template. A claim in this list takes precedence over
    /// any volumes in the template, with the same name.
    /// TODO: Define the behavior if a claim already exists with the same name.
    /// +optional
    #[prost(message, repeated, tag="4")]
    pub volume_claim_templates: ::prost::alloc::vec::Vec<super::super::core::v1::PersistentVolumeClaim>,
    /// serviceName is the name of the service that governs this StatefulSet.
    /// This service must exist before the StatefulSet, and is responsible for
    /// the network identity of the set. Pods get DNS/hostnames that follow the
    /// pattern: pod-specific-string.serviceName.default.svc.cluster.local
    /// where "pod-specific-string" is managed by the StatefulSet controller.
    #[prost(string, optional, tag="5")]
    pub service_name: ::core::option::Option<::prost::alloc::string::String>,
    /// podManagementPolicy controls how pods are created during initial scale up,
    /// when replacing pods on nodes, or when scaling down. The default policy is
    /// `OrderedReady`, where pods are created in increasing order (pod-0, then
    /// pod-1, etc) and the controller will wait until each pod is ready before
    /// continuing. When scaling down, the pods are removed in the opposite order.
    /// The alternative policy is `Parallel` which will create pods in parallel
    /// to match the desired scale without waiting, and on scale down will delete
    /// all pods at once.
    /// +optional
    #[prost(string, optional, tag="6")]
    pub pod_management_policy: ::core::option::Option<::prost::alloc::string::String>,
    /// updateStrategy indicates the StatefulSetUpdateStrategy that will be
    /// employed to update Pods in the StatefulSet when a revision is made to
    /// Template.
    #[prost(message, optional, tag="7")]
    pub update_strategy: ::core::option::Option<StatefulSetUpdateStrategy>,
    /// revisionHistoryLimit is the maximum number of revisions that will
    /// be maintained in the StatefulSet's revision history. The revision history
    /// consists of all revisions not represented by a currently applied
    /// StatefulSetSpec version. The default value is 10.
    #[prost(int32, optional, tag="8")]
    pub revision_history_limit: ::core::option::Option<i32>,
    /// Minimum number of seconds for which a newly created pod should be ready
    /// without any of its container crashing for it to be considered available.
    /// Defaults to 0 (pod will be considered available as soon as it is ready)
    /// This is an alpha field and requires enabling StatefulSetMinReadySeconds feature gate.
    /// +optional
    #[prost(int32, optional, tag="9")]
    pub min_ready_seconds: ::core::option::Option<i32>,
}
/// StatefulSetStatus represents the current state of a StatefulSet.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatefulSetStatus {
    /// observedGeneration is the most recent generation observed for this StatefulSet. It corresponds to the
    /// StatefulSet's generation, which is updated on mutation by the API Server.
    /// +optional
    #[prost(int64, optional, tag="1")]
    pub observed_generation: ::core::option::Option<i64>,
    /// replicas is the number of Pods created by the StatefulSet controller.
    #[prost(int32, optional, tag="2")]
    pub replicas: ::core::option::Option<i32>,
    /// readyReplicas is the number of Pods created by the StatefulSet controller that have a Ready Condition.
    #[prost(int32, optional, tag="3")]
    pub ready_replicas: ::core::option::Option<i32>,
    /// currentReplicas is the number of Pods created by the StatefulSet controller from the StatefulSet version
    /// indicated by currentRevision.
    #[prost(int32, optional, tag="4")]
    pub current_replicas: ::core::option::Option<i32>,
    /// updatedReplicas is the number of Pods created by the StatefulSet controller from the StatefulSet version
    /// indicated by updateRevision.
    #[prost(int32, optional, tag="5")]
    pub updated_replicas: ::core::option::Option<i32>,
    /// currentRevision, if not empty, indicates the version of the StatefulSet used to generate Pods in the
    /// sequence [0,currentReplicas).
    #[prost(string, optional, tag="6")]
    pub current_revision: ::core::option::Option<::prost::alloc::string::String>,
    /// updateRevision, if not empty, indicates the version of the StatefulSet used to generate Pods in the sequence
    /// [replicas-updatedReplicas,replicas)
    #[prost(string, optional, tag="7")]
    pub update_revision: ::core::option::Option<::prost::alloc::string::String>,
    /// collisionCount is the count of hash collisions for the StatefulSet. The StatefulSet controller
    /// uses this field as a collision avoidance mechanism when it needs to create the name for the
    /// newest ControllerRevision.
    /// +optional
    #[prost(int32, optional, tag="9")]
    pub collision_count: ::core::option::Option<i32>,
    /// Represents the latest available observations of a statefulset's current state.
    /// +optional
    /// +patchMergeKey=type
    /// +patchStrategy=merge
    #[prost(message, repeated, tag="10")]
    pub conditions: ::prost::alloc::vec::Vec<StatefulSetCondition>,
    /// Total number of available pods (ready for at least minReadySeconds) targeted by this statefulset.
    /// This is an alpha field and requires enabling StatefulSetMinReadySeconds feature gate.
    /// Remove omitempty when graduating to beta
    /// +optional
    #[prost(int32, optional, tag="11")]
    pub available_replicas: ::core::option::Option<i32>,
}
/// StatefulSetUpdateStrategy indicates the strategy that the StatefulSet
/// controller will use to perform updates. It includes any additional parameters
/// necessary to perform the update for the indicated strategy.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatefulSetUpdateStrategy {
    /// Type indicates the type of the StatefulSetUpdateStrategy.
    /// Default is RollingUpdate.
    /// +optional
    #[prost(string, optional, tag="1")]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
    /// RollingUpdate is used to communicate parameters when Type is RollingUpdateStatefulSetStrategyType.
    /// +optional
    #[prost(message, optional, tag="2")]
    pub rolling_update: ::core::option::Option<RollingUpdateStatefulSetStrategy>,
}
