BEGIN;

CREATE UNIQUE INDEX unique_timeslot_per_email
ON timeslots(email)
WHERE email IS NOT NULL;

COMMIT;
