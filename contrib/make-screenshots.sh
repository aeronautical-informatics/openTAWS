cd $(dirname "$0")

rm -r screenshots
termtosvg screenshots --still-frames --template screenshot-colorscheme.svg --command 'cargo test cucumber'

for f in $(find screenshots -name '*.svg')
do
	echo $f
	rsvg-convert -f pdf -o ${f%%.svg}.pdf $f &
done
