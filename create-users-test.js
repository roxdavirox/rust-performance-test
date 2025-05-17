import http from 'k6/http';
import { check } from 'k6';
import { uuidv4 } from 'https://jslib.k6.io/k6-utils/1.4.0/index.js';

export const options = {
  scenarios: {
    criacao_consulta: {
      executor: 'ramping-arrival-rate',
      startRate: 50,             // começa com 50 req/s
      timeUnit: '1s',
      preAllocatedVUs: 100,
      maxVUs: 1000,
      stages: [
        { target: 200, duration: '1m' }, // rampa para 200 req/s em 1 minuto
        { target: 500, duration: '2m' }, // rampa para 500 req/s em 2 minutos
        // cooldown de 30s removido — não existe na Rinha 2023
      ],
    },
  },
  thresholds: {
    http_req_duration: ['p(95)<200'],
    http_req_failed: ['rate<0.01'],
  },
};

export default function () {
  const uuid = uuidv4();
  const apelido = `user-${uuid}`;
  const nome = `${uuid}-Usuário de teste`;
  const nascimento = '1990-01-01';
  const stack = Math.random() > 0.5 ? ['Rust', 'Axum', 'SQL'] : undefined;

  const payload = JSON.stringify({
    apelido,
    nome,
    nascimento,
    ...(stack && { stack }),
  });

  const headers = { 'Content-Type': 'application/json' };

  const res = http.post('http://localhost:9999/pessoas', payload, { headers });

  check(res, {
    // compatível com a Rinha: 201, 400 e 422 são válidos
    'status válido (201|400|422)': (r) => [201, 400, 422].includes(r.status),
    'body has id': (r) => r.body && r.body.includes('id'),
  });
}

