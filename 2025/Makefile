.PHONY: benchmark/01

benchmark/01:
	@echo "Building Zig binary..."
	@cd zig && mise exec -- zig build day01 --release=fast
	@echo "Running benchmarks..."
	hyperfine --warmup 3 \
		'cd python && python day01.py' \
		'cd typescript && DAY=01 npm run day' \
		'cd zig && ./zig-out/bin/day01'
