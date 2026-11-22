-- Privilégio mínimo: aplicação lê/escreve dados, não cria esquema nem altera objetos.
CREATE ROLE insurgencia_app LOGIN PASSWORD 'throwaway-local-only';

REVOKE CREATE ON SCHEMA public FROM PUBLIC;
GRANT USAGE ON SCHEMA public TO insurgencia_app;
GRANT SELECT, INSERT ON module_manifest TO insurgencia_app;
GRANT SELECT, INSERT ON evidence TO insurgencia_app;
GRANT USAGE, SELECT ON SEQUENCE evidence_id_seq TO insurgencia_app;

REVOKE UPDATE, DELETE, TRUNCATE ON module_manifest FROM insurgencia_app;
REVOKE UPDATE, DELETE, TRUNCATE ON evidence FROM insurgencia_app;
