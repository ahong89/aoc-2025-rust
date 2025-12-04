BUILD=build
DAY?=$(day)

run: $(BUILD)/day$(day)
	./$<

$(BUILD)/day%: day%.rs
	rustc $< --out-dir $(BUILD)

day%.rs:
	cp template.rs $@

clean:
	rm -rf $(BUILD)/*
