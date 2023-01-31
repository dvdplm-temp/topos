/// Certificate - main exchange item
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Certificate {
    #[prost(message, optional, tag = "1")]
    pub prev_id: ::core::option::Option<super::super::shared::v1::CertificateId>,
    #[prost(message, optional, tag = "2")]
    pub source_subnet_id: ::core::option::Option<super::super::shared::v1::SubnetId>,
    #[prost(bytes = "vec", tag = "3")]
    pub state_root: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "4")]
    pub tx_root_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag = "5")]
    pub target_subnets: ::prost::alloc::vec::Vec<super::super::shared::v1::SubnetId>,
    #[prost(uint32, tag = "6")]
    pub verifier: u32,
    #[prost(message, optional, tag = "7")]
    pub id: ::core::option::Option<super::super::shared::v1::CertificateId>,
    #[prost(message, optional, tag = "8")]
    pub proof: ::core::option::Option<super::super::shared::v1::StarkProof>,
    #[prost(message, optional, tag = "9")]
    pub signature: ::core::option::Option<super::super::shared::v1::Frost>,
}
