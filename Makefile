SHELL=/bin/bash

include .env
export $(shell sed 's/=.*//' .env)

CUR_QUESTION_NUM := $(shell \
		 ls -l src/solutions | grep day | awk '{print $$9}' |  sed s/day//g |\
			 sort --numeric-sort --reverse | head -1)

NEXT_QUESTION_NUM := $(shell echo $$(( $(CUR_QUESTION_NUM) + 1 )) )

N = $(CUR_QUESTION_NUM)


.PHONY: download
download:
	curl -s https://adventofcode.com/2022/day/$(N)/input \
		-H "cookie: session=$$SESSION;"  \
		--output src/solutions/day$(N)/input;

.PHONY: gen
gen: N = $(NEXT_QUESTION_NUM)
gen:
	cp -R src/solutions/day1 src/solutions/day$(N);


.PHONY: new
new: N = $(NEXT_QUESTION_NUM)
new: gen download


.PHONY: run
run:
	cargo run $(N)

