# HTTPS Docker Traefik

Traefik reverse proxy with automatic HTTPS via Let's Encrypt using Cloudflare DNS challenge.

## Prerequisites

- Docker and Docker Compose
- A domain managed by Cloudflare (e.g. `robiulhossain.com`)
- A Cloudflare DNS API token with permissions: `Zone > DNS > Edit`

## Setup

### 1. Create the external network

```bash
docker network create demo
```

### 2. Configure environment

```bash
cp .env.example .env
```

Edit `.env` and set your Cloudflare DNS API token:

```
CF_DNS_API_TOKEN=your_cloudflare_dns_api_token_here
```

### 3. Update domain

Edit `whoami.yml` and replace `test.robiulhossain.com` with your actual domain.

### 4. Start Traefik

```bash
docker compose up -d
```

### 5. Deploy the sample app

```bash
docker compose -f whoami.yml up -d
```

### 6. Verify

Visit `https://test.robiulhossain.com` — the whoami service should respond over HTTPS. The Traefik dashboard is available at `http://localhost:8080`.

## Project Structure

```
https-docker-tarefik/
├── docker-compose.yml      # Traefik reverse proxy
├── whoami.yml              # Sample app with Traefik labels
├── .env.example            # Environment template
├── config/
│   └── traefik.yaml        # Static Traefik config (entrypoints, ACME, providers)
└── data/
    └── certs/              # ACME certificate storage (auto-created)
```

## Key Configuration

- **Entrypoints**: `web` (port 80, redirects to HTTPS) and `websecure` (port 443)
- **Certificate Resolver**: `cloudflare` — uses Let's Encrypt production with DNS challenge via Cloudflare
- **TLS Certificates**: Stored in `data/certs/cloudflare-acme.json`
- **Docker Provider**: Discovers services via labels (`exposedByDefault: false`)
- **Dashboard**: Insecure mode enabled at `http://localhost:8080`

## References

- [HTTPS for Docker with Traefik](https://www.youtube.com/watch?v=-hfejNXqOzA&t=2154s)
