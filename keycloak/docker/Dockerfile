    FROM quay.io/keycloak/keycloak:22.0.1

    # Customize context path
    ENV KC_HTTP_RELATIVE_PATH=/auth

    # If the server should expose healthcheck endpoints.
    ENV KC_HEALTH_ENABLED=true

    # If the server should expose metrics endpoints.
    ENV KC_METRICS_ENABLED=true

    ENV KC_PROXY=passthrough
    ENV KC_CACHE=local

    ENV KC_HTTP_ENABLED=true
    ENV KC_HTTP_PORT=8080

    ENV KC_HTTPS_PROTOCOLS=TLSv1.3,TLSv1.2

    ENV KC_HOSTNAME=id.rust.test:8443

    RUN /opt/keycloak/bin/kc.sh build