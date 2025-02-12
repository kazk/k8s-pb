// This file is @generated by prost-build.
/// APIGroupDiscovery holds information about which resources are being served for all version of the API Group.
/// It contains a list of APIVersionDiscovery that holds a list of APIResourceDiscovery types served for a version.
/// Versions are in descending order of preference, with the first version being the preferred entry.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct APIGroupDiscovery {
    /// Standard object's metadata.
    /// The only field completed will be name. For instance, resourceVersion will be empty.
    /// name is the name of the API group whose discovery information is presented here.
    /// name is allowed to be "" to represent the legacy, ungroupified resources.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// versions are the versions supported in this group. They are sorted in descending order of preference,
    /// with the preferred version being the first entry.
    /// +listType=map
    /// +listMapKey=version
    #[prost(message, repeated, tag = "2")]
    pub versions: ::prost::alloc::vec::Vec<APIVersionDiscovery>,
}
/// APIGroupDiscoveryList is a resource containing a list of APIGroupDiscovery.
/// This is one of the types able to be returned from the /api and /apis endpoint and contains an aggregated
/// list of API resources (built-ins, Custom Resource Definitions, resources from aggregated servers)
/// that a cluster supports.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct APIGroupDiscoveryList {
    /// ResourceVersion will not be set, because this does not have a replayable ordering among multiple apiservers.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// items is the list of groups for discovery. The groups are listed in priority order.
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<APIGroupDiscovery>,
}
/// APIResourceDiscovery provides information about an API resource for discovery.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct APIResourceDiscovery {
    /// resource is the plural name of the resource.  This is used in the URL path and is the unique identifier
    /// for this resource across all versions in the API group.
    /// Resources with non-empty groups are located at /apis/<APIGroupDiscovery.objectMeta.name>/<APIVersionDiscovery.version>/<APIResourceDiscovery.Resource>
    /// Resources with empty groups are located at /api/v1/<APIResourceDiscovery.Resource>
    #[prost(string, optional, tag = "1")]
    pub resource: ::core::option::Option<::prost::alloc::string::String>,
    /// responseKind describes the group, version, and kind of the serialization schema for the object type this endpoint typically returns.
    /// APIs may return other objects types at their discretion, such as error conditions, requests for alternate representations, or other operation specific behavior.
    /// This value will be null or empty if an APIService reports subresources but supports no operations on the parent resource
    #[prost(message, optional, tag = "2")]
    pub response_kind:
        ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::GroupVersionKind>,
    /// scope indicates the scope of a resource, either Cluster or Namespaced
    #[prost(string, optional, tag = "3")]
    pub scope: ::core::option::Option<::prost::alloc::string::String>,
    /// singularResource is the singular name of the resource.  This allows clients to handle plural and singular opaquely.
    /// For many clients the singular form of the resource will be more understandable to users reading messages and should be used when integrating the name of the resource into a sentence.
    /// The command line tool kubectl, for example, allows use of the singular resource name in place of plurals.
    /// The singular form of a resource should always be an optional element - when in doubt use the canonical resource name.
    #[prost(string, optional, tag = "4")]
    pub singular_resource: ::core::option::Option<::prost::alloc::string::String>,
    /// verbs is a list of supported API operation types (this includes
    /// but is not limited to get, list, watch, create, update, patch,
    /// delete, deletecollection, and proxy).
    /// +listType=set
    #[prost(string, repeated, tag = "5")]
    pub verbs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// shortNames is a list of suggested short names of the resource.
    /// +listType=set
    #[prost(string, repeated, tag = "6")]
    pub short_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// categories is a list of the grouped resources this resource belongs to (e.g. 'all').
    /// Clients may use this to simplify acting on multiple resource types at once.
    /// +listType=set
    #[prost(string, repeated, tag = "7")]
    pub categories: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// subresources is a list of subresources provided by this resource. Subresources are located at /apis/<APIGroupDiscovery.objectMeta.name>/<APIVersionDiscovery.version>/<APIResourceDiscovery.Resource>/name-of-instance/<APIResourceDiscovery.subresources\[i\].subresource>
    /// +listType=map
    /// +listMapKey=subresource
    #[prost(message, repeated, tag = "8")]
    pub subresources: ::prost::alloc::vec::Vec<APISubresourceDiscovery>,
}
/// APISubresourceDiscovery provides information about an API subresource for discovery.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct APISubresourceDiscovery {
    /// subresource is the name of the subresource.  This is used in the URL path and is the unique identifier
    /// for this resource across all versions.
    #[prost(string, optional, tag = "1")]
    pub subresource: ::core::option::Option<::prost::alloc::string::String>,
    /// responseKind describes the group, version, and kind of the serialization schema for the object type this endpoint typically returns.
    /// Some subresources do not return normal resources, these will have null or empty return types.
    #[prost(message, optional, tag = "2")]
    pub response_kind:
        ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::GroupVersionKind>,
    /// acceptedTypes describes the kinds that this endpoint accepts.
    /// Subresources may accept the standard content types or define
    /// custom negotiation schemes. The list may not be exhaustive for
    /// all operations.
    /// +listType=map
    /// +listMapKey=group
    /// +listMapKey=version
    /// +listMapKey=kind
    #[prost(message, repeated, tag = "3")]
    pub accepted_types:
        ::prost::alloc::vec::Vec<super::super::super::apimachinery::pkg::apis::meta::v1::GroupVersionKind>,
    /// verbs is a list of supported API operation types (this includes
    /// but is not limited to get, list, watch, create, update, patch,
    /// delete, deletecollection, and proxy). Subresources may define
    /// custom verbs outside the standard Kubernetes verb set. Clients
    /// should expect the behavior of standard verbs to align with
    /// Kubernetes interaction conventions.
    /// +listType=set
    #[prost(string, repeated, tag = "4")]
    pub verbs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// APIVersionDiscovery holds a list of APIResourceDiscovery types that are served for a particular version within an API Group.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct APIVersionDiscovery {
    /// version is the name of the version within a group version.
    #[prost(string, optional, tag = "1")]
    pub version: ::core::option::Option<::prost::alloc::string::String>,
    /// resources is a list of APIResourceDiscovery objects for the corresponding group version.
    /// +listType=map
    /// +listMapKey=resource
    #[prost(message, repeated, tag = "2")]
    pub resources: ::prost::alloc::vec::Vec<APIResourceDiscovery>,
    /// freshness marks whether a group version's discovery document is up to date.
    /// "Current" indicates the discovery document was recently
    /// refreshed. "Stale" indicates the discovery document could not
    /// be retrieved and the returned discovery document may be
    /// significantly out of date. Clients that require the latest
    /// version of the discovery information be retrieved before
    /// performing an operation should not use the aggregated document
    #[prost(string, optional, tag = "3")]
    pub freshness: ::core::option::Option<::prost::alloc::string::String>,
}
