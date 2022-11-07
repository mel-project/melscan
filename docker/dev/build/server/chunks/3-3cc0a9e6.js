import { c as create_ssr_component, v as validate_component } from './index-e26e4552-f700655a.js';
import { C as CoinDag } from './CoinDag-f56c1ef3-3d396e09.js';
import './common-1a394d1a-ca1b3e74.js';
import 'vis-network';
import 'vis-data';

const css = {
  code: ".wrap.svelte-1nt133{height:100vh;width:100vw;position:fixed;top:0;left:0}",
  map: null
};
let load = async (loadEvent) => {
  let { height, txhash } = loadEvent.params;
  return { status: 200, props: { height, txhash } };
};
const Explore = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  let { height } = $$props;
  let { txhash } = $$props;
  if ($$props.height === void 0 && $$bindings.height && height !== void 0)
    $$bindings.height(height);
  if ($$props.txhash === void 0 && $$bindings.txhash && txhash !== void 0)
    $$bindings.txhash(txhash);
  $$result.css.add(css);
  return `<div class="${"wrap svelte-1nt133"}">${validate_component(CoinDag, "CoinDag").$$render($$result, { height, txhash }, {}, {})}
</div>`;
});

var explore_svelte = /*#__PURE__*/Object.freeze({
  __proto__: null,
  'default': Explore,
  load: load
});

const index = 3;
const file = '_app/immutable/pages/blocks/_height_/_txhash_/explore.svelte-a485cfdd.js';
const imports = ["_app/immutable/pages/blocks/_height_/_txhash_/explore.svelte-a485cfdd.js","_app/immutable/chunks/index-d5cc66c1.js","_app/immutable/chunks/CoinDag-7236be9e.js","_app/immutable/chunks/common-9379a3d9.js","_app/immutable/chunks/singletons-eca981c1.js"];
const stylesheets = ["_app/immutable/assets/explore-d5301ae9.css","_app/immutable/assets/CoinDag-3faf1cb9.css"];

export { file, imports, index, explore_svelte as module, stylesheets };
//# sourceMappingURL=3-3cc0a9e6.js.map
