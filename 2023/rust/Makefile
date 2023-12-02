.PHONY: init/%
init/%: DAY=$*
init/%:
	cp template.rs src/day$(DAY).rs
	cp bin_template.rs src/bin/day$(DAY).rs
	echo "pub mod day$(DAY);" >> src/lib.rs
	sed -i '' 's/XX/$(DAY)/' src/bin/day$(DAY).rs
