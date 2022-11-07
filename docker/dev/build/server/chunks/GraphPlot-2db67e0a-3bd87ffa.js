import { c as create_ssr_component, d as subscribe, e as escape, b as add_attribute, n as noop, f as safe_not_equal } from './index-e26e4552-f700655a.js';
import './common-1a394d1a-ca1b3e74.js';
import 'uplot';

const subscriber_queue = [];
function writable(value, start = noop) {
  let stop;
  const subscribers = /* @__PURE__ */ new Set();
  function set(new_value) {
    if (safe_not_equal(value, new_value)) {
      value = new_value;
      if (stop) {
        const run_queue = !subscriber_queue.length;
        for (const subscriber of subscribers) {
          subscriber[1]();
          subscriber_queue.push(subscriber, value);
        }
        if (run_queue) {
          for (let i = 0; i < subscriber_queue.length; i += 2) {
            subscriber_queue[i][0](subscriber_queue[i + 1]);
          }
          subscriber_queue.length = 0;
        }
      }
    }
  }
  function update(fn) {
    set(fn(value));
  }
  function subscribe2(run, invalidate = noop) {
    const subscriber = [run, invalidate];
    subscribers.add(subscriber);
    if (subscribers.size === 1) {
      stop = start(set) || noop;
    }
    run(value);
    return () => {
      subscribers.delete(subscriber);
      if (subscribers.size === 0) {
        stop();
        stop = null;
      }
    };
  }
  return { set, update, subscribe: subscribe2 };
}
const css = {
  code: ".loading.svelte-1tcqiwl{opacity:0.4}.uplot{position:relative}.u-legend{position:absolute !important;top:10px !important;left:15%}",
  map: null
};
const GraphPlot = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  let $loading, $$unsubscribe_loading;
  let { fetchData } = $$props;
  let { initStart = null } = $$props;
  let { initEnd = null } = $$props;
  let { title = "" } = $$props;
  let { unit = "" } = $$props;
  let { label = "Value" } = $$props;
  let { height = "20rem" } = $$props;
  let { stroke = "black" } = $$props;
  let { fill = "rgba(0, 0, 0, 0.1)" } = $$props;
  let { stepped = false } = $$props;
  let container;
  let loading = writable(false);
  $$unsubscribe_loading = subscribe(loading, (value) => $loading = value);
  if ($$props.fetchData === void 0 && $$bindings.fetchData && fetchData !== void 0)
    $$bindings.fetchData(fetchData);
  if ($$props.initStart === void 0 && $$bindings.initStart && initStart !== void 0)
    $$bindings.initStart(initStart);
  if ($$props.initEnd === void 0 && $$bindings.initEnd && initEnd !== void 0)
    $$bindings.initEnd(initEnd);
  if ($$props.title === void 0 && $$bindings.title && title !== void 0)
    $$bindings.title(title);
  if ($$props.unit === void 0 && $$bindings.unit && unit !== void 0)
    $$bindings.unit(unit);
  if ($$props.label === void 0 && $$bindings.label && label !== void 0)
    $$bindings.label(label);
  if ($$props.height === void 0 && $$bindings.height && height !== void 0)
    $$bindings.height(height);
  if ($$props.stroke === void 0 && $$bindings.stroke && stroke !== void 0)
    $$bindings.stroke(stroke);
  if ($$props.fill === void 0 && $$bindings.fill && fill !== void 0)
    $$bindings.fill(fill);
  if ($$props.stepped === void 0 && $$bindings.stepped && stepped !== void 0)
    $$bindings.stepped(stepped);
  $$result.css.add(css);
  $$unsubscribe_loading();
  return `<div id="${"container"}" style="${"height: " + escape(height, true)}" class="${["svelte-1tcqiwl", $loading ? "loading" : ""].join(" ").trim()}"${add_attribute("this", container, 0)}></div>`;
});

export { GraphPlot as G };
//# sourceMappingURL=GraphPlot-2db67e0a-3bd87ffa.js.map
