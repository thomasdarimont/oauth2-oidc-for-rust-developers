# OAuth2 and OpenID Connect for Rust Developers

## Setup

### Add local host aliases
Add the following entries to your `/etc/hosts` file

```
127.0.0.1 id.rust.test api.rust.test apps.rust.test
```

### Generate certs
The example uses TLS where possible, therfore we need custom certificates.

We use [mkcert](https://github.com/FiloSottile/mkcert) to automatically generate certs for the example environment.

Run the following command to generate the certs:
```
mkcert -cert-file ./certs/rust.test.pem -key-file ./certs/rust.test-key.pem -install id.rust.test api.rust.test apps.rust.test
```

## Keycloak 

[Keycloak](https://www.keycloak.org) is used as OAuth2 Authorization Server and OpenID Provider.

### Run Keycloak

```
cd keycloak; docker compose -f keycloak/docker-compose.yml up
```

### Access Admin UI

Login via https://id.rust.test:8443/auth with `admin/admin`.
Switch to the `playground` realm that was automatically generated during startup.

The playground realm contains the following custom clients:
- `web-frontend-spa` accessible via https://apps.rust.test:4443/web-frontend-spa/

You can login with the following users:
- `tester` with password `test`

# Example Apps

## OpenID Connect Public Client

### JavaScript

A public client that uses the authorization code flow with PKCE for authentication.
The frontend can also access the APIs provided by resource server with an access token. 

Run with `cd apps/web-frontend-spa; cargo run`

## OAuth2 Resource Server Examples

The resource servers provide a simple "me" endpoint that return user data contained in the access token.

### Actix

Run with `cd apps/api-actix; cargo run`

### Rocket

Run with `cd apps/api-rocket; cargo run`

## CLIs with Device Flow

### Plain

Run with `cd apps/cli-device-flow-plain; cargo run`

### Oauth2 Library

Run with `cd apps/cli-device-flow-oauth2; cargo run`