import { c as create_ssr_component, v as validate_component, b as add_attribute, e as escape, a as each } from './index-e26e4552-f700655a.js';
import { m as melscan } from './common-1a394d1a-ca1b3e74.js';
import { B as BreadCrumb, a as BreadCrumbs } from './page-types-9e95c4e2-05d65f96.js';
import { G as GraphPlot } from './GraphPlot-2db67e0a-3bd87ffa.js';
import { T as TopNav } from './TopNav-2806625e-0b2732a9.js';
import 'uplot';

const css = {
  code: ".txn-row.svelte-17fuq11 td.svelte-17fuq11{vertical-align:top;padding-bottom:1rem}",
  map: null
};
let load = async (loadEvent) => {
  let { covhash } = loadEvent.params;
  let url = `/raw/address/${covhash}`;
  let res = await melscan(loadEvent.fetch, url);
  res.transactions = res.transactions.map((t) => {
    t.date = new Date(t.date);
    return t;
  });
  return {
    status: 200,
    props: { summary: res, covhash }
  };
};
const U5Bcovhashu5D = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  let truncatedTransactions;
  let { summary } = $$props;
  let { covhash } = $$props;
  let firstHeight = summary.transactions.length > 0 ? summary.transactions[0].height : 0;
  let firstDate = summary.transactions.length > 0 ? summary.transactions[0].date.toLocaleDateString("en-GB", {
    year: "numeric",
    month: "long",
    day: "numeric"
  }) : "";
  let balanceHistory = (() => {
    let accum = 0;
    return summary.transactions.map((t) => {
      if ("MEL" in t.deltas)
        accum += t.deltas["MEL"];
      return {
        height: t.height,
        date: t.date,
        value: accum
      };
    });
  })();
  let transactionCount = 50;
  if ($$props.summary === void 0 && $$bindings.summary && summary !== void 0)
    $$bindings.summary(summary);
  if ($$props.covhash === void 0 && $$bindings.covhash && covhash !== void 0)
    $$bindings.covhash(covhash);
  $$result.css.add(css);
  truncatedTransactions = summary.transactions.slice().reverse().slice(0, transactionCount);
  return `${validate_component(TopNav, "TopNav").$$render($$result, {}, {}, {
    default: () => {
      return `${validate_component(BreadCrumbs, "BreadCrumbs").$$render(
        $$result,
        {
          breadcrumbs: [BreadCrumb("Melscan", "/"), BreadCrumb(`Address ${covhash}`, `.`)]
        },
        {},
        {}
      )}`;
    }
  })}

	<div class="${"container mx-auto max-w-screen-lg"}"><div class="${"mb-3 mt-8"}"><h3 class="${"text-2xl font-bold"}">Summary</h3></div>

		<div class="${"m-3"}"><table class="${"table-fixed w-full text-sm text-left"}"><tbody><tr><td class="${"text-black text-opacity-50 font-bold w-1/2"}">First seen</td>
						<td><a class="${"text-blue-800 font-medium"}"${add_attribute("href", `/blocks/${firstHeight}`, 0)}>${escape(firstHeight)}</a>
							(${escape(firstDate)})
						</td></tr>
					<tr><td class="${"text-black text-opacity-50 font-bold w-1/2"}">Total balance</td>
						<td>${each(Object.entries(summary.balances), ([denom, balance]) => {
    return `<b class="${"font-medium"}">${escape(balance.toFixed(6))}</b>\xA0<i>${escape(denom)}</i>\xA0\xA0`;
  })}</td></tr></tbody></table></div>

		<div class="${"mb-3 mt-8"}"><h3 class="${"text-2xl font-bold"}">Balance history (MEL)</h3>
			<div class="${"m-3"}">${validate_component(GraphPlot, "GraphPlot").$$render(
    $$result,
    {
      stepped: true,
      unit: "MEL",
      fetchData: async (start, end) => balanceHistory.filter((t) => (!start || t.date >= start) && (!end || t.date <= end))
    },
    {},
    {}
  )}</div></div>

		<div class="${"mb-3 mt-8"}"><h3 class="${"text-2xl font-bold"}">Transaction history</h3>
			<div class="${"m-3"}"><table class="${"table-fixed w-full text-sm text-left"}"><thead class="${"text-black text-opacity-50 font-bold"}"><td class="${"w-24"}">Height</td>
						<td>Hash</td>
						<td class="${"w-32"}">Balance change</td></thead>
					<tbody>${each(truncatedTransactions, (txn) => {
    return `<tr class="${"txn-row svelte-17fuq11"}"><td class="${"svelte-17fuq11"}"><a class="${"text-blue-800 font-medium"}"${add_attribute("href", `/blocks/${txn.height}`, 0)}>${escape(txn.height)}</a></td>
								<td class="${"overflow-ellipsis overflow-hidden mono svelte-17fuq11"}"><a class="${"text-blue-800"}"${add_attribute("href", `/blocks/${txn.height}/${txn.txhash}`, 0)}>${escape(txn.txhash)}</a></td>
								<td class="${"svelte-17fuq11"}">${each(Object.entries(txn.deltas), ([denom, change]) => {
      return `${change > 0 ? `<span class="${"text-green-800"}">+${escape(change.toFixed(6))} ${escape(denom)}</span><br>` : `<span class="${"text-red-800"}">${escape(change.toFixed(6))} ${escape(denom)}</span><br>`}`;
    })}</td>
							</tr>`;
  })}</tbody></table>
				${truncatedTransactions.length != summary.transactions.length ? `<button class="${"text-blue-800"}">Load more</button>` : ``}</div></div>
	</div>`;
});

var index_svelte = /*#__PURE__*/Object.freeze({
  __proto__: null,
  'default': U5Bcovhashu5D,
  load: load
});

const index = 2;
const file = '_app/immutable/pages/address/_covhash_/index.svelte-d565c7a7.js';
const imports = ["_app/immutable/pages/address/_covhash_/index.svelte-d565c7a7.js","_app/immutable/chunks/index-d5cc66c1.js","_app/immutable/chunks/common-9379a3d9.js","_app/immutable/chunks/singletons-eca981c1.js","_app/immutable/chunks/page-types-6d3ebd03.js","_app/immutable/chunks/GraphPlot-f03243f5.js","_app/immutable/chunks/index-74e7b1a8.js","_app/immutable/chunks/TopNav-61067f92.js"];
const stylesheets = ["_app/immutable/assets/index-7ee93b27.css","_app/immutable/assets/BreadCrumbs-4f9b45d3.css","_app/immutable/assets/GraphPlot-74ddbde4.css","_app/immutable/assets/TopNav-6aef7732.css"];

export { file, imports, index, index_svelte as module, stylesheets };
//# sourceMappingURL=2-b2a07579.js.map
