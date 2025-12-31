
// this file is generated — do not edit it


declare module "svelte/elements" {
	export interface HTMLAttributes<T> {
		'data-sveltekit-keepfocus'?: true | '' | 'off' | undefined | null;
		'data-sveltekit-noscroll'?: true | '' | 'off' | undefined | null;
		'data-sveltekit-preload-code'?:
			| true
			| ''
			| 'eager'
			| 'viewport'
			| 'hover'
			| 'tap'
			| 'off'
			| undefined
			| null;
		'data-sveltekit-preload-data'?: true | '' | 'hover' | 'tap' | 'off' | undefined | null;
		'data-sveltekit-reload'?: true | '' | 'off' | undefined | null;
		'data-sveltekit-replacestate'?: true | '' | 'off' | undefined | null;
	}
}

export {};


declare module "$app/types" {
	export interface AppTypes {
		RouteId(): "/" | "/decrypt" | "/encrypt" | "/keys" | "/settings";
		RouteParams(): {
			
		};
		LayoutParams(): {
			"/": Record<string, never>;
			"/decrypt": Record<string, never>;
			"/encrypt": Record<string, never>;
			"/keys": Record<string, never>;
			"/settings": Record<string, never>
		};
		Pathname(): "/" | "/decrypt" | "/decrypt/" | "/encrypt" | "/encrypt/" | "/keys" | "/keys/" | "/settings" | "/settings/";
		ResolvedPathname(): `${"" | `/${string}`}${ReturnType<AppTypes['Pathname']>}`;
		Asset(): "/icons/128x128.png" | "/icons/128x128@2x.png" | "/icons/32x32.png" | "/icons/Square107x107Logo.png" | "/icons/Square142x142Logo.png" | "/icons/Square150x150Logo.png" | "/icons/Square284x284Logo.png" | "/icons/Square30x30Logo.png" | "/icons/Square310x310Logo.png" | "/icons/Square44x44Logo.png" | "/icons/Square71x71Logo.png" | "/icons/Square89x89Logo.png" | "/icons/StoreLogo.png" | "/icons/icon.png" | string & {};
	}
}