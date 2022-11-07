import { c as create_ssr_component, v as validate_component, e as escape } from './index-e26e4552-f700655a.js';
import { q as queryGraph } from './common-1a394d1a-ca1b3e74.js';
import { B as BreadCrumb, a as BreadCrumbs } from './page-types-9e95c4e2-05d65f96.js';
import { T as TopNav } from './TopNav-2806625e-0b2732a9.js';
import { G as GraphPlot } from './GraphPlot-2db67e0a-3bd87ffa.js';
import 'uplot';

const css = {
  code: "#head.svelte-mydso4 button.svelte-mydso4{font-size:90%;margin-left:4px;margin-right:4px;border:#ccc;border-width:1px;border-style:solid;padding-left:4px;padding-right:4px;border-radius:10%}#head.svelte-mydso4 button.svelte-mydso4:hover{background-color:#eee}.card.svelte-mydso4.svelte-mydso4{border:#ccc;border-width:1px;border-style:solid;padding-left:4px;padding-right:4px;border-radius:8px;margin:4px;padding:16px}.ticker-card.svelte-mydso4.svelte-mydso4{display:flex;justify-content:center;flex-direction:column}",
  map: null
};
let load = async (event) => {
  let { params } = event;
  return { status: 200, props: { params } };
};
const U5Brightu5D = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  let last_price;
  let last_liquidity;
  let last_height;
  let { params } = $$props;
  let { left, right } = params;
  let pool_key = { left, right };
  let breadcrumbs = [BreadCrumb("Melscan", "/")];
  let handler = {
    get(target) {
      return "";
    }
  };
  let tooltips = new Proxy({}, handler);
  let price_data = [];
  let liquidity_data = [];
  const getPriceData = async (start, end) => await queryGraph({
    id: {
      type: "pool_price",
      from: right,
      to: left
    },
    start,
    end
  });
  if ($$props.params === void 0 && $$bindings.params && params !== void 0)
    $$bindings.params(params);
  $$result.css.add(css);
  last_price = price_data.length > 0 ? price_data[price_data.length - 1].value : 0;
  last_liquidity = liquidity_data.length > 0 ? liquidity_data[liquidity_data.length - 1].value : 0;
  last_height = price_data.length > 0 ? price_data[price_data.length - 1].height : 0;
  return `${validate_component(TopNav, "TopNav").$$render($$result, {}, {}, {
    default: () => {
      return `${validate_component(BreadCrumbs, "BreadCrumbs").$$render($$result, { breadcrumbs }, {}, {})}`;
    }
  })}
	<div class="${"container mx-auto max-w-screen-lg"}"><div class="${"mb-3 mt-8"}" style="${"display: flex"}"><h3 class="${"text-2xl font-bold"}">Pair ${escape(pool_key.left)}/${escape(pool_key.right)}</h3></div>

		<div class="${"grid grid-cols-12 md:grid-flow-col grid-flow-row"}"><div class="${"col-span-12 md:col-span-3 card ticker-card svelte-mydso4"}"><div><small>Price</small>${escape(tooltips["price"])}</div>
				<div class="${"text-lg font-medium"}">${escape(last_price.toFixed(4))}
					${escape(pool_key.left)}/${escape(pool_key.right)}</div></div>
			<div class="${"col-span-12 md:col-span-3 card ticker-card svelte-mydso4"}"><div><small>Liquidity</small>${escape(tooltips["liquidity"])}</div>
				<div class="${"text-lg font-medium"}">${escape(last_liquidity.toFixed(4))}
					<span class="${"text-sm"}">(${escape(pool_key.left)} \xD7 ${escape(pool_key.right)})<sup>1/2</sup></span></div></div>
			<div class="${"col-span-12 md:col-span-3 card ticker-card svelte-mydso4"}"><div><small>Current height</small>${escape(tooltips["height"])}</div>
				<div class="${"text-lg font-medium"}">${escape(last_height)}</div></div>

			<div class="${"md:col-span-9 col-span-12 md:row-span-3 card svelte-mydso4"}"><div class="${"grid grid-cols-2 mb-2 svelte-mydso4"}" id="${"head"}"><div class="${"text-left"}"><button class="${"svelte-mydso4"}">Liquidity</button>
						<button class="${"svelte-mydso4"}">Price</button></div></div>
				${validate_component(GraphPlot, "GraphPlot").$$render(
    $$result,
    {
      fetchData: async (start, end) => await getPriceData(start, end),
      unit: left,
      label: right + " price"
    },
    {},
    {}
  )}</div></div>
	</div>`;
});

var index_svelte = /*#__PURE__*/Object.freeze({
  __proto__: null,
  'default': U5Brightu5D,
  load: load
});

const index = 8;
const file = '_app/immutable/pages/pools/_left_/_right_/index.svelte-c008b748.js';
const imports = ["_app/immutable/pages/pools/_left_/_right_/index.svelte-c008b748.js","_app/immutable/chunks/index-d5cc66c1.js","_app/immutable/chunks/common-9379a3d9.js","_app/immutable/chunks/singletons-eca981c1.js","_app/immutable/chunks/page-types-6d3ebd03.js","_app/immutable/chunks/TopNav-61067f92.js","_app/immutable/chunks/GraphPlot-f03243f5.js","_app/immutable/chunks/index-74e7b1a8.js"];
const stylesheets = ["_app/immutable/assets/index-aa56ea07.css","_app/immutable/assets/BreadCrumbs-4f9b45d3.css","_app/immutable/assets/TopNav-6aef7732.css","_app/immutable/assets/GraphPlot-74ddbde4.css"];

export { file, imports, index, index_svelte as module, stylesheets };
//# sourceMappingURL=8-e6b8e1f1.js.map
