export const backendUrl = (endpoint) => 'http://127.0.0.1:13000' + endpoint;

const queryBackend = async (fetch, endpoint) => {
	const url = backendUrl(endpoint);
	const response = await fetch(url);
	if (!response.ok) {
		throw `failed to fetch ${endpoint} data`;
	}
	return await response.json();
};

export const loader = (endpoint) => {
	return {
		load: async ({ params, fetch, session, stuff }) => {
			return {
				status: 200,
				props: {
					data: await queryBackend(fetch, endpoint)
				}
			};
		},
        query: fetch => queryBackend(fetch, endpoint)
	}
};
