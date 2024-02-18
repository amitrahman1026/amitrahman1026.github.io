run: markdown
	trunk serve --watch ./src

build: markdown
	trunk build --release 

install:
	rustup target add wasm32-unknown-unknown
	cargo install --locked trunk

markdown:
	sh ./scripts/build_md_lists.sh

new-post:
	sh ./scripts/new_blogpost.sh

deploy:
	git push -u origin main
	git subtree push --prefix dist origin gh-pages

