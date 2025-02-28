-- This file should undo anything in `up.sql`



ALTER TABLE "strikes" DROP COLUMN "id";
ALTER TABLE "strikes" ADD COLUMN "id" OID NOT NULL;

