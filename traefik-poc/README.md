# Traefik POC

Proof-of-concept for Traefik reverse proxy with Docker, covering quick-start HTTP and full HTTPS via Let's Encrypt.

## Projects

### 1. [quick-start-docker](quick-start-docker/)

Minimal Traefik setup — HTTP only, no TLS. Good for learning the basics.

- Exposes port 80 (HTTP) and 8080 (dashboard)
- Auto-discovers Docker containers via labels

### 2. [https-docker-tarefik](https-docker-tarefik/)

Production-like setup with HTTPS using Let's Encrypt + Cloudflare DNS challenge.

- Automatic HTTP-to-HTTPS redirect
- Let's Encrypt ACME with Cloudflare DNS validation
- Includes a sample `whoami` service as a test app

## Prerequisites

- Docker and Docker Compose
- A domain managed by Cloudflare (for the HTTPS setup)
- A Cloudflare DNS API token (for the HTTPS setup)

## References

- [HTTPS for Docker with Traefik](https://www.youtube.com/watch?v=-hfejNXqOzA&t=2154s)
- [Deploy via VPS](https://www.youtube.com/watch?v=F-9KWQByeU0&t=1524s)
- [Traefik Crash Course](https://www.youtube.com/watch?v=C6IL8tjwC5E&t=68s)
- [Build Your Own Traefik](https://www.youtube.com/watch?v=B9t1iNNiHUA)
- [HTTPS for Kubernetes](https://www.youtube.com/watch?v=vJweuU6Qrgo)
