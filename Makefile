SUBDIRS := "hello-marco" "calc" "trust"

all: $(SUBDIRS)
$(SUBDIRS):
		$(MAKE) -C $@ $(MAKECMDGOALS)

.PHONY: all $(SUBDIRS)

format: $(SUBDIRS)

format-check: $(SUBDIRS)

lint: $(SUBDIRS)

test: $(SUBDIRS)

build-release: $(SUBDIRS)
