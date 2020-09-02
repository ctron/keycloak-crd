/*
 * Copyright (c) 2020 Jens Reimann and others.
 *
 * See the NOTICE file(s) distributed with this work for additional
 * information regarding copyright ownership.
 *
 * This program and the accompanying materials are made available under the
 * terms of the Eclipse Public License 2.0 which is available at
 * http://www.eclipse.org/legal/epl-2.0
 *
 * SPDX-License-Identifier: EPL-2.0
 */

use k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector;
use k8s_openapi::Resource;
use kube_derive::CustomResource;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(CustomResource, Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[kube(
    group = "keycloak.org",
    version = "v1alpha1",
    kind = "KeycloakClient",
    namespaced,
    derive = "PartialEq",
    status = "KeycloakClientStatus"
)]
#[kube(apiextensions = "v1beta1")]
#[serde(default, rename_all = "camelCase")]
pub struct KeycloakClientSpec {
    pub client: Client,
    pub realm_selector: LabelSelector,
}

impl Default for KeycloakClient {
    fn default() -> Self {
        KeycloakClient {
            kind: KeycloakClient::KIND.into(),
            api_version: KeycloakClient::API_VERSION.into(),
            metadata: Default::default(),
            spec: Default::default(),
            status: Default::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct Client {
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub access: BTreeMap<String, bool>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub admin_url: String,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub attributes: BTreeMap<String, String>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub base_url: String,
    pub bearer_only: bool,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub client_authenticator_type: String,
    pub client_id: String,
    pub consent_required: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub default_roles: Vec<String>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub description: String,
    pub direct_access_grants_enabled: bool,
    pub enabled: bool,
    pub frontchannel_logout: bool,
    pub full_scope_allowed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub implicit_flow_enabled: bool,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub name: String,
    pub node_registration_timeout: i32,
    pub not_before: i32,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub protocol: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub protocol_mappers: Vec<ProtocolMapper>,
    pub public_client: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub redirect_uris: Vec<String>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub root_url: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub secret: String,
    pub service_accounts_enabled: bool,
    pub standard_flow_enabled: bool,
    pub surrogate_auth_required: bool,
    pub use_template_config: bool,
    pub use_template_mappers: bool,
    pub use_template_scope: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub web_origins: Vec<String>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct ProtocolMapper {
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub config: BTreeMap<String, String>,
    pub consent_required: bool,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub consent_text: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub id: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub protocol: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub protocol_mapper: String,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct KeycloakClientStatus {
    pub phase: String,
    pub message: String,
    pub ready: bool,
}
