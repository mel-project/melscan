import { c as create_ssr_component } from './index-e26e4552-f700655a.js';
import { i as isTestnet } from './common-1a394d1a-ca1b3e74.js';

const css = {
  code: "nav.svelte-oa6t0o{text-overflow:ellipsis;overflow-x:hidden}.outer.svelte-oa6t0o{display:flex;flex-direction:row;width:100%;justify-content:space-between}.inner.svelte-oa6t0o{flex-grow:1;margin-left:0.5rem}.second.svelte-oa6t0o{font-size:1rem}",
  map: null
};
const TopNav = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  let { testnet = isTestnet } = $$props;
  if ($$props.testnet === void 0 && $$bindings.testnet && testnet !== void 0)
    $$bindings.testnet(testnet);
  $$result.css.add(css);
  return `<nav class="${"mx-auto max-w-screen-lg mt-8 text-xl leading-tight outer svelte-oa6t0o"}">${testnet ? `<div class="${"font-bold text-black italic text-opacity-50 tnet"}">Testnet</div>` : ``}
	<div class="${"inner svelte-oa6t0o"}">${slots.default ? slots.default({}) : ``}
		<div class="${"text-sm text-blue-600 hover:underline"}"><b>${testnet ? `<a href="${"https://scan.themelio.org"}">Switch to mainnet</a>` : `<a href="${"https://scan-testnet.themelio.org"}">Switch to testnet</a>`}</b></div></div>
	<div class="${"second svelte-oa6t0o"}"><a href="${"/stats"}">[stats]</a>\xA0<a href="${"https://github.com/themeliolabs/melscan"}">[github]</a></div>
</nav>`;
});

export { TopNav as T };
//# sourceMappingURL=TopNav-2806625e-0b2732a9.js.map
