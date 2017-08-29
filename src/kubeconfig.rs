extern crate serde_yaml;

use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Kubeconfig {
    // clusters is a map of referencable names to cluster configs
    pub clusters: HashMap<String, Cluster>,
    // users is a map of referencable names to user configs
    pub users: HashMap<String, User>,
    // contexts is a map of referencable names to context configs
    pub contexts: HashMap<String, Context>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Cluster {
    // location_of_origin indicates where this object came from.  It is used for round tripping
    // config post-merge, but never serialized.
    pub location_of_origin: String,
    // server is the address of the kubernetes cluster (https://hostname:port).
    pub server: String,
    // insecure_skip_tls_verify skips the validity check for the server's certificate. This will
    // make your HTTPS connections insecure.
    pub insecure_skip_tls_verify: bool,
    // certificate_authority is the path to a cert file for the certificate authority.
    pub certificate_authority: String,
    // certificate_authority_data contains PEM-encoded certificate authority certificates.
    // Overrides CertificateAuthority
    pub certificate_authority_data: Vec<u8>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    // location_of_origin indicates where this object came from.  It is used for round tripping
    // config post-merge, but never serialized.
    pub location_of_origin: String,
    // client_certificate is the path to a client cert file for TLS.
    pub client_certificate: String,
    // client_certificate_data contains PEM-encoded data from a client cert file for TLS. Overrides
    // ClientCertificate
    pub client_certificate_data: Vec<u8>,
    // client_key is the path to a client key file for TLS.
    pub client_key: String,
    // client_key_data contains PEM-encoded data from a client key file for TLS. Overrides
    // ClientKey
    pub client_key_data: Vec<u8>,
    // token is the bearer token for authentication to the kubernetes cluster.
    pub token: String,
    // token_file is a pointer to a file that contains a bearer token (as described above).  If
    // both Token and TokenFile are present, Token takes precedence.
    pub token_file: String,
    // impersonate is the username to act-as.
    pub impersonate: String,
    // impersonate_groups is the groups to imperonate.
    pub impersonate_groups: Vec<String>,
    // impersonate_user_extra contains additional information for impersonated user.
    pub impersonate_user_extra: HashMap<String, Vec<String>>,
    // username is the username for basic authentication to the kubernetes cluster.
    pub username: String,
    // password is the password for basic authentication to the kubernetes cluster.
    pub password: String,
    // auth_provider specifies a custom authentication plugin for the kubernetes cluster.
    pub auth_provider: AuthProviderConfig,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthProviderConfig {
    pub name: String,
    pub config: HashMap<String, String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Context {
    // location_of_origin indicates where this object came from.  It is used for round tripping
    // config post-merge, but never serialized.
    pub location_of_origin: String,
    // cluster is the name of the cluster for this context
    pub cluster: String,
    // auth_info is the name of the authInfo for this context
    pub auth_info: String,
    // namespace is the default namespace to use on unspecified requests
    pub namespace: String,
}
