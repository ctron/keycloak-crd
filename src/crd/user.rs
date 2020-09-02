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
    kind = "KeycloakUser",
    namespaced,
    derive = "PartialEq",
    status = "KeycloakUserStatus"
)]
#[kube(apiextensions = "v1beta1")]
#[serde(default, rename_all = "camelCase")]
pub struct KeycloakUserSpec {
    pub realm_selector: LabelSelector,
    pub user: User,
}

impl Default for KeycloakUser {
    fn default() -> Self {
        KeycloakUser {
            kind: KeycloakUser::KIND.into(),
            api_version: KeycloakUser::API_VERSION.into(),
            metadata: Default::default(),
            spec: Default::default(),
            status: Default::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct User {
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub attributes: BTreeMap<String, String>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub client_roles: BTreeMap<String, Vec<String>>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub credentials: Vec<Credential>,
    pub email: String,
    pub email_verified: bool,
    pub enabled: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub federated_identities: Vec<FederatedIdentity>,
    pub first_name: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<String>,
    pub id: String,
    pub last_name: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub realm_roles: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub required_actions: Vec<String>,
    pub username: String,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct Credential {
    pub temporary: bool,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub r#type: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub value: String,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct FederatedIdentity {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub identity_provider: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub user_id: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub user_name: String,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct KeycloakUserStatus {
    pub phase: String,
    pub message: String,
}
