# Variables
MARP_FILE = rust-intro.md
ASSETS_DIR = assets
OUTPUT_DIR = dist

# CLI Command (using npx to avoid global install)
MARP_CLI = npx @marp-team/marp-cli

.PHONY: all present pdf html pptx clean help

all: pdf html

## present: Run Marp in preview mode for presentation
present:
	$(MARP_CLI) $(MARP_FILE) --preview

## pdf: Export slides to PDF
pdf:
	mkdir -p $(OUTPUT_DIR)
	$(MARP_CLI) $(MARP_FILE) --pdf --allow-local-files -o $(OUTPUT_DIR)/rust-intro.pdf

## html: Export slides to HTML
html:
	mkdir -p $(OUTPUT_DIR)
	$(MARP_CLI) $(MARP_FILE) --html --allow-local-files -o $(OUTPUT_DIR)/rust-intro.html

## pptx: Export slides to PowerPoint
pptx:
	mkdir -p $(OUTPUT_DIR)
	$(MARP_CLI) $(MARP_FILE) --pptx --allow-local-files -o $(OUTPUT_DIR)/rust-intro.pptx

## clean: Remove the dist directory
clean:
	rm -rf $(OUTPUT_DIR)

## help: Show this help message
help:
	@echo "Usage: make [target]"
	@echo ""
	@echo "Targets:"
	@grep -E '^##' Makefile | sed -e 's/## //g' | column -t -s ':'
