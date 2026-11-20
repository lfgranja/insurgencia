-- Migração de teste (alvo local descartável) — padrão MDMV: Postgres, JSONB, GIN, sqlx (sem ORM).
CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE IF NOT EXISTS module_manifest (
    id           text PRIMARY KEY,
    version      text NOT NULL,
    payload      jsonb NOT NULL DEFAULT '{}'::jsonb,
    content_hash char(64) NOT NULL,
    created_at   timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_module_manifest_payload_gin
    ON module_manifest USING gin (payload);

CREATE TABLE IF NOT EXISTS evidence (
    id          bigserial PRIMARY KEY,
    criterion   text NOT NULL,
    verdict     text NOT NULL,
    source      text NOT NULL,
    recorded_at timestamptz NOT NULL DEFAULT now()
);

COMMENT ON TABLE module_manifest IS 'Contrato versionado de Módulo; acesso via sqlx, nunca ORM.';
