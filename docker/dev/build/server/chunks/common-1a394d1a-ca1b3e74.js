import './index-e26e4552-f700655a.js';

const baseUrl = "BASE_URL_DYNAMIC";
const isTestnet = baseUrl.includes("testnet");
const backendUrl = (endpoint) => baseUrl + endpoint;
const melscan = async (fetch2, endpoint) => {
  const url = backendUrl(endpoint);
  try {
    const response = await fetch2(url);
    console.log(`requesting ${url}`);
    if (!response.ok) {
      console.error(`failed to fetch '${url}' data`);
    }
    let res = response.json();
    return res;
  } catch {
  }
};
const queryGraph = async (query) => {
  const url = backendUrl("/raw/graph");
  let response = await fetch(url, {
    method: "POST",
    headers: {
      "Content-Type": "application/json"
    },
    credentials: "omit",
    body: JSON.stringify(query)
  });
  let data = await response.json();
  return data.map((elem) => {
    elem.date = new Date(elem.date);
    return elem;
  });
};
let handler = {
  get: function(target) {
    return "";
  }
};
const tooltips = new Proxy({}, handler);

export { isTestnet as i, melscan as m, queryGraph as q, tooltips as t };
//# sourceMappingURL=common-1a394d1a-ca1b3e74.js.map
