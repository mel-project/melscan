import { c as create_ssr_component, v as validate_component, e as escape, a as each, b as add_attribute } from './index-e26e4552-f700655a.js';
import { m as melscan } from './common-1a394d1a-ca1b3e74.js';
import { T as TopNav } from './TopNav-2806625e-0b2732a9.js';

const css$1 = {
  code: ".hs.svelte-dnxwpb{width:100%;border:1px black solid;font-size:0.8rem;padding:0.3rem}.error.svelte-dnxwpb{font-size:0.8rem;color:red;padding-top:0.2rem}",
  map: null
};
const HashSearch = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  let value = "";
  let error = "";
  $$result.css.add(css$1);
  return `<input placeholder="${"Search for a hash or height"}" class="${"hs svelte-dnxwpb"}"${add_attribute("value", value, 0)} ${""}>
	${error.length > 0 ? `<div class="${"error svelte-dnxwpb"}">${escape(error)}</div>` : ``}`;
});
const css = {
  code: "td.svelte-2lq0zg{max-width:0;text-overflow:ellipsis}",
  map: null
};
let load = async (loadEvent) => {
  let endpoint = "/raw/overview";
  let props = await melscan(loadEvent.fetch, endpoint);
  return { status: 200, props: { params: props } };
};
const Routes = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  let recentTxx;
  let { params } = $$props;
  if ($$props.params === void 0 && $$bindings.params && params !== void 0)
    $$bindings.params(params);
  $$result.css.add(css);
  recentTxx = () => {
    let x = params.recent_blocks.map((b) => b.transactions).flat();
    if (x.length > 50) {
      x.length = 50;
    }
    return x;
  };
  {
    {
      console.debug(params.recent_blocks);
    }
  }
  return `${validate_component(TopNav, "TopNav").$$render($$result, {}, {}, {
    default: () => {
      return `<a href="${"/"}">Melscan</a>`;
    }
  })}
<div class="${"container mx-auto max-w-screen-lg"}"><div class="${"grid mt-8"}"><div class="${"col-span-full"}">${validate_component(HashSearch, "HashSearch").$$render($$result, {}, {}, {})}</div></div>

	<div class="${"grid grid-cols-1 md:grid-cols-2 mt-8 mb-8"}"><div class="${"col-span-2 mb-3"}"><h3 class="${"text-2xl font-bold"}">Melmint/Melswap</h3></div>
		<div><span class="${"text-lg font-bold"}"><span class="${"text-black text-opacity-50"}">1 ERG =</span>
				${escape((1 / params.erg_per_mel).toFixed(5))} MEL
			</span>
			<br>
			<small class="${"text-blue-600 font-bold"}"><a href="${"/pools/ERG/MEL"}">See details \u2192</a></small></div>
		<div><span class="${"text-lg font-bold"}"><span class="${"text-black text-opacity-50"}">1 SYM =</span>
				${escape((1 / params.sym_per_mel).toFixed(5))} MEL
			</span>
			<br>
			<small class="${"text-blue-600 font-bold"}"><a href="${"/pools/MEL/SYM"}">See details \u2192</a></small></div></div>

	<div class="${"grid grid-cols-1 md:grid-cols-2 mt-12 mb-12 grid-flow-row"}"><div class="${"col-span-full mb-3"}"><h3 class="${"text-2xl font-bold"}">Latest activity</h3></div>
		<div id="${"latest-blocks"}"><h3 class="${"text-xl font-semibold"}">Latest blocks</h3>
			<div class="${"info-section"}"><small class="${"text-black text-opacity-50 font-bold"}">Most recently confirmed blocks</small>
				<div class="${"lds-ellipsis"}"><div></div>
					<div></div>
					<div></div>
					<div></div></div></div>
			<table class="${"table-auto w-full mt-3"}"><thead class="${"text-left text-sm text-black text-opacity-50"}"><tr><th class="${"block-height"}">Height</th>
						<th class="${"block-weight"}">Total weight</th>
						<th class="${"block-reward"}">Proposer reward</th></tr></thead>
				<tbody id="${"block-rows"}" class="${"leading-loose text-sm"}">${each(params.recent_blocks, (block) => {
    return `<tr><td class="${"font-medium svelte-2lq0zg"}"><a href="${"/blocks/" + escape(block.header.height, true)}" class="${"text-blue-600"}">${escape(block.header.height)}</a></td>
							<td class="${"svelte-2lq0zg"}">${escape(block.total_weight)} wu</td>
							<td class="${"svelte-2lq0zg"}">${escape((block.reward_amount / 1e6).toFixed(6))} MEL</td>
						</tr>`;
  })}</tbody></table></div>
		<div><h3 class="${"text-xl font-semibold"}">Latest transactions</h3>
			<div class="${"info-section"}"><small class="${"text-black text-opacity-50 font-bold"}">Transactions in latest blocks</small>
				<div class="${"lds-ellipsis"}"><div></div>
					<div></div>
					<div></div>
					<div></div></div></div>
			<table class="${"table-auto w-full mt-3"}"><thead class="${"text-left text-sm text-black text-opacity-50"}"><tr><th>Hash</th>
						<th>Height</th>
						<th>Mel moved</th></tr></thead>
				<tbody id="${"transaction-rows"}" class="${"leading-loose text-sm"}">${each(recentTxx(), (tx) => {
    return `<tr><td class="${"font-medium svelte-2lq0zg"}" style="${"overflow: hidden; text-overflow: ellipsis"}"><a href="${"/blocks/" + escape(tx.height, true) + "/" + escape(tx.hash, true)}" class="${"text-blue-600"}">${escape(tx.hash)}</a></td>
							<td class="${"svelte-2lq0zg"}">${escape(tx.height)}</td>
							<td class="${"svelte-2lq0zg"}">${escape((tx.mel_moved / 1e6).toFixed(6))} MEL</td>
						</tr>`;
  })}</tbody></table></div></div>
</div>`;
});

var index_svelte = /*#__PURE__*/Object.freeze({
	__proto__: null,
	'default': Routes,
	load: load
});

const index = 7;
const file = '_app/immutable/pages/index.svelte-c792b860.js';
const imports = ["_app/immutable/pages/index.svelte-c792b860.js","_app/immutable/chunks/index-d5cc66c1.js","_app/immutable/chunks/common-9379a3d9.js","_app/immutable/chunks/singletons-eca981c1.js","_app/immutable/chunks/TopNav-61067f92.js"];
const stylesheets = ["_app/immutable/assets/index-f5e555e2.css","_app/immutable/assets/TopNav-6aef7732.css"];

export { file, imports, index, index_svelte as module, stylesheets };
//# sourceMappingURL=7-10d583b3.js.map
