CREATE EXTENSION IF NOT EXISTS dblink;

DO
$do$
BEGIN
   IF NOT EXISTS (SELECT FROM pg_database WHERE datname = 'bartender') THEN
      PERFORM dblink_exec('dbname=postgres', 'CREATE DATABASE bartender');
END IF;
END
$do$;

DO
$do$
BEGIN
   IF NOT EXISTS (SELECT FROM pg_database WHERE datname = 'todo') THEN
      PERFORM dblink_exec('dbname=postgres', 'CREATE DATABASE todo');
END IF;
END
$do$;
