import { c as create_ssr_component, b as add_attribute, e as escape } from './index-e26e4552-f700655a.js';
import './common-1a394d1a-ca1b3e74.js';
import 'vis-network';
import 'vis-data';

const css = {
  code: ".pop.svelte-1shtdo9{position:absolute;top:2rem;left:2rem;z-index:2000;background-color:white;border:1px solid #aaa;padding:1rem}.superroot.svelte-1shtdo9{height:100%;position:relative}.root.svelte-1shtdo9{height:100%;border:1px solid #aaa;background-color:white}.loading.svelte-1shtdo9{opacity:0.5;pointer-events:none;background-color:#fefefe}",
  map: null
};
const CoinDag = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  let { height } = $$props;
  let { txhash } = $$props;
  let { embed = false } = $$props;
  let container;
  if ($$props.height === void 0 && $$bindings.height && height !== void 0)
    $$bindings.height(height);
  if ($$props.txhash === void 0 && $$bindings.txhash && txhash !== void 0)
    $$bindings.txhash(txhash);
  if ($$props.embed === void 0 && $$bindings.embed && embed !== void 0)
    $$bindings.embed(embed);
  $$result.css.add(css);
  return `<div class="${"superroot svelte-1shtdo9"}">${!embed ? `<div class="${"pop svelte-1shtdo9"}">Transaction <a${add_attribute("href", `/blocks/${height}/${txhash}`, 0)} class="${"text-blue-800 font-bold"}">${escape(txhash)}</a></div>` : ``}
		<div class="${["root svelte-1shtdo9", ""].join(" ").trim()}"${add_attribute("this", container, 0)}></div>
	</div>`;
});

export { CoinDag as C };
//# sourceMappingURL=CoinDag-f56c1ef3-3d396e09.js.map
