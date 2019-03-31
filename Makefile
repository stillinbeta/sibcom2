.phony: tests debug release docker-push


debug:
	cargo build

tests:
	cargo build --verbose

release:
	cargo build --release

docker-login:
	docker login -u stillinbeta -p "$DOCKER_PASSWORD"

docker-build: release
	docker build -t sibcom2 .

docker-push: docker-build
	docker images
	docker tag sibcom2 stillinbeta/sibcom2
	docker push stillinbeta/sibcom2

