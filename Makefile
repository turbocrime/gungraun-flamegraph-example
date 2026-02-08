MAKEFILE_DIR := $(dir $(abspath $(lastword $(MAKEFILE_LIST))))

DOCKERIZE = docker run --rm \
  -v $(MAKEFILE_DIR):/work -w /work \
  -v gungraun-$@-cargo:/cargo \
  -e CARGO_HOME=/cargo \
  -e GUNGRAUN_HOME=$@ \
  --security-opt seccomp=unconfined \
  nixery.dev/$(if $(filter arm64,$(shell uname -m)),arm64/shell,shell)/gcc/cargo/valgrind \
  sh -c 'export PATH=$$CARGO_HOME/bin:$$PATH && $(1)'


.PHONY: help
help:
	@echo "make repro  - reproduce bugs with upstream gungraun-runner"
	@echo "make fixed  - demonstrate the fix"
	@echo "make clean  - remove generated output"

.DEFAULT_GOAL := help

repro:
	$(call DOCKERIZE,cargo install --git https://github.com/gungraun/gungraun --rev c1f4ff1 gungraun-runner && cargo bench)

fixed:
	$(call DOCKERIZE,cargo install --git https://github.com/turbocrime/gungraun --branch fix/flamegraph-multithreaded gungraun-runner && cargo bench)

.PHONY: clean
clean:
	rm -rfv ./repro ./fixed

.PHONY: clean-all
clean-all:
	rm -rfv ./repro ./fixed ./target
	docker volume rm gungraun-repro-cargo gungraun-fixed-cargo
