import { c as create_ssr_component } from './index-e26e4552-f700655a.js';
import { m as melscan } from './common-1a394d1a-ca1b3e74.js';

/* empty css                                                                                       */let load = async (event) => {
  let { params, fetch, url } = event;
  let res = await melscan(fetch, "/raw" + url.pathname);
  return { status: 200, props: res };
};
const Spenders = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  return ``;
});

var spenders_svelte = /*#__PURE__*/Object.freeze({
  __proto__: null,
  'default': Spenders,
  load: load
});

const index = 5;
const file = '_app/immutable/pages/blocks/_height_/_txhash_/spenders.svelte-6749ccc9.js';
const imports = ["_app/immutable/pages/blocks/_height_/_txhash_/spenders.svelte-6749ccc9.js","_app/immutable/chunks/index-d5cc66c1.js","_app/immutable/chunks/common-9379a3d9.js","_app/immutable/chunks/singletons-eca981c1.js"];
const stylesheets = ["_app/immutable/assets/BreadCrumbs-4f9b45d3.css"];

export { file, imports, index, spenders_svelte as module, stylesheets };
//# sourceMappingURL=5-06cc97ee.js.map
