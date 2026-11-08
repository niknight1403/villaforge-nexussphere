# NexusSphere Architektur-Dokumentation

## 1. Übersicht
NexusSphere ist eine Multi-Device-Plattform zur Steuerung autonomer Agenten-Schwärme.

## 2. Kernkomponenten
- Core: Rust (Zero-Knowledge Gateway)
- Agent-Swarm: Python (FastAPI)
- Frontend: React 19 (tRPC, Tailwind)

## 3. OpenAPI Gateway Spec (Auszug)
```yaml
openapi: 3.0.0
info:
  title: NexusSphere Agent Gateway
  version: 1.0.0
paths:
  /v1/nodes:
    get:
      summary: Liste alle aktiven Nodes
```

## 4. Deployment
Kubernetes-Manifeste befinden sich unter /deploy.