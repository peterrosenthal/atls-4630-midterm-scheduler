BEGIN;

CREATE TABLE IF NOT EXISTS timeslots (
  id serial PRIMARY KEY,
  email TEXT,
  start_time TIMESTAMPTZ NOT NULL,
  end_time TIMESTAMPTZ NOT NULL,
  CONSTRAINT timeslots_unique_start_end UNIQUE (start_time, end_time)
);

-- Seed rows: times specified in local America/Denver and converted to timestamptz using AT TIME ZONE.
-- Use a single multi-row INSERT with ON CONFLICT DO NOTHING to avoid duplicates and handle idempotency.

INSERT INTO timeslots (email, start_time, end_time)
VALUES
  (NULL, '2025-10-28 17:00:00' AT TIME ZONE 'America/Denver', '2025-10-28 17:20:00' AT TIME ZONE 'America/Denver'),
  (NULL, '2025-10-28 17:20:00' AT TIME ZONE 'America/Denver', '2025-10-28 17:40:00' AT TIME ZONE 'America/Denver'),
  (NULL, '2025-10-28 17:40:00' AT TIME ZONE 'America/Denver', '2025-10-28 18:00:00' AT TIME ZONE 'America/Denver'),
  (NULL, '2025-10-28 18:00:00' AT TIME ZONE 'America/Denver', '2025-10-28 18:20:00' AT TIME ZONE 'America/Denver'),
  (NULL, '2025-10-28 18:20:00' AT TIME ZONE 'America/Denver', '2025-10-28 18:40:00' AT TIME ZONE 'America/Denver'),
  (NULL, '2025-10-28 18:40:00' AT TIME ZONE 'America/Denver', '2025-10-28 19:00:00' AT TIME ZONE 'America/Denver'),
  (NULL, '2025-10-30 17:00:00' AT TIME ZONE 'America/Denver', '2025-10-30 17:20:00' AT TIME ZONE 'America/Denver'),
  (NULL, '2025-10-30 17:20:00' AT TIME ZONE 'America/Denver', '2025-10-30 17:40:00' AT TIME ZONE 'America/Denver'),
  (NULL, '2025-10-30 17:40:00' AT TIME ZONE 'America/Denver', '2025-10-30 18:00:00' AT TIME ZONE 'America/Denver'),
  (NULL, '2025-10-30 18:00:00' AT TIME ZONE 'America/Denver', '2025-10-30 18:20:00' AT TIME ZONE 'America/Denver'),
  (NULL, '2025-10-30 18:20:00' AT TIME ZONE 'America/Denver', '2025-10-30 18:40:00' AT TIME ZONE 'America/Denver'),
  (NULL, '2025-10-30 18:40:00' AT TIME ZONE 'America/Denver', '2025-10-30 19:00:00' AT TIME ZONE 'America/Denver')
ON CONFLICT (start_time, end_time) DO NOTHING;

COMMIT;