import { Temporal } from "@js-temporal/polyfill";

export default interface Timeslot {
  id: number;
  email?: string;
  startTime: Temporal.Instant;
  endTime: Temporal.Instant;
}
