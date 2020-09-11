# Keycloak Custom Resource Definition in Rust

[![Crates.io](https://img.shields.io/crates/v/keycloak-crd.svg)](https://crates.io/crates/keycloak-crd)
![CI](https://github.com/ctron/keycloak-crd/workflows/CI/badge.svg)

This repository contains Rust types, for working with the CRDs from the
[Keycloak operator](https://github.com/keycloak/keycloak-operator).

> [Keycloak](https://www.keycloak.org/) is an open source identity and access management solution.

## Usage

This crate can be used in combination with [kube-rs](https://github.com/clux/kube-rs):

~~~rust
fn main() {
    let keycloaks: Api<Keycloak> = Api::namespaced(client, &namespace);
    keycloaks
            .create(
                &Default::default(),
                &Keycloak {
                    metadata: ObjectMeta {
                        name: Some("my-instance".into()),
                        namespace: Some("my-namespace".into()),
                        ..Default::default()
                    },
                    spec: KeycloakSpec {
                        external_access: ExternalAccess {
                            enabled: true,
                            ..Default::default()
                        },
                        instances: 1,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            )
            .await?; 
}
~~~
