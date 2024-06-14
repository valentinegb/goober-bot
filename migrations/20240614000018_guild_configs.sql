CREATE TABLE IF NOT EXISTS guild_configs (
    guild_id bit(64) PRIMARY KEY,
    strikes_enabled bool DEFAULT false
);
