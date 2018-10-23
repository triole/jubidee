APP_NAME=$(shell cat Cargo.toml | grep -Po "(?<=name\s=\s\")[a-zA-Z0-9-_]+")
TARGET_FOLDER=build
TARGET_BUILD=${TARGET_FOLDER}/${APP_NAME}
COMMIT_NO=$(shell git rev-list --all --count)

ARGS_TPL="src/args.tpl"
ARGS_YAML="src/args.yaml"

all: run_test run_build
build: run_build
test: run_test


run_build:
	# replace version in args.yaml
	cat "${ARGS_TPL}" | sed '/version/s/\.X\"/\.${COMMIT_NO}\"/g' > ${ARGS_YAML}

	# build binary
	cargo build --release
	mkdir -p ${TARGET_FOLDER}
	mv "target/release/${APP_NAME}" "${TARGET_BUILD}"
	strip "${TARGET_BUILD}"
	cp config/${APP_NAME}.yaml ${TARGET_FOLDER}/
	hyperfine "${TARGET_BUILD}"

run_test:
	cargo test
