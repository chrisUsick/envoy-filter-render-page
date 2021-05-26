build:
	docker run --rm --user "$(id -u)":"$(id -g)" -v "${PWD}":/usr/src/app -w /usr/src/app envoy-filter-render-page-build:latest cargo +nightly build --target=wasm32-unknown-unknown --release 

build-image: 
	docker build -t envoy-filter-render-page-build -f Dockerfile.wasm .
