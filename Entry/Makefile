entry.svg:entry.pdf
	inkscape --export-type svg $^
entry.pdf:entry.typ
	typst c $^
entry.png:entry.pdf
	inkscape --export-type png --export-dpi 300 $^
all:entry.svg entry.pdf entry.png
clean:
	rm -f entry.pdf entry.png
