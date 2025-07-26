.PHONY: watch serve

watch:
	watchexec -w "src" -- wasm-pack build

serve:
	npm run serve
