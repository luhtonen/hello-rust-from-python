SUBDIRS := "hello-marco" "calc" "trust"

all: $(SUBDIRS)
$(SUBDIRS):
		$(MAKE) -C $@ $(MAKECMDGOALS)

.PHONY: all $(SUBDIRS)

format: $(SUBDIRS)

lint: $(SUBDIRS)

test: $(SUBDIRS)

release: $(SUBDIRS)
