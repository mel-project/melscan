
import type {Load, LoadEvent} from "@sveltejs/kit/types"

export const backendUrl = (endpoint) => 'http://127.0.0.1:13000' + endpoint;

export const melscan = async (fetch, endpoint): Promise<Response> => {
	const url = backendUrl(endpoint);
	const response = await fetch(url);
	if (!response.ok) {
		throw `failed to fetch '${endpoint}' data`;
	}
	return await response.json();
};

export type Fetch = (info: RequestInfo, init?: RequestInit)=> Promise<Response>;
export type Loader = {load: Load, query: (Fetch)=>Promise<Response>}

export const load =  endpoint => async (event: LoadEvent) => {
	let {fetch, url} = event;
	return {
		status: 200,
		props: {
			data: await melscan(fetch, endpoint)
		}
	};
}

export const loader: (e: string) => Loader = (endpoint) => {
	return {
		load: load(endpoint),
        query: fetch => melscan(fetch, endpoint)
	}
};
