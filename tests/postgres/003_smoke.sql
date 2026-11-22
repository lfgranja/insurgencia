-- Verificação de comportamento: JSONB + GIN + privilégio mínimo.
INSERT INTO module_manifest (id, version, payload, content_hash)
VALUES ('insurgencia.exemplo', '1.0.0', '{"pilares":["soberania","materialismo"]}'::jsonb,
        repeat('a', 64));

SELECT id, version, payload ? 'pilares' AS tem_pilares
FROM module_manifest WHERE payload @> '{"pilares":["soberania"]}'::jsonb;

SELECT table_name, privilege_type
FROM information_schema.role_table_grants
WHERE grantee = 'insurgencia_app' AND table_schema = 'public'
ORDER BY table_name, privilege_type;
