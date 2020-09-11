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

use crate::crd::{Client, User};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector;
use kube_derive::CustomResource;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(CustomResource, Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[kube(
    group = "keycloak.org",
    version = "v1alpha1",
    kind = "KeycloakRealm",
    namespaced,
    derive = "Default",
    derive = "PartialEq",
    status = "KeycloakRealmStatus"
)]
#[kube(apiextensions = "v1beta1")]
#[serde(default, rename_all = "camelCase")]
pub struct KeycloakRealmSpec {
    pub instance_selector: LabelSelector,
    pub realm: Realm,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct Realm {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub clients: Vec<Client>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub display_name: String,
    pub enabled: bool,
    pub events_enabled: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub events_listeners: Vec<String>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub id: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub identity_providers: Vec<IdentityProvider>,
    pub realm: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub users: Vec<User>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub realm_overrides: Vec<Override>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct IdentityProvider {
    add_read_token_role_on_create: bool,
    #[serde(skip_serializing_if = "String::is_empty")]
    alias: String,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    config: BTreeMap<String, String>,
    #[serde(skip_serializing_if = "String::is_empty")]
    display_name: String,
    enabled: bool,
    #[serde(skip_serializing_if = "String::is_empty")]
    first_broker_login_flow_alias: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    internal_id: String,
    link_only: bool,
    #[serde(skip_serializing_if = "String::is_empty")]
    post_broker_login_flow_alias: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    provider_id: String,
    store_token: bool,
    trust_email: bool,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct Override {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub for_flow: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub identity_provider: String,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct KeycloakRealmStatus {
    pub phase: String,
    pub message: String,
    pub ready: bool,
}
