import { c as create_ssr_component, a as each, b as add_attribute, e as escape } from './index-e26e4552-f700655a.js';

/* empty css                                                    */const css = {
  code: ".breadcrumb.svelte-hwbu88:first-child{margin-left:0}.breadcrumb.svelte-hwbu88{color:gray;margin:0 0.1em 0 0.3em}.breadcrumb.svelte-hwbu88:last-child{color:black}",
  map: null
};
const BreadCrumbs = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  let { breadcrumbs } = $$props;
  if ($$props.breadcrumbs === void 0 && $$bindings.breadcrumbs && breadcrumbs !== void 0)
    $$bindings.breadcrumbs(breadcrumbs);
  $$result.css.add(css);
  return `<div class="${"breadcrumbs"}">${each(breadcrumbs, (breadcrumb, index) => {
    return `<a class="${"breadcrumb svelte-hwbu88"}"${add_attribute("href", breadcrumb.href, 0)}>${escape(breadcrumb.title)}</a>
			${index + 1 !== breadcrumbs.length ? `\u{1F892}` : ``}`;
  })}
	</div>`;
});
let BreadCrumb = (title, href) => {
  return {
    title,
    href
  };
};

export { BreadCrumb as B, BreadCrumbs as a };
//# sourceMappingURL=page-types-9e95c4e2-05d65f96.js.map
