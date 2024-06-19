-- This file should undo anything in `up.sql`
ALTER TABLE "tokens" DROP COLUMN "account_name";
ALTER TABLE "tokens" DROP COLUMN "issuer";
ALTER TABLE "tokens" DROP COLUMN "secret";
ALTER TABLE "tokens" ADD COLUMN "account_name" VARCHAR(255) NOT NULL;
ALTER TABLE "tokens" ADD COLUMN "issuer" VARCHAR(50) NOT NULL;
ALTER TABLE "tokens" ADD COLUMN "secret" VARCHAR(64) NOT NULL;

