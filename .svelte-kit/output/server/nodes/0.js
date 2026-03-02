

export const index = 0;
let component_cache;
export const component = async () => component_cache ??= (await import('../entries/pages/_layout.svelte.js')).default;
export const universal = {
  "prerender": true,
  "ssr": false
};
export const universal_id = "src/routes/+layout.ts";
export const imports = ["_app/immutable/nodes/0.CDc7A8E7.js","_app/immutable/chunks/DBDMbGDQ.js","_app/immutable/chunks/Da2QqrXs.js","_app/immutable/chunks/Dy2HbpQc.js","_app/immutable/chunks/BO-c5S7Y.js","_app/immutable/chunks/D_9tBarb.js","_app/immutable/chunks/DJtdpqmd.js"];
export const stylesheets = ["_app/immutable/assets/0.BM3uZKYm.css"];
export const fonts = [];
