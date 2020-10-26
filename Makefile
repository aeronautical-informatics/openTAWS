OCI_ENGINE     ?= podman
IMAGE_TAG       = alpine-ruby-cucumber
OCI_RUN_CMD     = $(OCI_ENGINE) run --tty --interactive $(OCI_LOG_DRIVER) \
		  --volume .:/app --workdir /app $(IMAGE_TAG) 
OCI_LOG_DRIVER ?= --log-driver=none

cargo: cargo/test

cargo/test: 
	@cargo test

cargo/build:
	@cargo build

oci/image:
	@$(OCI_ENGINE) build --tag $(IMAGE_TAG) contrib/docker-alpine-ruby-cucumber > /dev/null

report: report/cli

report/cli: oci/image
	@$(OCI_RUN_CMD) 

report/html: oci/image
	@mkdir -p report
	@$(OCI_RUN_CMD) --format html > report/index.html

clean:
	@rm -rf report target
