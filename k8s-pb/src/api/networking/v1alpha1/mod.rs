// This file is @generated by prost-build.
/// IPAddress represents a single IP of a single IP Family. The object is designed to be used by APIs
/// that operate on IP addresses. The object is used by the Service core API for allocation of IP addresses.
/// An IP address can be represented in different formats, to guarantee the uniqueness of the IP,
/// the name of the object is the IP address in canonical format, four decimal digits separated
/// by dots suppressing leading zeros for IPv4 and the representation defined by RFC 5952 for IPv6.
/// Valid: 192.168.1.5 or 2001:db8::1 or 2001:db8:aaaa:bbbb:cccc:dddd:eeee:1
/// Invalid: 10.01.2.3 or 2001:db8:0:0:0::1
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IPAddress {
    /// Standard object's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// spec is the desired state of the IPAddress.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag = "2")]
    pub spec: ::core::option::Option<IPAddressSpec>,
}
/// IPAddressList contains a list of IPAddress.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IPAddressList {
    /// Standard object's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// items is the list of IPAddresses.
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<IPAddress>,
}
/// IPAddressSpec describe the attributes in an IP Address.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IPAddressSpec {
    /// ParentRef references the resource that an IPAddress is attached to.
    /// An IPAddress must reference a parent object.
    /// +required
    #[prost(message, optional, tag = "1")]
    pub parent_ref: ::core::option::Option<ParentReference>,
}
/// ParentReference describes a reference to a parent object.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParentReference {
    /// Group is the group of the object being referenced.
    /// +optional
    #[prost(string, optional, tag = "1")]
    pub group: ::core::option::Option<::prost::alloc::string::String>,
    /// Resource is the resource of the object being referenced.
    /// +required
    #[prost(string, optional, tag = "2")]
    pub resource: ::core::option::Option<::prost::alloc::string::String>,
    /// Namespace is the namespace of the object being referenced.
    /// +optional
    #[prost(string, optional, tag = "3")]
    pub namespace: ::core::option::Option<::prost::alloc::string::String>,
    /// Name is the name of the object being referenced.
    /// +required
    #[prost(string, optional, tag = "4")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
}
/// ServiceCIDR defines a range of IP addresses using CIDR format (e.g. 192.168.0.0/24 or 2001:db2::/64).
/// This range is used to allocate ClusterIPs to Service objects.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceCidr {
    /// Standard object's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// spec is the desired state of the ServiceCIDR.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag = "2")]
    pub spec: ::core::option::Option<ServiceCidrSpec>,
    /// status represents the current state of the ServiceCIDR.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag = "3")]
    pub status: ::core::option::Option<ServiceCidrStatus>,
}
/// ServiceCIDRList contains a list of ServiceCIDR objects.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceCidrList {
    /// Standard object's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// items is the list of ServiceCIDRs.
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<ServiceCidr>,
}
/// ServiceCIDRSpec define the CIDRs the user wants to use for allocating ClusterIPs for Services.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceCidrSpec {
    /// CIDRs defines the IP blocks in CIDR notation (e.g. "192.168.0.0/24" or "2001:db8::/64")
    /// from which to assign service cluster IPs. Max of two CIDRs is allowed, one of each IP family.
    /// The network address of each CIDR, the address that identifies the subnet of a host, is reserved
    /// and will not be allocated. The broadcast address for IPv4 CIDRs is also reserved and will not be
    /// allocated.
    /// This field is immutable.
    /// +optional
    /// +listType=atomic
    #[prost(string, repeated, tag = "1")]
    pub cidrs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// ServiceCIDRStatus describes the current state of the ServiceCIDR.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceCidrStatus {
    /// conditions holds an array of metav1.Condition that describe the state of the ServiceCIDR.
    /// Current service state
    /// +optional
    /// +patchMergeKey=type
    /// +patchStrategy=merge
    /// +listType=map
    /// +listMapKey=type
    #[prost(message, repeated, tag = "1")]
    pub conditions:
        ::prost::alloc::vec::Vec<super::super::super::apimachinery::pkg::apis::meta::v1::Condition>,
}
