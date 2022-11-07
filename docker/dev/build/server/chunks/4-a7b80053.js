import { c as create_ssr_component, v as validate_component, e as escape, a as each, b as add_attribute } from './index-e26e4552-f700655a.js';
import 'd3-sankey';
import { m as melscan, t as tooltips } from './common-1a394d1a-ca1b3e74.js';
import { B as BreadCrumb, a as BreadCrumbs } from './page-types-9e95c4e2-05d65f96.js';
import { T as TopNav } from './TopNav-2806625e-0b2732a9.js';
import { C as CoinDag } from './CoinDag-f56c1ef3-3d396e09.js';
import 'vis-network';
import 'vis-data';

const css = {
  code: "td.svelte-11ygxhc{vertical-align:top}.dag.svelte-11ygxhc{height:20rem}.data-field.svelte-11ygxhc{max-height:20rem;overflow-y:auto;overflow-x:hidden;word-break:break-all;margin-bottom:1rem}",
  map: null
};
let load = async (loadEvent) => {
  let { height, txhash } = loadEvent.params;
  let url = `/raw/blocks/${height}/${txhash}`;
  let res = await melscan(loadEvent.fetch, url);
  return { status: 200, props: res };
};
function print_coin(coin) {
  return `${coin[0]} ${coin[1]}`;
}
const U5Btxhashu5D = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  let breadcrumbs;
  let { txhash } = $$props;
  let { txhash_abbr } = $$props;
  let { height } = $$props;
  let { transaction } = $$props;
  let { inputs_with_cdh } = $$props;
  let { outputs } = $$props;
  let { fee } = $$props;
  let { base_fee } = $$props;
  let { tips } = $$props;
  let { net_loss } = $$props;
  let { net_gain } = $$props;
  let { gross_gain } = $$props;
  let { weight } = $$props;
  let { kind } = $$props;
  let { covenants } = $$props;
  if ($$props.txhash === void 0 && $$bindings.txhash && txhash !== void 0)
    $$bindings.txhash(txhash);
  if ($$props.txhash_abbr === void 0 && $$bindings.txhash_abbr && txhash_abbr !== void 0)
    $$bindings.txhash_abbr(txhash_abbr);
  if ($$props.height === void 0 && $$bindings.height && height !== void 0)
    $$bindings.height(height);
  if ($$props.transaction === void 0 && $$bindings.transaction && transaction !== void 0)
    $$bindings.transaction(transaction);
  if ($$props.inputs_with_cdh === void 0 && $$bindings.inputs_with_cdh && inputs_with_cdh !== void 0)
    $$bindings.inputs_with_cdh(inputs_with_cdh);
  if ($$props.outputs === void 0 && $$bindings.outputs && outputs !== void 0)
    $$bindings.outputs(outputs);
  if ($$props.fee === void 0 && $$bindings.fee && fee !== void 0)
    $$bindings.fee(fee);
  if ($$props.base_fee === void 0 && $$bindings.base_fee && base_fee !== void 0)
    $$bindings.base_fee(base_fee);
  if ($$props.tips === void 0 && $$bindings.tips && tips !== void 0)
    $$bindings.tips(tips);
  if ($$props.net_loss === void 0 && $$bindings.net_loss && net_loss !== void 0)
    $$bindings.net_loss(net_loss);
  if ($$props.net_gain === void 0 && $$bindings.net_gain && net_gain !== void 0)
    $$bindings.net_gain(net_gain);
  if ($$props.gross_gain === void 0 && $$bindings.gross_gain && gross_gain !== void 0)
    $$bindings.gross_gain(gross_gain);
  if ($$props.weight === void 0 && $$bindings.weight && weight !== void 0)
    $$bindings.weight(weight);
  if ($$props.kind === void 0 && $$bindings.kind && kind !== void 0)
    $$bindings.kind(kind);
  if ($$props.covenants === void 0 && $$bindings.covenants && covenants !== void 0)
    $$bindings.covenants(covenants);
  $$result.css.add(css);
  breadcrumbs = [
    BreadCrumb("Melscan", "/"),
    BreadCrumb(`Block ${height}`, `.`),
    BreadCrumb(`Transaction ${txhash.substring(0, 10)}..`, "")
  ];
  return `${validate_component(TopNav, "TopNav").$$render($$result, {}, {}, {
    default: () => {
      return `${validate_component(BreadCrumbs, "BreadCrumbs").$$render($$result, { breadcrumbs }, {}, {})}`;
    }
  })}
	<div class="${"container mx-auto max-w-screen-lg"}"><div class="${"mb-3 mt-8"}"><h3 class="${"text-2xl font-bold"}">Summary</h3></div>

		<div class="${"m-3"}"><table class="${"table-fixed w-full text-sm text-left"}"><tbody><tr><td class="${"text-black text-opacity-50 font-bold w-1/3 svelte-11ygxhc"}">Height</td>
						<td class="${"svelte-11ygxhc"}"><a href="${"/blocks/" + escape(height, true)}" class="${"text-blue-800 font-medium"}">${escape(height)}</a></td></tr>
					<tr><td class="${"text-black text-opacity-50 font-bold svelte-11ygxhc"}"><span class="${"name"}">Kind</span>
							${escape(tooltips["kind"])}</td>
						<td class="${"font-medium svelte-11ygxhc"}">${escape(kind)}</td></tr>
					<tr><td class="${"text-black text-opacity-50 font-bold svelte-11ygxhc"}"><span class="${"name"}">Hash</span></td>
						<td class="${"font-medium mono overflow-ellipsis overflow-hidden svelte-11ygxhc"}">${escape(txhash)}</td></tr>

					<td class="${"text-black text-opacity-50 font-bold w-1/3 svelte-11ygxhc"}">Total output</td>
					<td class="${"svelte-11ygxhc"}">${each(gross_gain, (gain_entry) => {
    return `${escape(gain_entry[0])} ${escape(gain_entry[1])} ${escape("  ")}`;
  })}</td>
					<tr><td class="${"text-black text-opacity-50 font-bold svelte-11ygxhc"}"><span class="${"name"}">Fee</span>
							${escape(tooltips["fee"])}</td>
						<td class="${"svelte-11ygxhc"}">${escape(print_coin(fee))}<br>
							<span class="${"text-black text-opacity-50"}">${escape(print_coin(base_fee))} <i>base</i></span><br>
							<span class="${"text-black text-opacity-50"}">${escape(print_coin(tips))} <i>tips</i></span><br></td></tr>
					<tr><td class="${"text-black text-opacity-50 font-bold svelte-11ygxhc"}"><span class="${"name"}">Weight</span>
							${escape(tooltips["weight"])}</td>
						<td class="${"svelte-11ygxhc"}">${escape(weight)} wu</td></tr></tbody></table></div>
		<div class="${"grid grid-cols-1 text-sm"}"><div class="${"m-3"}"><span class="${"text-black text-opacity-50 font-bold"}"><span class="${"name"}">Net Senders</span>
					${escape(tooltips["netSenders"])}
				</span><br>
				<table class="${"table-fixed w-full text-left"}"><tbody>${each(Object.entries(net_loss), (entry) => {
    return `<tr><td class="${"overflow-ellipsis overflow-hidden svelte-11ygxhc"}"><a class="${"text-blue-800"}"${add_attribute("href", `/address/${entry[0]}`, 0)}>${escape(entry[0])}</a></td>
								<td class="${"font-medium svelte-11ygxhc"}" style="${"color: #a22041"}">${escape(entry[1][0][0])} ${escape(entry[1][0][1])}</td>
							</tr>`;
  })}</tbody></table></div>

			<div class="${"m-3"}"><span class="${"text-black text-opacity-50 font-bold"}"><span class="${"name"}">Net Receivers</span>
					${escape(tooltips["netReceivers"])}
				</span><br>
				<table class="${"table-fixed w-full text-left"}"><tbody>${each(Object.entries(net_gain), (entry) => {
    return `<tr><td class="${"overflow-ellipsis overflow-hidden svelte-11ygxhc"}"><a class="${"text-blue-800"}"${add_attribute("href", `/address/${entry[0]}`, 0)}>${escape(entry[0])}</a></td>
								<td class="${"font-medium svelte-11ygxhc"}" style="${"color:#007b43"}">${escape(entry[1][0][0])} ${escape(entry[1][0][1])}</td>
							</tr>`;
  })}

						<tr><td class="${"svelte-11ygxhc"}"><i>(Total fees)</i></td>
							<td class="${"font-medium svelte-11ygxhc"}" style="${"color: #007b43"}">${escape(fee[0])}
								${escape(fee[1])}</td></tr></tbody></table></div></div>

		<div class="${"mb-3 mt-8"}"><h3 class="${"text-2xl font-bold"}">Coin graph</h3>
			<p><a${add_attribute("href", `/blocks/${height}/${txhash}/explore`, 0)} class="${"text-blue-800"}">(see in explorer)</a></p></div>

		<div class="${"mb-3 mt-8"}"><div class="${"dag svelte-11ygxhc"}">
					${validate_component(CoinDag, "CoinDag").$$render($$result, { embed: true, height, txhash }, {}, {})}</div></div>

		<div class="${"mb-3 mt-8"}"><h3 class="${"text-2xl font-bold"}">Covenants</h3></div>

		<div class="${"m-3"}"><table class="${"table-fixed w-full text-sm text-left"}"><tbody>${each(covenants, ([covhash, covenant]) => {
    return `<tr><td class="${"text-black text-opacity-50 font-bold overflow-ellipsis overflow-hidden svelte-11ygxhc"}"><span class="${"name"}">${escape(covhash)}</span></td>
							<td class="${"svelte-11ygxhc"}"><div class="${"data-field svelte-11ygxhc"}">${each(covenant, (opcode) => {
      return `${escape(opcode.toLowerCase())} <br>`;
    })}
								</div></td>
						</tr>`;
  })}</tbody></table></div>

		<div class="${"mb-3 mt-8"}"><h3 class="${"text-2xl font-bold"}">Unlock inputs</h3></div>

		<div class="${"m-3"}"><table class="${"table-fixed w-full text-sm text-left"}"><tbody><tr><td class="${"text-black text-opacity-50 font-bold overflow-ellipsis overflow-hidden w-1/3 svelte-11ygxhc"}"><span class="${"name"}">Data field (${escape(transaction.data.length / 2)} bytes)</span></td>
						<td class="${"w-2/3 svelte-11ygxhc"}"><div class="${"data-field mono svelte-11ygxhc"}">${escape(transaction.data)}</div></td></tr>
					${each(transaction.sigs, (signature, i) => {
    return `<tr><td class="${"text-black text-opacity-50 font-bold overflow-ellipsis overflow-hidden w-1/3 svelte-11ygxhc"}"><span class="${"name"}">Signature ${escape(i)} (${escape(signature.length / 2)} bytes)</span></td>
							<td class="${"mono w-2/3 break-all svelte-11ygxhc"}">${escape(signature)}</td>
						</tr>`;
  })}</tbody></table></div>

		<div class="${"mb-3 mt-8"}"><h3 class="${"text-2xl font-bold"}">Inputs</h3></div>

		<div class="${"m-3"}">${each(inputs_with_cdh, ([index, input, cdh, value, additional_data, recipient]) => {
    return `<table class="${"table-fixed w-full text-sm text-left mt-1 mb-1"}"><tbody><tr><td class="${"text-black text-opacity-50 font-bold w-1/3 svelte-11ygxhc"}">Index</td>
							<td class="${"svelte-11ygxhc"}">${escape(index)}</td></tr>
						<tr><td class="${"text-black text-opacity-50 font-bold  svelte-11ygxhc"}">Spent CoinID</td>
							<td class="${"overflow-ellipsis overflow-hidden svelte-11ygxhc"}"><a class="${"text-blue-800 mono "}" href="${"/blocks/" + escape(cdh.height, true) + "/" + escape(input.txhash, true)}">${escape(input.txhash)}</a>-${escape(input.index)}</td></tr>
						<tr><td class="${"text-black text-opacity-50 font-bold svelte-11ygxhc"}">Value</td>
							<td class="${"overflow-ellipsis overflow-hidden svelte-11ygxhc"}">${escape(value[0])} ${escape(value[1])}</td></tr>
						<tr><td class="${"text-black text-opacity-50 font-bold  svelte-11ygxhc"}">Recipient</td>
							<td class="${"overflow-ellipsis overflow-hidden svelte-11ygxhc"}">${escape(recipient)}</td></tr>
						<tr><td class="${"text-black text-opacity-50 font-bold svelte-11ygxhc"}">Additional data</td>
							<td class="${"svelte-11ygxhc"}">${escape(additional_data || '""')}</td>
						</tr></tbody>
				</table>`;
  })}</div>

		<div class="${"mb-3 mt-8"}"><h3 class="${"text-2xl font-bold"}">Outputs</h3></div>

		<div class="${"m-3"}">${each(outputs, ([index, coin_data, value, additional_data, recipient]) => {
    return `<table class="${"table-fixed w-full text-sm text-left mt-1 mb-1"}"><tbody><tr><td class="${"text-black text-opacity-50 font-bold w-1/3 svelte-11ygxhc"}">Index</td>
							<td class="${"svelte-11ygxhc"}">${escape(index)}</td></tr>
						<tr><td class="${"text-black text-opacity-50 font-bold svelte-11ygxhc"}">Value</td>
							<td class="${"svelte-11ygxhc"}">${escape(value[0])} ${escape(value[1])}</td></tr>
						<tr><td class="${"text-black text-opacity-50 font-bold svelte-11ygxhc"}">Recipient</td>
							<td class="${"overflow-ellipsis overflow-hidden svelte-11ygxhc"}">${escape(recipient)}</td></tr>
						<tr><td class="${"text-black text-opacity-50 font-bold svelte-11ygxhc"}">Additional data</td>
							<td class="${"svelte-11ygxhc"}">${escape(additional_data || '""')}</td>
						</tr></tbody>
				</table>`;
  })}</div>
	</div>`;
});

var index_svelte = /*#__PURE__*/Object.freeze({
  __proto__: null,
  'default': U5Btxhashu5D,
  load: load
});

const index = 4;
const file = '_app/immutable/pages/blocks/_height_/_txhash_/index.svelte-e7f60b1a.js';
const imports = ["_app/immutable/pages/blocks/_height_/_txhash_/index.svelte-e7f60b1a.js","_app/immutable/chunks/index-d5cc66c1.js","_app/immutable/chunks/common-9379a3d9.js","_app/immutable/chunks/singletons-eca981c1.js","_app/immutable/chunks/page-types-6d3ebd03.js","_app/immutable/chunks/TopNav-61067f92.js","_app/immutable/chunks/CoinDag-7236be9e.js"];
const stylesheets = ["_app/immutable/assets/index-ed3ae8e7.css","_app/immutable/assets/BreadCrumbs-4f9b45d3.css","_app/immutable/assets/TopNav-6aef7732.css","_app/immutable/assets/CoinDag-3faf1cb9.css"];

export { file, imports, index, index_svelte as module, stylesheets };
//# sourceMappingURL=4-a7b80053.js.map
