#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListByCustomRpManifest {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CustomRpManifest>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomRpRouteDefinition {
    pub name: String,
    pub endpoint: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomRpActionRouteDefinition {
    #[serde(flatten)]
    pub custom_rp_route_definition: CustomRpRouteDefinition,
    #[serde(rename = "routingType", skip_serializing_if = "Option::is_none")]
    pub routing_type: Option<custom_rp_action_route_definition::RoutingType>,
}
pub mod custom_rp_action_route_definition {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum RoutingType {
        Proxy,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomRpResourceTypeRouteDefinition {
    #[serde(flatten)]
    pub custom_rp_route_definition: CustomRpRouteDefinition,
    #[serde(rename = "routingType", skip_serializing_if = "Option::is_none")]
    pub routing_type: Option<custom_rp_resource_type_route_definition::RoutingType>,
}
pub mod custom_rp_resource_type_route_definition {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum RoutingType {
        Proxy,
        #[serde(rename = "Proxy,Cache")]
        ProxyCache,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomRpValidations {
    #[serde(rename = "validationType", skip_serializing_if = "Option::is_none")]
    pub validation_type: Option<custom_rp_validations::ValidationType>,
    pub specification: String,
}
pub mod custom_rp_validations {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ValidationType {
        Swagger,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomRpManifest {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<custom_rp_manifest::Properties>,
}
pub mod custom_rp_manifest {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[serde(skip_serializing_if = "Vec::is_empty")]
        pub actions: Vec<CustomRpActionRouteDefinition>,
        #[serde(rename = "resourceTypes", skip_serializing_if = "Vec::is_empty")]
        pub resource_types: Vec<CustomRpResourceTypeRouteDefinition>,
        #[serde(skip_serializing_if = "Vec::is_empty")]
        pub validations: Vec<CustomRpValidations>,
        #[serde(rename = "provisioningState", skip_serializing)]
        pub provisioning_state: Option<properties::ProvisioningState>,
    }
    pub mod properties {
        use super::*;
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum ProvisioningState {
            Accepted,
            Deleting,
            Running,
            Succeeded,
            Failed,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceProviderOperationList {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ResourceProviderOperation>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceProviderOperation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<resource_provider_operation::Display>,
}
pub mod resource_provider_operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Display {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Association {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<association::Properties>,
}
pub mod association {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[serde(rename = "targetResourceId", skip_serializing_if = "Option::is_none")]
        pub target_resource_id: Option<String>,
        #[serde(rename = "provisioningState", skip_serializing)]
        pub provisioning_state: Option<properties::ProvisioningState>,
    }
    pub mod properties {
        use super::*;
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum ProvisioningState {
            Accepted,
            Deleting,
            Running,
            Succeeded,
            Failed,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssociationsList {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Association>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    pub location: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceProvidersUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDefinition>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorDefinition {
    #[serde(skip_serializing)]
    pub code: Option<String>,
    #[serde(skip_serializing)]
    pub message: Option<String>,
    #[serde(skip_serializing)]
    pub details: Vec<ErrorDefinition>,
}