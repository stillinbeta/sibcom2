TIME := $(shell date +%s)

.phony: tests debug release docker-push


debug:
	cargo build

tests:
	cargo build --verbose

release:
	cargo build --release

docker-login:
	echo $$DOCKER_PASSWORD | docker login -u stillinbeta --password-stdin

docker-build: release
	docker build -t sibcom2 .

docker-push: docker-build
	docker images
	docker tag sibcom2 stillinbeta/sibcom2:$(TIME)
	docker tag sibcom2 stillinbeta/sibcom2:latest
	docker push stillinbeta/sibcom2:$(TIME)
  docker push stillinbeta/sibcom2:latest

