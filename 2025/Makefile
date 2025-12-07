.PHONY: benchmark/01

benchmark/01:
	@echo "Building binaries..."
	@cd zig && mise exec -- zig build day01 --release=fast > /dev/null 2>&1
	@cd typescript && npm run build > /dev/null 2>&1
	@cd gleam && mise exec -- gleam build -t erlang > /dev/null 2>&1
	@cd gleam && mise exec -- gleam build -t javascript > /dev/null 2>&1
	@echo "import { main } from './build/dev/javascript/advent_of_code/day01.mjs'; main();" > gleam/run_day01.mjs
	@echo "Running benchmarks..."
	@hyperfine --warmup 3 \
		--command-name "Gleam (Erlang)" 'cd gleam && mise exec -- erl -pa build/dev/erlang/*/ebin -eval "advent_of_code@@main:run(day01)" -noshell' \
		--command-name "Gleam (JavaScript)" 'cd gleam && node run_day01.mjs' \
		--command-name "Python" 'cd python && python day01.py' \
		--command-name "TypeScript" 'cd typescript && node dist/day01.js' \
		--command-name "Zig" 'cd zig && ./zig-out/bin/day01'
