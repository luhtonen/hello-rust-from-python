SUBDIRS := "hello-marco" "calc" "trust" "sqlite-hf"

all: $(SUBDIRS)
$(SUBDIRS):
		$(MAKE) -C $@ $(MAKECMDGOALS)

.PHONY: all $(SUBDIRS)

format: $(SUBDIRS)

lint: $(SUBDIRS)

test: $(SUBDIRS)

release: $(SUBDIRS)
