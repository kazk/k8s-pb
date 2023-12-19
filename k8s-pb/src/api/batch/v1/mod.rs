/// CronJob represents the configuration of a single cron job.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CronJob {
    /// Standard object's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    >,
    /// Specification of the desired behavior of a cron job, including the schedule.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag = "2")]
    pub spec: ::core::option::Option<CronJobSpec>,
    /// Current status of a cron job.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag = "3")]
    pub status: ::core::option::Option<CronJobStatus>,
}
/// CronJobList is a collection of cron jobs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CronJobList {
    /// Standard list metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta,
    >,
    /// items is the list of CronJobs.
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<CronJob>,
}
/// CronJobSpec describes how the job execution will look like and when it will actually run.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CronJobSpec {
    /// The schedule in Cron format, see <https://en.wikipedia.org/wiki/Cron.>
    #[prost(string, optional, tag = "1")]
    pub schedule: ::core::option::Option<::prost::alloc::string::String>,
    /// The time zone name for the given schedule, see <https://en.wikipedia.org/wiki/List_of_tz_database_time_zones.>
    /// If not specified, this will default to the time zone of the kube-controller-manager process.
    /// The set of valid time zone names and the time zone offset is loaded from the system-wide time zone
    /// database by the API server during CronJob validation and the controller manager during execution.
    /// If no system-wide time zone database can be found a bundled version of the database is used instead.
    /// If the time zone name becomes invalid during the lifetime of a CronJob or due to a change in host
    /// configuration, the controller will stop creating new new Jobs and will create a system event with the
    /// reason UnknownTimeZone.
    /// More information can be found in <https://kubernetes.io/docs/concepts/workloads/controllers/cron-jobs/#time-zones>
    /// +optional
    #[prost(string, optional, tag = "8")]
    pub time_zone: ::core::option::Option<::prost::alloc::string::String>,
    /// Optional deadline in seconds for starting the job if it misses scheduled
    /// time for any reason.  Missed jobs executions will be counted as failed ones.
    /// +optional
    #[prost(int64, optional, tag = "2")]
    pub starting_deadline_seconds: ::core::option::Option<i64>,
    /// Specifies how to treat concurrent executions of a Job.
    /// Valid values are:
    ///
    /// - "Allow" (default): allows CronJobs to run concurrently;
    /// - "Forbid": forbids concurrent runs, skipping next run if previous run hasn't finished yet;
    /// - "Replace": cancels currently running job and replaces it with a new one
    /// +optional
    #[prost(string, optional, tag = "3")]
    pub concurrency_policy: ::core::option::Option<::prost::alloc::string::String>,
    /// This flag tells the controller to suspend subsequent executions, it does
    /// not apply to already started executions.  Defaults to false.
    /// +optional
    #[prost(bool, optional, tag = "4")]
    pub suspend: ::core::option::Option<bool>,
    /// Specifies the job that will be created when executing a CronJob.
    #[prost(message, optional, tag = "5")]
    pub job_template: ::core::option::Option<JobTemplateSpec>,
    /// The number of successful finished jobs to retain. Value must be non-negative integer.
    /// Defaults to 3.
    /// +optional
    #[prost(int32, optional, tag = "6")]
    pub successful_jobs_history_limit: ::core::option::Option<i32>,
    /// The number of failed finished jobs to retain. Value must be non-negative integer.
    /// Defaults to 1.
    /// +optional
    #[prost(int32, optional, tag = "7")]
    pub failed_jobs_history_limit: ::core::option::Option<i32>,
}
/// CronJobStatus represents the current state of a cron job.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CronJobStatus {
    /// A list of pointers to currently running jobs.
    /// +optional
    /// +listType=atomic
    #[prost(message, repeated, tag = "1")]
    pub active: ::prost::alloc::vec::Vec<super::super::core::v1::ObjectReference>,
    /// Information when was the last time the job was successfully scheduled.
    /// +optional
    #[prost(message, optional, tag = "4")]
    pub last_schedule_time: ::core::option::Option<
        super::super::super::apimachinery::pkg::apis::meta::v1::Time,
    >,
    /// Information when was the last time the job successfully completed.
    /// +optional
    #[prost(message, optional, tag = "5")]
    pub last_successful_time: ::core::option::Option<
        super::super::super::apimachinery::pkg::apis::meta::v1::Time,
    >,
}
/// Job represents the configuration of a single job.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Job {
    /// Standard object's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    >,
    /// Specification of the desired behavior of a job.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag = "2")]
    pub spec: ::core::option::Option<JobSpec>,
    /// Current status of a job.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag = "3")]
    pub status: ::core::option::Option<JobStatus>,
}
/// JobCondition describes current state of a job.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobCondition {
    /// Type of job condition, Complete or Failed.
    #[prost(string, optional, tag = "1")]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
    /// Status of the condition, one of True, False, Unknown.
    #[prost(string, optional, tag = "2")]
    pub status: ::core::option::Option<::prost::alloc::string::String>,
    /// Last time the condition was checked.
    /// +optional
    #[prost(message, optional, tag = "3")]
    pub last_probe_time: ::core::option::Option<
        super::super::super::apimachinery::pkg::apis::meta::v1::Time,
    >,
    /// Last time the condition transit from one status to another.
    /// +optional
    #[prost(message, optional, tag = "4")]
    pub last_transition_time: ::core::option::Option<
        super::super::super::apimachinery::pkg::apis::meta::v1::Time,
    >,
    /// (brief) reason for the condition's last transition.
    /// +optional
    #[prost(string, optional, tag = "5")]
    pub reason: ::core::option::Option<::prost::alloc::string::String>,
    /// Human readable message indicating details about last transition.
    /// +optional
    #[prost(string, optional, tag = "6")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
/// JobList is a collection of jobs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobList {
    /// Standard list metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta,
    >,
    /// items is the list of Jobs.
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<Job>,
}
/// JobSpec describes how the job execution will look like.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobSpec {
    /// Specifies the maximum desired number of pods the job should
    /// run at any given time. The actual number of pods running in steady state will
    /// be less than this number when ((.spec.completions - .status.successful) < .spec.parallelism),
    /// i.e. when the work left to do is less than max parallelism.
    /// More info: <https://kubernetes.io/docs/concepts/workloads/controllers/jobs-run-to-completion/>
    /// +optional
    #[prost(int32, optional, tag = "1")]
    pub parallelism: ::core::option::Option<i32>,
    /// Specifies the desired number of successfully finished pods the
    /// job should be run with.  Setting to null means that the success of any
    /// pod signals the success of all pods, and allows parallelism to have any positive
    /// value.  Setting to 1 means that parallelism is limited to 1 and the success of that
    /// pod signals the success of the job.
    /// More info: <https://kubernetes.io/docs/concepts/workloads/controllers/jobs-run-to-completion/>
    /// +optional
    #[prost(int32, optional, tag = "2")]
    pub completions: ::core::option::Option<i32>,
    /// Specifies the duration in seconds relative to the startTime that the job
    /// may be continuously active before the system tries to terminate it; value
    /// must be positive integer. If a Job is suspended (at creation or through an
    /// update), this timer will effectively be stopped and reset when the Job is
    /// resumed again.
    /// +optional
    #[prost(int64, optional, tag = "3")]
    pub active_deadline_seconds: ::core::option::Option<i64>,
    /// Specifies the policy of handling failed pods. In particular, it allows to
    /// specify the set of actions and conditions which need to be
    /// satisfied to take the associated action.
    /// If empty, the default behaviour applies - the counter of failed pods,
    /// represented by the jobs's .status.failed field, is incremented and it is
    /// checked against the backoffLimit. This field cannot be used in combination
    /// with restartPolicy=OnFailure.
    ///
    /// This field is beta-level. It can be used when the `JobPodFailurePolicy`
    /// feature gate is enabled (enabled by default).
    /// +optional
    #[prost(message, optional, tag = "11")]
    pub pod_failure_policy: ::core::option::Option<PodFailurePolicy>,
    /// Specifies the number of retries before marking this job failed.
    /// Defaults to 6
    /// +optional
    #[prost(int32, optional, tag = "7")]
    pub backoff_limit: ::core::option::Option<i32>,
    /// Specifies the limit for the number of retries within an
    /// index before marking this index as failed. When enabled the number of
    /// failures per index is kept in the pod's
    /// batch.kubernetes.io/job-index-failure-count annotation. It can only
    /// be set when Job's completionMode=Indexed, and the Pod's restart
    /// policy is Never. The field is immutable.
    /// This field is beta-level. It can be used when the `JobBackoffLimitPerIndex`
    /// feature gate is enabled (enabled by default).
    /// +optional
    #[prost(int32, optional, tag = "12")]
    pub backoff_limit_per_index: ::core::option::Option<i32>,
    /// Specifies the maximal number of failed indexes before marking the Job as
    /// failed, when backoffLimitPerIndex is set. Once the number of failed
    /// indexes exceeds this number the entire Job is marked as Failed and its
    /// execution is terminated. When left as null the job continues execution of
    /// all of its indexes and is marked with the `Complete` Job condition.
    /// It can only be specified when backoffLimitPerIndex is set.
    /// It can be null or up to completions. It is required and must be
    /// less than or equal to 10^4 when is completions greater than 10^5.
    /// This field is beta-level. It can be used when the `JobBackoffLimitPerIndex`
    /// feature gate is enabled (enabled by default).
    /// +optional
    #[prost(int32, optional, tag = "13")]
    pub max_failed_indexes: ::core::option::Option<i32>,
    /// A label query over pods that should match the pod count.
    /// Normally, the system sets this field for you.
    /// More info: <https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/#label-selectors>
    /// +optional
    #[prost(message, optional, tag = "4")]
    pub selector: ::core::option::Option<
        super::super::super::apimachinery::pkg::apis::meta::v1::LabelSelector,
    >,
    /// manualSelector controls generation of pod labels and pod selectors.
    /// Leave `manualSelector` unset unless you are certain what you are doing.
    /// When false or unset, the system pick labels unique to this job
    /// and appends those labels to the pod template.  When true,
    /// the user is responsible for picking unique labels and specifying
    /// the selector.  Failure to pick a unique label may cause this
    /// and other jobs to not function correctly.  However, You may see
    /// `manualSelector=true` in jobs that were created with the old `extensions/v1beta1`
    /// API.
    /// More info: <https://kubernetes.io/docs/concepts/workloads/controllers/jobs-run-to-completion/#specifying-your-own-pod-selector>
    /// +optional
    #[prost(bool, optional, tag = "5")]
    pub manual_selector: ::core::option::Option<bool>,
    /// Describes the pod that will be created when executing a job.
    /// The only allowed template.spec.restartPolicy values are "Never" or "OnFailure".
    /// More info: <https://kubernetes.io/docs/concepts/workloads/controllers/jobs-run-to-completion/>
    #[prost(message, optional, tag = "6")]
    pub template: ::core::option::Option<super::super::core::v1::PodTemplateSpec>,
    /// ttlSecondsAfterFinished limits the lifetime of a Job that has finished
    /// execution (either Complete or Failed). If this field is set,
    /// ttlSecondsAfterFinished after the Job finishes, it is eligible to be
    /// automatically deleted. When the Job is being deleted, its lifecycle
    /// guarantees (e.g. finalizers) will be honored. If this field is unset,
    /// the Job won't be automatically deleted. If this field is set to zero,
    /// the Job becomes eligible to be deleted immediately after it finishes.
    /// +optional
    #[prost(int32, optional, tag = "8")]
    pub ttl_seconds_after_finished: ::core::option::Option<i32>,
    /// completionMode specifies how Pod completions are tracked. It can be
    /// `NonIndexed` (default) or `Indexed`.
    ///
    /// `NonIndexed` means that the Job is considered complete when there have
    /// been .spec.completions successfully completed Pods. Each Pod completion is
    /// homologous to each other.
    ///
    /// `Indexed` means that the Pods of a
    /// Job get an associated completion index from 0 to (.spec.completions - 1),
    /// available in the annotation batch.kubernetes.io/job-completion-index.
    /// The Job is considered complete when there is one successfully completed Pod
    /// for each index.
    /// When value is `Indexed`, .spec.completions must be specified and
    /// `.spec.parallelism` must be less than or equal to 10^5.
    /// In addition, The Pod name takes the form
    /// `$(job-name)-$(index)-$(random-string)`,
    /// the Pod hostname takes the form `$(job-name)-$(index)`.
    ///
    /// More completion modes can be added in the future.
    /// If the Job controller observes a mode that it doesn't recognize, which
    /// is possible during upgrades due to version skew, the controller
    /// skips updates for the Job.
    /// +optional
    #[prost(string, optional, tag = "9")]
    pub completion_mode: ::core::option::Option<::prost::alloc::string::String>,
    /// suspend specifies whether the Job controller should create Pods or not. If
    /// a Job is created with suspend set to true, no Pods are created by the Job
    /// controller. If a Job is suspended after creation (i.e. the flag goes from
    /// false to true), the Job controller will delete all active Pods associated
    /// with this Job. Users must design their workload to gracefully handle this.
    /// Suspending a Job will reset the StartTime field of the Job, effectively
    /// resetting the ActiveDeadlineSeconds timer too. Defaults to false.
    ///
    /// +optional
    #[prost(bool, optional, tag = "10")]
    pub suspend: ::core::option::Option<bool>,
    /// podReplacementPolicy specifies when to create replacement Pods.
    /// Possible values are:
    /// - TerminatingOrFailed means that we recreate pods
    ///    when they are terminating (has a metadata.deletionTimestamp) or failed.
    /// - Failed means to wait until a previously created Pod is fully terminated (has phase
    ///    Failed or Succeeded) before creating a replacement Pod.
    ///
    /// When using podFailurePolicy, Failed is the the only allowed value.
    /// TerminatingOrFailed and Failed are allowed values when podFailurePolicy is not in use.
    /// This is an beta field. To use this, enable the JobPodReplacementPolicy feature toggle.
    /// This is on by default.
    /// +optional
    #[prost(string, optional, tag = "14")]
    pub pod_replacement_policy: ::core::option::Option<::prost::alloc::string::String>,
}
/// JobStatus represents the current state of a Job.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobStatus {
    /// The latest available observations of an object's current state. When a Job
    /// fails, one of the conditions will have type "Failed" and status true. When
    /// a Job is suspended, one of the conditions will have type "Suspended" and
    /// status true; when the Job is resumed, the status of this condition will
    /// become false. When a Job is completed, one of the conditions will have
    /// type "Complete" and status true.
    /// More info: <https://kubernetes.io/docs/concepts/workloads/controllers/jobs-run-to-completion/>
    /// +optional
    /// +patchMergeKey=type
    /// +patchStrategy=merge
    /// +listType=atomic
    #[prost(message, repeated, tag = "1")]
    pub conditions: ::prost::alloc::vec::Vec<JobCondition>,
    /// Represents time when the job controller started processing a job. When a
    /// Job is created in the suspended state, this field is not set until the
    /// first time it is resumed. This field is reset every time a Job is resumed
    /// from suspension. It is represented in RFC3339 form and is in UTC.
    /// +optional
    #[prost(message, optional, tag = "2")]
    pub start_time: ::core::option::Option<
        super::super::super::apimachinery::pkg::apis::meta::v1::Time,
    >,
    /// Represents time when the job was completed. It is not guaranteed to
    /// be set in happens-before order across separate operations.
    /// It is represented in RFC3339 form and is in UTC.
    /// The completion time is only set when the job finishes successfully.
    /// +optional
    #[prost(message, optional, tag = "3")]
    pub completion_time: ::core::option::Option<
        super::super::super::apimachinery::pkg::apis::meta::v1::Time,
    >,
    /// The number of pending and running pods.
    /// +optional
    #[prost(int32, optional, tag = "4")]
    pub active: ::core::option::Option<i32>,
    /// The number of pods which reached phase Succeeded.
    /// +optional
    #[prost(int32, optional, tag = "5")]
    pub succeeded: ::core::option::Option<i32>,
    /// The number of pods which reached phase Failed.
    /// +optional
    #[prost(int32, optional, tag = "6")]
    pub failed: ::core::option::Option<i32>,
    /// The number of pods which are terminating (in phase Pending or Running
    /// and have a deletionTimestamp).
    ///
    /// This field is beta-level. The job controller populates the field when
    /// the feature gate JobPodReplacementPolicy is enabled (enabled by default).
    /// +optional
    #[prost(int32, optional, tag = "11")]
    pub terminating: ::core::option::Option<i32>,
    /// completedIndexes holds the completed indexes when .spec.completionMode =
    /// "Indexed" in a text format. The indexes are represented as decimal integers
    /// separated by commas. The numbers are listed in increasing order. Three or
    /// more consecutive numbers are compressed and represented by the first and
    /// last element of the series, separated by a hyphen.
    /// For example, if the completed indexes are 1, 3, 4, 5 and 7, they are
    /// represented as "1,3-5,7".
    /// +optional
    #[prost(string, optional, tag = "7")]
    pub completed_indexes: ::core::option::Option<::prost::alloc::string::String>,
    /// FailedIndexes holds the failed indexes when backoffLimitPerIndex=true.
    /// The indexes are represented in the text format analogous as for the
    /// `completedIndexes` field, ie. they are kept as decimal integers
    /// separated by commas. The numbers are listed in increasing order. Three or
    /// more consecutive numbers are compressed and represented by the first and
    /// last element of the series, separated by a hyphen.
    /// For example, if the failed indexes are 1, 3, 4, 5 and 7, they are
    /// represented as "1,3-5,7".
    /// This field is beta-level. It can be used when the `JobBackoffLimitPerIndex`
    /// feature gate is enabled (enabled by default).
    /// +optional
    #[prost(string, optional, tag = "10")]
    pub failed_indexes: ::core::option::Option<::prost::alloc::string::String>,
    /// uncountedTerminatedPods holds the UIDs of Pods that have terminated but
    /// the job controller hasn't yet accounted for in the status counters.
    ///
    /// The job controller creates pods with a finalizer. When a pod terminates
    /// (succeeded or failed), the controller does three steps to account for it
    /// in the job status:
    ///
    /// 1. Add the pod UID to the arrays in this field.
    /// 2. Remove the pod finalizer.
    /// 3. Remove the pod UID from the arrays while increasing the corresponding
    ///      counter.
    ///
    /// Old jobs might not be tracked using this field, in which case the field
    /// remains null.
    /// +optional
    #[prost(message, optional, tag = "8")]
    pub uncounted_terminated_pods: ::core::option::Option<UncountedTerminatedPods>,
    /// The number of pods which have a Ready condition.
    /// +optional
    #[prost(int32, optional, tag = "9")]
    pub ready: ::core::option::Option<i32>,
}
/// JobTemplateSpec describes the data a Job should have when created from a template
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobTemplateSpec {
    /// Standard object's metadata of the jobs created from this template.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    >,
    /// Specification of the desired behavior of the job.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag = "2")]
    pub spec: ::core::option::Option<JobSpec>,
}
/// PodFailurePolicy describes how failed pods influence the backoffLimit.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PodFailurePolicy {
    /// A list of pod failure policy rules. The rules are evaluated in order.
    /// Once a rule matches a Pod failure, the remaining of the rules are ignored.
    /// When no rule matches the Pod failure, the default handling applies - the
    /// counter of pod failures is incremented and it is checked against
    /// the backoffLimit. At most 20 elements are allowed.
    /// +listType=atomic
    #[prost(message, repeated, tag = "1")]
    pub rules: ::prost::alloc::vec::Vec<PodFailurePolicyRule>,
}
/// PodFailurePolicyOnExitCodesRequirement describes the requirement for handling
/// a failed pod based on its container exit codes. In particular, it lookups the
/// .state.terminated.exitCode for each app container and init container status,
/// represented by the .status.containerStatuses and .status.initContainerStatuses
/// fields in the Pod status, respectively. Containers completed with success
/// (exit code 0) are excluded from the requirement check.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PodFailurePolicyOnExitCodesRequirement {
    /// Restricts the check for exit codes to the container with the
    /// specified name. When null, the rule applies to all containers.
    /// When specified, it should match one the container or initContainer
    /// names in the pod template.
    /// +optional
    #[prost(string, optional, tag = "1")]
    pub container_name: ::core::option::Option<::prost::alloc::string::String>,
    /// Represents the relationship between the container exit code(s) and the
    /// specified values. Containers completed with success (exit code 0) are
    /// excluded from the requirement check. Possible values are:
    ///
    /// - In: the requirement is satisfied if at least one container exit code
    ///    (might be multiple if there are multiple containers not restricted
    ///    by the 'containerName' field) is in the set of specified values.
    /// - NotIn: the requirement is satisfied if at least one container exit code
    ///    (might be multiple if there are multiple containers not restricted
    ///    by the 'containerName' field) is not in the set of specified values.
    /// Additional values are considered to be added in the future. Clients should
    /// react to an unknown operator by assuming the requirement is not satisfied.
    #[prost(string, optional, tag = "2")]
    pub operator: ::core::option::Option<::prost::alloc::string::String>,
    /// Specifies the set of values. Each returned container exit code (might be
    /// multiple in case of multiple containers) is checked against this set of
    /// values with respect to the operator. The list of values must be ordered
    /// and must not contain duplicates. Value '0' cannot be used for the In operator.
    /// At least one element is required. At most 255 elements are allowed.
    /// +listType=set
    #[prost(int32, repeated, packed = "false", tag = "3")]
    pub values: ::prost::alloc::vec::Vec<i32>,
}
/// PodFailurePolicyOnPodConditionsPattern describes a pattern for matching
/// an actual pod condition type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PodFailurePolicyOnPodConditionsPattern {
    /// Specifies the required Pod condition type. To match a pod condition
    /// it is required that specified type equals the pod condition type.
    #[prost(string, optional, tag = "1")]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
    /// Specifies the required Pod condition status. To match a pod condition
    /// it is required that the specified status equals the pod condition status.
    /// Defaults to True.
    #[prost(string, optional, tag = "2")]
    pub status: ::core::option::Option<::prost::alloc::string::String>,
}
/// PodFailurePolicyRule describes how a pod failure is handled when the requirements are met.
/// One of onExitCodes and onPodConditions, but not both, can be used in each rule.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PodFailurePolicyRule {
    /// Specifies the action taken on a pod failure when the requirements are satisfied.
    /// Possible values are:
    ///
    /// - FailJob: indicates that the pod's job is marked as Failed and all
    ///    running pods are terminated.
    /// - FailIndex: indicates that the pod's index is marked as Failed and will
    ///    not be restarted.
    ///    This value is beta-level. It can be used when the
    ///    `JobBackoffLimitPerIndex` feature gate is enabled (enabled by default).
    /// - Ignore: indicates that the counter towards the .backoffLimit is not
    ///    incremented and a replacement pod is created.
    /// - Count: indicates that the pod is handled in the default way - the
    ///    counter towards the .backoffLimit is incremented.
    /// Additional values are considered to be added in the future. Clients should
    /// react to an unknown action by skipping the rule.
    #[prost(string, optional, tag = "1")]
    pub action: ::core::option::Option<::prost::alloc::string::String>,
    /// Represents the requirement on the container exit codes.
    /// +optional
    #[prost(message, optional, tag = "2")]
    pub on_exit_codes: ::core::option::Option<PodFailurePolicyOnExitCodesRequirement>,
    /// Represents the requirement on the pod conditions. The requirement is represented
    /// as a list of pod condition patterns. The requirement is satisfied if at
    /// least one pattern matches an actual pod condition. At most 20 elements are allowed.
    /// +listType=atomic
    /// +optional
    #[prost(message, repeated, tag = "3")]
    pub on_pod_conditions: ::prost::alloc::vec::Vec<
        PodFailurePolicyOnPodConditionsPattern,
    >,
}
/// UncountedTerminatedPods holds UIDs of Pods that have terminated but haven't
/// been accounted in Job status counters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UncountedTerminatedPods {
    /// succeeded holds UIDs of succeeded Pods.
    /// +listType=set
    /// +optional
    #[prost(string, repeated, tag = "1")]
    pub succeeded: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// failed holds UIDs of failed Pods.
    /// +listType=set
    /// +optional
    #[prost(string, repeated, tag = "2")]
    pub failed: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}

impl crate::Resource for CronJob {
    const API_VERSION: &'static str = "batch/v1";
    const GROUP: &'static str = "batch";
    const VERSION: &'static str = "v1";
    const KIND: &'static str = "CronJob";
    const NAME: &'static str = "cronjobs";
}
impl crate::HasMetadata for CronJob {
    type Metadata = crate::apimachinery::pkg::apis::meta::v1::ObjectMeta;
    fn metadata(&self) -> Option<&<Self as crate::HasMetadata>::Metadata> {
        self.metadata.as_ref()
    }
    fn metadata_mut(&mut self) -> Option<&mut <Self as crate::HasMetadata>::Metadata> {
        self.metadata.as_mut()
    }
}
impl crate::HasSpec for CronJob {
    type Spec = crate::api::batch::v1::CronJobSpec;
    fn spec(&self) -> Option<&<Self as crate::HasSpec>::Spec> {
        self.spec.as_ref()
    }
    fn spec_mut(&mut self) -> Option<&mut <Self as crate::HasSpec>::Spec> {
        self.spec.as_mut()
    }
}
impl crate::HasStatus for CronJob {
    type Status = crate::api::batch::v1::CronJobStatus;
    fn status(&self) -> Option<&<Self as crate::HasStatus>::Status> {
        self.status.as_ref()
    }
    fn status_mut(&mut self) -> Option<&mut <Self as crate::HasStatus>::Status> {
        self.status.as_mut()
    }
}


impl crate::Resource for Job {
    const API_VERSION: &'static str = "batch/v1";
    const GROUP: &'static str = "batch";
    const VERSION: &'static str = "v1";
    const KIND: &'static str = "Job";
    const NAME: &'static str = "jobs";
}
impl crate::HasMetadata for Job {
    type Metadata = crate::apimachinery::pkg::apis::meta::v1::ObjectMeta;
    fn metadata(&self) -> Option<&<Self as crate::HasMetadata>::Metadata> {
        self.metadata.as_ref()
    }
    fn metadata_mut(&mut self) -> Option<&mut <Self as crate::HasMetadata>::Metadata> {
        self.metadata.as_mut()
    }
}
impl crate::HasSpec for Job {
    type Spec = crate::api::batch::v1::JobSpec;
    fn spec(&self) -> Option<&<Self as crate::HasSpec>::Spec> {
        self.spec.as_ref()
    }
    fn spec_mut(&mut self) -> Option<&mut <Self as crate::HasSpec>::Spec> {
        self.spec.as_mut()
    }
}
impl crate::HasStatus for Job {
    type Status = crate::api::batch::v1::JobStatus;
    fn status(&self) -> Option<&<Self as crate::HasStatus>::Status> {
        self.status.as_ref()
    }
    fn status_mut(&mut self) -> Option<&mut <Self as crate::HasStatus>::Status> {
        self.status.as_mut()
    }
}
impl crate::HasConditions for Job {
    type Condition = crate::api::batch::v1::JobCondition;
    fn conditions(&self) -> Option<&[<Self as crate::HasConditions>::Condition]> {
        self.status.as_ref().map(|s| s.conditions.as_slice())
    }
    fn conditions_mut(&mut self) -> Option<&mut Vec<<Self as crate::HasConditions>::Condition>> {
        self.status
            .as_mut()
            .and_then(|s| Some(s.conditions.as_mut()))
    }
}

