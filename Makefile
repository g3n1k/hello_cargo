.PHONY: buildpre build run

buildpre:
	docker build -f Dockerfile.pre -t g3n1k/wfm-be-pre .
	# docker build --build-arg SQLX_OFFLINE=true -f Dockerfile.pre -t g3n1k/wfm-be-pre .

build:
	docker build --build-arg SQLX_OFFLINE=true -f Dockerfile -t g3n1k/wfm-be .

run:
	docker run -p 8001:8080 -e DATABASE_URL=postgres://user:user@192.168.1.111/wfm g3n1k/wfm-be
