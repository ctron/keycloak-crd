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

use k8s_openapi::api::core::v1::ResourceRequirements;
use k8s_openapi::Resource;
use kube_derive::CustomResource;
use serde::{Deserialize, Serialize};

#[derive(CustomResource, Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[kube(
    group = "keycloak.org",
    version = "v1alpha1",
    kind = "Keycloak",
    namespaced,
    derive = "PartialEq",
    status = "KeycloakStatus"
)]
#[kube(apiextensions = "v1beta1")]
#[serde(default, rename_all = "camelCase")]
pub struct KeycloakSpec {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub extensions: Vec<String>,

    pub external_access: ExternalAccess,
    pub external_database: ExternalDatabase,
    pub instances: u32,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub profile: String,
    pub keycloak_deployment_spec: KeycloakDeploymentSpec,
    pub migration: Migration,
    pub pod_disruption_budget: PodDisruptionBudget,
    pub postgres_deployment_spec: PostgresDeploymentSpec,
    pub storage_class_name: String,
}

impl Default for Keycloak {
    fn default() -> Self {
        Keycloak {
            kind: Keycloak::KIND.into(),
            api_version: Keycloak::API_VERSION.into(),
            metadata: Default::default(),
            spec: Default::default(),
            status: Default::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct ExternalAccess {
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_termination: Option<TlsTermination>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TlsTermination {
    Reencrypt,
    Passthrough,
    #[serde(other)]
    Unknown,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct ExternalDatabase {
    pub enabled: bool,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct KeycloakDeploymentSpec {
    pub resources: ResourceRequirements,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct Migration {
    pub backups: Backups,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct Backups {
    pub enabled: bool,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct PodDisruptionBudget {
    pub enabled: bool,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct PostgresDeploymentSpec {
    resources: ResourceRequirements,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct KeycloakStatus {
    pub phase: String,
    pub message: String,
    pub ready: bool,

    pub credential_secret: String,
    pub version: String,
    #[serde(rename = "internalURL")]
    pub internal_url: String,
}
