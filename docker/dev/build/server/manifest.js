const manifest = {
	appDir: "_app",
	assets: new Set([]),
	mimeTypes: {},
	_: {
		entry: {"file":"_app/immutable/start-8147eba8.js","imports":["_app/immutable/start-8147eba8.js","_app/immutable/chunks/index-d5cc66c1.js","_app/immutable/chunks/index-74e7b1a8.js","_app/immutable/chunks/singletons-eca981c1.js"],"stylesheets":[]},
		nodes: [
			() => import('./chunks/0-e530bfee.js'),
			() => import('./chunks/1-3aa0f176.js'),
			() => import('./chunks/7-10d583b3.js'),
			() => import('./chunks/9-b0ffb9e4.js'),
			() => import('./chunks/2-b2a07579.js'),
			() => import('./chunks/6-36b3a02a.js'),
			() => import('./chunks/4-a7b80053.js'),
			() => import('./chunks/8-e6b8e1f1.js'),
			() => import('./chunks/3-3cc0a9e6.js'),
			() => import('./chunks/5-06cc97ee.js')
		],
		routes: [
			{
				type: 'page',
				id: "",
				pattern: /^\/$/,
				names: [],
				types: [],
				path: "/",
				shadow: null,
				a: [0,2],
				b: [1]
			},
			{
				type: 'page',
				id: "stats",
				pattern: /^\/stats\/?$/,
				names: [],
				types: [],
				path: "/stats",
				shadow: null,
				a: [0,3],
				b: [1]
			},
			{
				type: 'page',
				id: "address/[covhash]",
				pattern: /^\/address\/([^/]+?)\/?$/,
				names: ["covhash"],
				types: [null],
				path: null,
				shadow: null,
				a: [0,4],
				b: [1]
			},
			{
				type: 'page',
				id: "blocks/[height]",
				pattern: /^\/blocks\/([^/]+?)\/?$/,
				names: ["height"],
				types: [null],
				path: null,
				shadow: null,
				a: [0,5],
				b: [1]
			},
			{
				type: 'page',
				id: "blocks/[height]/[txhash]",
				pattern: /^\/blocks\/([^/]+?)\/([^/]+?)\/?$/,
				names: ["height","txhash"],
				types: [null,null],
				path: null,
				shadow: null,
				a: [0,6],
				b: [1]
			},
			{
				type: 'page',
				id: "pools/[left]/[right]",
				pattern: /^\/pools\/([^/]+?)\/([^/]+?)\/?$/,
				names: ["left","right"],
				types: [null,null],
				path: null,
				shadow: null,
				a: [0,7],
				b: [1]
			},
			{
				type: 'page',
				id: "blocks/[height]/[txhash]/explore",
				pattern: /^\/blocks\/([^/]+?)\/([^/]+?)\/explore\/?$/,
				names: ["height","txhash"],
				types: [null,null],
				path: null,
				shadow: null,
				a: [0,8],
				b: [1]
			},
			{
				type: 'page',
				id: "blocks/[height]/[txhash]/spenders",
				pattern: /^\/blocks\/([^/]+?)\/([^/]+?)\/spenders\/?$/,
				names: ["height","txhash"],
				types: [null,null],
				path: null,
				shadow: null,
				a: [0,9],
				b: [1]
			}
		],
		matchers: async () => {
			
			return {  };
		}
	}
};

export { manifest };
//# sourceMappingURL=manifest.js.map
