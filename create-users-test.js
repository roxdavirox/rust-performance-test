import http from 'k6/http';
import { check } from 'k6';
import { uuidv4 } from 'https://jslib.k6.io/k6-utils/1.4.0/index.js';

export let options = {
  vus: 500,              // Simula 500 usuários simultâneos
  duration: '3m',        // Teste por 3 minutos
  thresholds: {
    http_req_duration: ['p(95)<200'], // latência p95 abaixo de 200ms
    http_req_failed: ['rate<0.01'],   // menos de 1% de erros
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
    'status is 201': (r) => r.status === 201,
    'body has id': (r) => r.body && r.body.includes('id'),
  });
}
