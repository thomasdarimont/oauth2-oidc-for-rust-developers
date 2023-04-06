# OAuth2 and OpenID Connect for Rust Developers

## Setup

### Add local host aliases
Add the following entries to your `/etc/hosts` file

```
127.0.0.1 id.rust.test api.rust.test apps.rust.test
```

### Generate certs
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

# Resource Server Examples

## Actix

Run with `cd apps/api-actix; cargo run`

## Rocket

Run with `cd apps/api-rocket; cargo run`