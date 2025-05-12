import http from "k6/http";
import { check } from "k6";
import { uuidv4 } from "https://jslib.k6.io/k6-utils/1.4.0/index.js";

export const options = {
  scenarios: {
    criacao_consulta: {
      executor: "ramping-arrival-rate",
      startRate: 6,
      timeUnit: "1s",
      preAllocatedVUs: 100,
      maxVUs: 600,
      stages: [
        { duration: "10s", target: 2 }, // warmup constante
        { duration: "15s", target: 5 }, // constante com random
        { duration: "3m", target: 600 }, // ramp up real
      ],
    },
  },
  thresholds: {
    http_req_duration: ["p(95)<200"],
    http_req_failed: ["rate<0.01"],
  },
};

export default function () {
  const uuid = uuidv4();
  const apelido = `user-${uuid}`;
  const nome = `${uuid}-Usuário de teste`;
  const nascimento = "1990-01-01";
  const stack = Math.random() > 0.5 ? ["Rust", "Axum", "SQL"] : undefined;

  const payload = JSON.stringify({
    apelido,
    nome,
    nascimento,
    ...(stack && { stack }),
  });

  const headers = { "Content-Type": "application/json" };

  const res = http.post("http://localhost:9999/pessoas", payload, { headers });

  check(res, {
    "status é válido": (r) => [201, 400, 422].includes(r.status),
  });
}
