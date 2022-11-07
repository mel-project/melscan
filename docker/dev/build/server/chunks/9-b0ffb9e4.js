import { c as create_ssr_component, v as validate_component, a as each, e as escape, b as add_attribute } from './index-e26e4552-f700655a.js';
import { m as melscan, q as queryGraph } from './common-1a394d1a-ca1b3e74.js';
import { B as BreadCrumb, a as BreadCrumbs } from './page-types-9e95c4e2-05d65f96.js';
import { G as GraphPlot } from './GraphPlot-2db67e0a-3bd87ffa.js';
import { T as TopNav } from './TopNav-2806625e-0b2732a9.js';
import 'uplot';

let load = async (loadEvent) => {
  let props = {
    leaderboard: await melscan(loadEvent.fetch, "/raw/leaderboard/MEL")
  };
  return { status: 200, props };
};
const Stats = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  let breadcrumbs = [BreadCrumb("Melscan", "/"), BreadCrumb("Stats", ".")];
  let { leaderboard } = $$props;
  let sortedLeaderboard = Object.entries(leaderboard).sort((a, b) => b[1] - a[1]);
  let totalBalance = Object.values(leaderboard).reduce((a, b) => a + b);
  if ($$props.leaderboard === void 0 && $$bindings.leaderboard && leaderboard !== void 0)
    $$bindings.leaderboard(leaderboard);
  return `${validate_component(TopNav, "TopNav").$$render($$result, {}, {}, {
    default: () => {
      return `${validate_component(BreadCrumbs, "BreadCrumbs").$$render($$result, { breadcrumbs }, {}, {})}`;
    }
  })}

	<div class="${"container mx-auto max-w-screen-lg"}"><div class="${"grid grid-cols-1 md:grid-cols-2 mt-8 mb-8"}"><div class="${"col-span-2 mb-3"}"><h3 class="${"text-2xl font-bold"}">Money supply</h3></div></div>
		<div class="${"grid grid-cols-1 md:grid-cols-12 mt-8 mb-8"}"><div class="${"col-span-4"}"><b>MEL</b><br><br>
				${validate_component(GraphPlot, "GraphPlot").$$render(
    $$result,
    {
      fetchData: async (start, end) => await queryGraph({
        id: { type: "coin_supply", denom: "MEL" },
        start,
        end
      }),
      unit: "MEL",
      label: "Supply",
      stroke: "rgba(0, 0, 40, 1)",
      fill: "rgba(0, 0, 255, 0.1)"
    },
    {},
    {}
  )}</div>
			<div class="${"col-span-4"}"><b>SYM (PoS token)</b><br><br>
				${validate_component(GraphPlot, "GraphPlot").$$render(
    $$result,
    {
      fetchData: async (start, end) => await queryGraph({
        id: { type: "coin_supply", denom: "SYM" },
        start,
        end
      }),
      unit: "SYM",
      label: "Supply",
      stroke: "rgba(40, 0, 0, 1)",
      fill: "rgba(255, 0, 0, 0.1)"
    },
    {},
    {}
  )}</div>
			<div class="${"col-span-4"}"><b>ERG (minting intermediary)</b><br><br>
				${validate_component(GraphPlot, "GraphPlot").$$render(
    $$result,
    {
      fetchData: async (start, end) => await queryGraph({
        id: { type: "coin_supply", denom: "ERG" },
        start,
        end
      }),
      unit: "ERG",
      label: "Supply",
      stroke: "rgba(0, 40, 0, 1)",
      fill: "rgba(0, 255, 0, 0.1)"
    },
    {},
    {}
  )}</div></div></div>

	<div class="${"container mx-auto max-w-screen-lg mt-8 mb-8"}"><div class="${"col-span-2 mb-3"}"><h3 class="${"text-2xl font-bold"}">Top 50 addresses</h3></div>
		<table class="${"table-auto w-full mt-3"}"><thead class="${"text-left text-sm text-black text-opacity-50"}"><tr><th>Ranking</th>
					<th>Address</th>
					<th class="${"text-right"}">MEL balance</th>
					<th class="${"text-right"}">Percentage of supply</th></tr></thead>
			<tbody class="${"text-sm"}">${each(sortedLeaderboard, ([address, balance], i) => {
    return `${i < 50 ? `<tr><td>${escape(i + 1)}</td>
							<td><a${add_attribute("href", `/address/${address}`, 0)} class="${"text-blue-800"}">${escape(address)}</a></td>
							<td class="${"text-right"}">${escape(balance.toFixed(6))}</td>
							<td class="${"text-right"}">${escape((100 * balance / totalBalance).toFixed(4))}%</td>
						</tr>` : ``}`;
  })}</tbody></table></div>`;
});

var stats_svelte = /*#__PURE__*/Object.freeze({
  __proto__: null,
  'default': Stats,
  load: load
});

const index = 9;
const file = '_app/immutable/pages/stats.svelte-0bfad4fc.js';
const imports = ["_app/immutable/pages/stats.svelte-0bfad4fc.js","_app/immutable/chunks/index-d5cc66c1.js","_app/immutable/chunks/common-9379a3d9.js","_app/immutable/chunks/singletons-eca981c1.js","_app/immutable/chunks/page-types-6d3ebd03.js","_app/immutable/chunks/GraphPlot-f03243f5.js","_app/immutable/chunks/index-74e7b1a8.js","_app/immutable/chunks/TopNav-61067f92.js"];
const stylesheets = ["_app/immutable/assets/BreadCrumbs-4f9b45d3.css","_app/immutable/assets/GraphPlot-74ddbde4.css","_app/immutable/assets/TopNav-6aef7732.css"];

export { file, imports, index, stats_svelte as module, stylesheets };
//# sourceMappingURL=9-b0ffb9e4.js.map
