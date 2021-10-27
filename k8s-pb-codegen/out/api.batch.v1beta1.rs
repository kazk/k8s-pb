/// CronJob represents the configuration of a single cron job.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CronJob {
    /// Standard object's metadata.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// Specification of the desired behavior of a cron job, including the schedule.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
    /// +optional
    #[prost(message, optional, tag="2")]
    pub spec: ::core::option::Option<CronJobSpec>,
    /// Current status of a cron job.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
    /// +optional
    #[prost(message, optional, tag="3")]
    pub status: ::core::option::Option<CronJobStatus>,
}
/// CronJobList is a collection of cron jobs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CronJobList {
    /// Standard list metadata.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// items is the list of CronJobs.
    #[prost(message, repeated, tag="2")]
    pub items: ::prost::alloc::vec::Vec<CronJob>,
}
/// CronJobSpec describes how the job execution will look like and when it will actually run.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CronJobSpec {
    /// The schedule in Cron format, see https://en.wikipedia.org/wiki/Cron.
    #[prost(string, optional, tag="1")]
    pub schedule: ::core::option::Option<::prost::alloc::string::String>,
    /// Optional deadline in seconds for starting the job if it misses scheduled
    /// time for any reason.  Missed jobs executions will be counted as failed ones.
    /// +optional
    #[prost(int64, optional, tag="2")]
    pub starting_deadline_seconds: ::core::option::Option<i64>,
    /// Specifies how to treat concurrent executions of a Job.
    /// Valid values are:
    /// - "Allow" (default): allows CronJobs to run concurrently;
    /// - "Forbid": forbids concurrent runs, skipping next run if previous run hasn't finished yet;
    /// - "Replace": cancels currently running job and replaces it with a new one
    /// +optional
    #[prost(string, optional, tag="3")]
    pub concurrency_policy: ::core::option::Option<::prost::alloc::string::String>,
    /// This flag tells the controller to suspend subsequent executions, it does
    /// not apply to already started executions.  Defaults to false.
    /// +optional
    #[prost(bool, optional, tag="4")]
    pub suspend: ::core::option::Option<bool>,
    /// Specifies the job that will be created when executing a CronJob.
    #[prost(message, optional, tag="5")]
    pub job_template: ::core::option::Option<JobTemplateSpec>,
    /// The number of successful finished jobs to retain.
    /// This is a pointer to distinguish between explicit zero and not specified.
    /// Defaults to 3.
    /// +optional
    #[prost(int32, optional, tag="6")]
    pub successful_jobs_history_limit: ::core::option::Option<i32>,
    /// The number of failed finished jobs to retain.
    /// This is a pointer to distinguish between explicit zero and not specified.
    /// Defaults to 1.
    /// +optional
    #[prost(int32, optional, tag="7")]
    pub failed_jobs_history_limit: ::core::option::Option<i32>,
}
/// CronJobStatus represents the current state of a cron job.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CronJobStatus {
    /// A list of pointers to currently running jobs.
    /// +optional
    /// +listType=atomic
    #[prost(message, repeated, tag="1")]
    pub active: ::prost::alloc::vec::Vec<super::super::core::v1::ObjectReference>,
    /// Information when was the last time the job was successfully scheduled.
    /// +optional
    #[prost(message, optional, tag="4")]
    pub last_schedule_time: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::Time>,
    /// Information when was the last time the job successfully completed.
    /// +optional
    #[prost(message, optional, tag="5")]
    pub last_successful_time: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::Time>,
}
/// JobTemplate describes a template for creating copies of a predefined pod.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobTemplate {
    /// Standard object's metadata.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// Defines jobs that will be created from this template.
    /// https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
    /// +optional
    #[prost(message, optional, tag="2")]
    pub template: ::core::option::Option<JobTemplateSpec>,
}
/// JobTemplateSpec describes the data a Job should have when created from a template
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobTemplateSpec {
    /// Standard object's metadata of the jobs created from this template.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// Specification of the desired behavior of the job.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
    /// +optional
    #[prost(message, optional, tag="2")]
    pub spec: ::core::option::Option<super::v1::JobSpec>,
}
