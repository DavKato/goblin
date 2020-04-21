const integromat =
  'https://hook.integromat.com/8pd3ieutuim1d3frdcb7vva5978ikois';

export const reportError = async (body) => {
  console.log(JSON.stringify(body));

  const res = await fetch(integromat, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(body),
  });

  return res;
};
