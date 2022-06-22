
import { page } from "$app/stores";
import type {Load, LoadEvent} from "@sveltejs/kit/types"

export const backendUrl = (endpoint) => 'http://127.0.0.1:13000' + endpoint;

export type Fetch = (info: RequestInfo, init?: RequestInit)=> Promise<Response>;
export type Loader = {load: Load, refresh: ()=>Promise<JSON>}
export const melscan = async (endpoint: string | URL): Promise<JSON> => {
	const url = backendUrl(endpoint);
	const response = await fetch(url);
	console.log(`requesting ${endpoint}`)
	if (!response.ok) {
		throw `failed to fetch '${endpoint}' data`;
	}
	let res = response.json()
	return res;
};


export const load =  (endpoint?: string) => async (event: LoadEvent) => {
	let {url} = event;
	console.log('loading')
	return {
		status: 200,
		props: {
			data: await melscan(endpoint || url)
		}
	};
}

export const loader: (e: string) => Loader = (endpoint) => {
	return {
		load: load(endpoint),
        refresh: () => {
			let url: URL;
			page.subscribe(p=>url = p.url)()
			return melscan(url);
		}
	}
};
