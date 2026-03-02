import{c as u,s as g,v as f,m as h}from"./ssr.js";import{a as y}from"./ssr2.js";import"./environment.js";let x={};function O(e){}function U(e){x=e}let k=null;function q(e){k=e}function z(e){}const w=u((e,t,n,v)=>{let{stores:o}=t,{page:r}=t,{constructors:s}=t,{components:a=[]}=t,{form:d}=t,{data_0:c=null}=t,{data_1:m=null}=t;g("__svelte__",o),y(o.page.notify),t.stores===void 0&&n.stores&&o!==void 0&&n.stores(o),t.page===void 0&&n.page&&r!==void 0&&n.page(r),t.constructors===void 0&&n.constructors&&s!==void 0&&n.constructors(s),t.components===void 0&&n.components&&a!==void 0&&n.components(a),t.form===void 0&&n.form&&d!==void 0&&n.form(d),t.data_0===void 0&&n.data_0&&c!==void 0&&n.data_0(c),t.data_1===void 0&&n.data_1&&m!==void 0&&n.data_1(m);let i,_,p=e.head;do i=!0,e.head=p,o.page.set(r),_=`  ${s[1]?`${f(s[0]||h,"svelte:component").$$render(e,{data:c,params:r.params,this:a[0]},{this:l=>{a[0]=l,i=!1}},{default:()=>`${f(s[1]||h,"svelte:component").$$render(e,{data:m,form:d,params:r.params,this:a[1]},{this:l=>{a[1]=l,i=!1}},{})}`})}`:`${f(s[0]||h,"svelte:component").$$render(e,{data:c,form:d,params:r.params,this:a[0]},{this:l=>{a[0]=l,i=!1}},{})}`} `;while(!i);return _}),F={app_template_contains_nonce:!1,async:!1,csp:{mode:"auto",directives:{"upgrade-insecure-requests":!1,"block-all-mixed-content":!1},reportOnly:{"upgrade-insecure-requests":!1,"block-all-mixed-content":!1}},csrf_check_origin:!0,csrf_trusted_origins:[],embedded:!1,env_public_prefix:"PUBLIC_",env_private_prefix:"",hash_routing:!1,hooks:null,preload_strategy:"modulepreload",root:w,service_worker:!1,service_worker_options:void 0,templates:{app:({head:e,body:t,assets:n,nonce:v,env:o})=>`<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>Armor</title>
    `+e+`
  </head>
  <body>
    <div style="display: contents">`+t+`</div>
  </body>
</html>
`,error:({status:e,message:t})=>`<!doctype html>
<html lang="en">
	<head>
		<meta charset="utf-8" />
		<title>`+t+`</title>

		<style>
			body {
				--bg: white;
				--fg: #222;
				--divider: #ccc;
				background: var(--bg);
				color: var(--fg);
				font-family:
					system-ui,
					-apple-system,
					BlinkMacSystemFont,
					'Segoe UI',
					Roboto,
					Oxygen,
					Ubuntu,
					Cantarell,
					'Open Sans',
					'Helvetica Neue',
					sans-serif;
				display: flex;
				align-items: center;
				justify-content: center;
				height: 100vh;
				margin: 0;
			}

			.error {
				display: flex;
				align-items: center;
				max-width: 32rem;
				margin: 0 1rem;
			}

			.status {
				font-weight: 200;
				font-size: 3rem;
				line-height: 1;
				position: relative;
				top: -0.05rem;
			}

			.message {
				border-left: 1px solid var(--divider);
				padding: 0 0 0 1rem;
				margin: 0 0 0 1rem;
				min-height: 2.5rem;
				display: flex;
				align-items: center;
			}

			.message h1 {
				font-weight: 400;
				font-size: 1em;
				margin: 0;
			}

			@media (prefers-color-scheme: dark) {
				body {
					--bg: #222;
					--fg: #ddd;
					--divider: #666;
				}
			}
		</style>
	</head>
	<body>
		<div class="error">
			<span class="status">`+e+`</span>
			<div class="message">
				<h1>`+t+`</h1>
			</div>
		</div>
	</body>
</html>
`},version_hash:"1q7jtxz"};async function S(){return{handle:void 0,handleFetch:void 0,handleError:void 0,handleValidationError:void 0,init:void 0,reroute:void 0,transport:void 0}}export{U as a,q as b,z as c,S as g,F as o,x as p,k as r,O as s};
