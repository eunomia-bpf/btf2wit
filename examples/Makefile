EXAMPLES = simple

EXAMPLES_WIT = $(foreach n,$(EXAMPLES),$(n).wit)

# EXAMPLES_BINDGEN_DIR = $(foreach n,$(EXAMPLES), $(n)-output)

$(EXAMPLES_WIT):%.wit : %.c
	gcc -g -c $< -o $(basename $@).bpf.o
	pahole -J $(basename $@).bpf.o
	cargo run -- $(basename $@).bpf.o -o $@

$(EXAMPLES):% : %.wit
	rm -rf $@
	mkdir simple
	cd $@ && \
	wit-bindgen c ../$@.wit && \
	wit-bindgen rust ../$@.wit
	rm -rf *.json

clean:
	rm -rf *.json
	rm -rf *.o
	rm -rf *.wit
	for x in $(EXAMPLES); do \
		rm -rf $$x; \
	done