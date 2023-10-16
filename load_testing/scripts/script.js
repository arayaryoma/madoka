import http from "k6/http";
import { check } from "k6";

export let options = {
  stages: [
    // Ramp-up from 1 to 30 VUs in 30s
    { duration: "10s", target: 10000 },

    // Stay on 30 VUs for 60s
    { duration: "60s", target: 10000 },

    // Ramp-down from 30 to 0 VUs in 10s
    { duration: "10s", target: 0 },
  ],
};

export default function () {
  const params = {
    headers: {
      host: "madoka.local",
    },
  };
  let res = http.get("http://localhost:3001/");
  check(res, { "status is 200": (r) => r.status === 200 });
}
