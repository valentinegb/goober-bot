-- Your SQL goes here

CREATE TABLE "configs"(
	"guild" BIGINT NOT NULL PRIMARY KEY,
	"strikes_enabled" BOOL NOT NULL,
	"strikes_log_channel" BIGINT,
	"anon_enabled" BOOL NOT NULL,
	"anon_channel" BIGINT,
	"anon_log_channel" BIGINT
);

CREATE TABLE "strikes"(
	"id" OID NOT NULL PRIMARY KEY,
	"guild" BIGINT NOT NULL,
	"user" BIGINT NOT NULL,
	"issuer" BIGINT NOT NULL,
	"issued" TIMESTAMP NOT NULL,
	"rule" TEXT,
	"comment" TEXT,
	"expiration" TIMESTAMP,
	"repealer" BIGINT
);

CREATE TABLE "analytics"(
	"command" TEXT NOT NULL PRIMARY KEY,
	"invocations" TIMESTAMP[] NOT NULL
);
