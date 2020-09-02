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

use k8s_openapi::Resource;
use keycloak_crd::Keycloak;

#[test]
fn verify_resource() {
    assert_eq!(Keycloak::KIND, "Keycloak");
    assert_eq!(Keycloak::GROUP, "keycloak.org");
    assert_eq!(Keycloak::VERSION, "v1alpha1");
    assert_eq!(Keycloak::API_VERSION, "keycloak.org/v1alpha1");
}

#[test]
fn provides_kind_and_apiversion() {
    let json = serde_json::to_value(Keycloak {
        ..Default::default()
    })
    .unwrap();

    assert_eq!(json["kind"], "Keycloak");
    assert_eq!(json["apiVersion"], "keycloak.org/v1alpha1");
}

#[test]
fn accepts_content() {
    let keycloak: Keycloak = serde_yaml::from_str(include_str!("data/keycloak_1.yaml")).unwrap();

    assert_eq!(keycloak.spec.extensions, Vec::<String>::new());

    assert_eq!(keycloak.spec.instances, 1);

    let status = keycloak.status.as_ref().unwrap();
    assert_eq!(&status.phase, "reconciling");
    assert_eq!(&status.message, "bar");
}
