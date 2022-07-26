import type { LoadEvent } from '@sveltejs/kit/types';

import type { GraphDatum, GraphQuery } from './page-types';

export const backendUrl = (endpoint) => 'http://127.0.0.1:13000' + endpoint;

export type Fetch = (info: RequestInfo, init?: RequestInit) => Promise<Response>;

export const melscan = async (fetch: Fetch, endpoint: string): Promise<any> => {
	const url = backendUrl(endpoint);
	const response = await fetch(url);
	console.log(`requesting ${url}`);
	if (!response.ok) {
		throw `failed to fetch '${url}' data`;
	}
	let res = response.json();
	return res;
};

export const queryGraph = async (query: GraphQuery): Promise<GraphDatum[]> => {
	const url = backendUrl('/raw/graph');
	let response = await fetch(url, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		credentials: 'omit',
		body: JSON.stringify(query)
	});
	let data = (await response.json()) as any[];
	// data now is an array of GraphDatums, except the data field is a string, so we loop over and change
	return data.map((elem) => {
		elem.date = new Date(elem.date);
		return elem as GraphDatum;
	});
};

export type EndpointLoader = (loadEvent: LoadEvent) => { [key: string]: string };

export type LoadFunction<T> = (
	loadEvent: LoadEvent<Record<string, string>, Record<string, any>>
) => Promise<T>;
// export type Loader<T> = (endpoint_loader: EndpointLoader) => LoadFunction<T>
// export const loader =  (endpoint_loader: EndpointLoader) => async (event: LoadEvent) => {
// 	let {url, fetch, params} = event;
// 	let sources_map = endpoint_loader(event)
// 	// console.log("Props: ", props);
// 	let sources = Object.values(sources_map);
// 	const refresh = ()=>Object.assign(Promise.all(Object.entries(sources_map).map(async (entry) => {
// 		let prop = entry[0];
// 		let domain = entry[1];
// 		console.log("hitting: ", domain);
// 		return {[prop]: await melscan(fetch, domain)}
// 	})))
// 	let data = await refresh();
// 	console.log(data);
// 	let props = Object.assign(...data);
// 	return {
// 		status: 200,
// 		props: {
// 			refresh,
// 			autorefresh: (interval?: number)=>{
// 				if(browser){
// 					console.log(browser)
// 					interval = interval || 1000;
// 					let interval_code = setInterval(async () => {
// 						// let v = await refresh()
// 						sources.map(i => {
// 							invalidate(i)
// 						})

// 					}, interval)
// 					console.log(interval_code)
// 					onDestroy(()=>clearInterval(interval_code))
// 				}
// 			},
// 			...props,
// 			params,
// 		}
// 	};
// }

// temp start
let handler = {
	get: function (target) {
		return '';
	}
};
export const tooltips = new Proxy({}, handler);
// temp end
