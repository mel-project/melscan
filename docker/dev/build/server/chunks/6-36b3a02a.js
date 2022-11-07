import { c as create_ssr_component, v as validate_component, e as escape, a as each } from './index-e26e4552-f700655a.js';
import { B as BreadCrumb, a as BreadCrumbs } from './page-types-9e95c4e2-05d65f96.js';
import { m as melscan, t as tooltips } from './common-1a394d1a-ca1b3e74.js';
import { T as TopNav } from './TopNav-2806625e-0b2732a9.js';

/* empty css                                                                                    */function guard(name) {
  return () => {
    throw new Error(`Cannot call ${name}(...) on the server`);
  };
}
const invalidate = guard("invalidate");
const css = {
  code: ".container.svelte-18eftd5 td.svelte-18eftd5{overflow:hidden;text-overflow:elipses}.bottom-nav.svelte-18eftd5.svelte-18eftd5{display:flex;flex-direction:row;justify-content:space-around;width:100%;padding:1em;padding-left:0}a.svelte-18eftd5.svelte-18eftd5:hover{text-decoration:underline}",
  map: null
};
let load = async (event) => {
  let { params, fetch, url } = event;
  let res = await melscan(fetch, "/raw" + url.pathname + "/summary");
  if (res == null) {
    invalidate(url);
    return;
  }
  return { status: 200, props: res };
};
const U5Bheightu5D = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  let breadcrumbs;
  let { header } = $$props;
  let { total_weight } = $$props;
  let { reward_amount } = $$props;
  let { transactions } = $$props;
  let { total_fees } = $$props;
  let { header_hash } = $$props;
  let { fee_multiplier } = $$props;
  if ($$props.header === void 0 && $$bindings.header && header !== void 0)
    $$bindings.header(header);
  if ($$props.total_weight === void 0 && $$bindings.total_weight && total_weight !== void 0)
    $$bindings.total_weight(total_weight);
  if ($$props.reward_amount === void 0 && $$bindings.reward_amount && reward_amount !== void 0)
    $$bindings.reward_amount(reward_amount);
  if ($$props.transactions === void 0 && $$bindings.transactions && transactions !== void 0)
    $$bindings.transactions(transactions);
  if ($$props.total_fees === void 0 && $$bindings.total_fees && total_fees !== void 0)
    $$bindings.total_fees(total_fees);
  if ($$props.header_hash === void 0 && $$bindings.header_hash && header_hash !== void 0)
    $$bindings.header_hash(header_hash);
  if ($$props.fee_multiplier === void 0 && $$bindings.fee_multiplier && fee_multiplier !== void 0)
    $$bindings.fee_multiplier(fee_multiplier);
  $$result.css.add(css);
  breadcrumbs = [BreadCrumb("Melscan", "/"), BreadCrumb(`Block ${header.height}`, ``)];
  return `<div class="${"container mx-auto max-w-screen-lg svelte-18eftd5"}">${validate_component(TopNav, "TopNav").$$render($$result, {}, {}, {
    default: () => {
      return `${validate_component(BreadCrumbs, "BreadCrumbs").$$render($$result, { breadcrumbs }, {}, {})}`;
    }
  })}
		<div class="${"bottom-nav svelte-18eftd5"}"><span><a href="${"/blocks/" + escape(header.height - 1, true)}" class="${"svelte-18eftd5"}">\u25C2 Previous</a></span>
			<span><a href="${"/blocks/" + escape(header.height + 1, true)}" rel="${"external"}" class="${"svelte-18eftd5"}">Next \u25B8</a></span></div>
		<div class="${"mb-3 mt-8"}"><h3 class="${"text-2xl font-bold"}">Summary</h3></div>
		<table id="${"block-info"}" class="${"table-fixed w-full m-3 text-sm "}"><tbody><tr><td class="${"w-1/3 text-black text-opacity-50 font-bold svelte-18eftd5"}"><span>Hash</span>
						${escape(tooltips["blockHash"])}</td>
					<td class="${"font-medium overflow-ellipsis overflow-hidden mono svelte-18eftd5"}">${escape(header_hash)}</td></tr>
				<tr><td class="${"w-1/3 text-black text-opacity-50 font-bold svelte-18eftd5"}"><span class="${"name"}">Height</span>
						</td>
					<td class="${"svelte-18eftd5"}">${escape(header.height)}</td></tr>
				<tr><td class="${"w-1/3 text-black text-opacity-50 font-bold svelte-18eftd5"}">Number of transactions</td>
					<td class="${"svelte-18eftd5"}">${escape(transactions.length)}</td></tr>
				<tr><td class="${"w-1/3 text-black text-opacity-50 font-bold svelte-18eftd5"}">Total transaction weight </td>
					<td class="${"svelte-18eftd5"}">${escape(total_weight)} wu</td></tr></tbody></table>

		<table class="${"table-fixed w-full m-3 text-sm"}"><tbody><tr><td class="${"w-1/3 text-black text-opacity-50 font-bold svelte-18eftd5"}"><span class="${"name"}">Fees charged</span>
						${escape(tooltips["feesCharged"])}</td>
					<td class="${"svelte-18eftd5"}">${escape(total_fees / 1e6)} MEL</td>
				</tr><tr><td class="${"w-1/3 text-black text-opacity-50 font-bold svelte-18eftd5"}">Fee multiplier </td>
					<td class="${"svelte-18eftd5"}">${escape(fee_multiplier)} \xB5MEL/wu</td></tr>
				<tr><td class="${"w-1/3 text-black text-opacity-50 font-bold svelte-18eftd5"}">Fee pool ${escape(tooltips["feePool"])}</td>
					<td class="${"svelte-18eftd5"}">${escape(header.fee_pool / 1e6)} MEL</td></tr></tbody></table>
		<table class="${"table-fixed w-full m-3 text-sm"}"><tbody><tr><td class="${"w-1/3 text-black text-opacity-50 font-bold  svelte-18eftd5"}">Coins merkle root </td>
					<td class="${"overflow-ellipsis overflow-hidden mono svelte-18eftd5"}">${escape(header.coins_hash)}</td></tr>
				<tr><td class="${"w-1/3 text-black text-opacity-50 font-bold svelte-18eftd5"}">Pools merkle root </td>
					<td class="${"overflow-ellipsis overflow-hidden mono svelte-18eftd5"}">${escape(header.pools_hash)}</td></tr>
				<tr><td class="${"w-1/3 text-black text-opacity-50 font-bold svelte-18eftd5"}">Transactions merkle root </td>
					<td class="${"overflow-ellipsis overflow-hidden mono svelte-18eftd5"}">${escape(header.transactions_hash)}</td></tr></tbody></table>

		<div class="${"mb-3 mt-8"}"><h3 class="${"text-2xl font-bold"}">Transactions</h3></div>

		<table class="${"table-fixed text-left w-full m-3 text-sm"}"><thead class="${"text-black text-opacity-50"}"><tr><th class="${"w-2/3"}">Hash</th>
					<th>Weight</th></tr></thead>
			<tbody>${each(transactions, (transaction) => {
    return `<tr><td class="${"overflow-ellipsis overflow-hidden mono svelte-18eftd5"}"><a href="${"/blocks/" + escape(header.height, true) + "/" + escape(transaction.hash, true)}" class="${"text-blue-600 svelte-18eftd5"}">${escape(transaction.hash)}</a></td>
						<td class="${"svelte-18eftd5"}">${escape(transaction.weight)} wu</td>
					</tr>`;
  })}</tbody></table>
	</div>`;
});

var index_svelte = /*#__PURE__*/Object.freeze({
  __proto__: null,
  'default': U5Bheightu5D,
  load: load
});

const index = 6;
const file = '_app/immutable/pages/blocks/_height_/index.svelte-b91b669b.js';
const imports = ["_app/immutable/pages/blocks/_height_/index.svelte-b91b669b.js","_app/immutable/chunks/index-d5cc66c1.js","_app/immutable/chunks/common-9379a3d9.js","_app/immutable/chunks/singletons-eca981c1.js","_app/immutable/chunks/page-types-6d3ebd03.js","_app/immutable/chunks/TopNav-61067f92.js"];
const stylesheets = ["_app/immutable/assets/index-718d917b.css","_app/immutable/assets/BreadCrumbs-4f9b45d3.css","_app/immutable/assets/TopNav-6aef7732.css"];

export { file, imports, index, index_svelte as module, stylesheets };
//# sourceMappingURL=6-36b3a02a.js.map
