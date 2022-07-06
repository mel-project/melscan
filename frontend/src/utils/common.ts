
import { browser } from "$app/env";
import { invalidate } from "$app/navigation";
import { getStores } from "$app/stores";
import type {Load, LoadEvent} from "@sveltejs/kit/types"

export const backendUrl = (endpoint) => 'http://127.0.0.1:13000' + endpoint;

export type Fetch = (info: RequestInfo, init?: RequestInit)=> Promise<Response>;

export const url_mapping = {
	'/': ['/raw/overview']
}

export const melscan = async (fetch: Fetch, endpoint: string): Promise<JSON> => {
	const url = backendUrl(endpoint);
	const response = await fetch(url);
	console.log(`requesting ${endpoint}`)
	if (!response.ok) {
		throw `failed to fetch '${url}' data`;
	}
	let res = response.json()
	return res;
};


export type EndpointLoader =  (loadEvent: LoadEvent) => string | [string];
export const loader =  (endpoints?: string | [string] | EndpointLoader) => async (event: LoadEvent) => {
	let {url, fetch, params} = event;

	if(typeof endpoints == "function"){
		endpoints = endpoints(event)
	}

	if(typeof endpoints == "string"){
		endpoints = [endpoints]
	}
	let sources: string[] = endpoints || url_mapping[url.pathname] || [url.pathname]
	let data: JSON[] = (await Promise.all(sources.map(e => melscan(fetch,e)))).flat();
	let props = Object.assign(...data);
	params = Object.assign({}, params)
	console.log("params: ",params);
	const refresh = ()=>Promise.all(sources.map(e => melscan(fetch, e)))
	return {
		status: 200,
		props: {
			refresh,
			autorefresh: ()=>{	
				setInterval(async () => {
					let v = await refresh()
					.catch((e)=>console.error(e))
					.then(async data=> {
						if(browser)
							invalidate(url.href);
					})
				}, 1000)
			},
			...props,
			params,
		}
	};
}

export const load = loader()