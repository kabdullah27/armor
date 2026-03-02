export const manifest = (() => {
function __memo(fn) {
	let value;
	return () => value ??= (value = fn());
}

return {
	appDir: "_app",
	appPath: "_app",
	assets: new Set(["css/core.min.css","css/franken-ui.css","icons/128x128.png","icons/128x128@2x.png","icons/32x32.png","icons/Square107x107Logo.png","icons/Square142x142Logo.png","icons/Square150x150Logo.png","icons/Square284x284Logo.png","icons/Square30x30Logo.png","icons/Square310x310Logo.png","icons/Square44x44Logo.png","icons/Square71x71Logo.png","icons/Square89x89Logo.png","icons/StoreLogo.png","icons/icon.png"]),
	mimeTypes: {".css":"text/css",".png":"image/png"},
	_: {
		client: {start:"_app/immutable/entry/start.CUP1gag6.js",app:"_app/immutable/entry/app.BFjgyvEz.js",imports:["_app/immutable/entry/start.CUP1gag6.js","_app/immutable/chunks/DJtdpqmd.js","_app/immutable/chunks/DBDMbGDQ.js","_app/immutable/chunks/BO-c5S7Y.js","_app/immutable/entry/app.BFjgyvEz.js","_app/immutable/chunks/C1FmrZbK.js","_app/immutable/chunks/DBDMbGDQ.js","_app/immutable/chunks/Da2QqrXs.js"],stylesheets:[],fonts:[],uses_env_dynamic_public:false},
		nodes: [
			__memo(() => import('./nodes/0.js')),
			__memo(() => import('./nodes/1.js')),
			__memo(() => import('./nodes/2.js')),
			__memo(() => import('./nodes/3.js')),
			__memo(() => import('./nodes/4.js')),
			__memo(() => import('./nodes/5.js')),
			__memo(() => import('./nodes/6.js'))
		],
		remotes: {
			
		},
		routes: [
			{
				id: "/",
				pattern: /^\/$/,
				params: [],
				page: { layouts: [0,], errors: [1,], leaf: 2 },
				endpoint: null
			},
			{
				id: "/decrypt",
				pattern: /^\/decrypt\/?$/,
				params: [],
				page: { layouts: [0,], errors: [1,], leaf: 3 },
				endpoint: null
			},
			{
				id: "/encrypt",
				pattern: /^\/encrypt\/?$/,
				params: [],
				page: { layouts: [0,], errors: [1,], leaf: 4 },
				endpoint: null
			},
			{
				id: "/keys",
				pattern: /^\/keys\/?$/,
				params: [],
				page: { layouts: [0,], errors: [1,], leaf: 5 },
				endpoint: null
			},
			{
				id: "/settings",
				pattern: /^\/settings\/?$/,
				params: [],
				page: { layouts: [0,], errors: [1,], leaf: 6 },
				endpoint: null
			}
		],
		prerendered_routes: new Set([]),
		matchers: async () => {
			
			return {  };
		},
		server_assets: {}
	}
}
})();
