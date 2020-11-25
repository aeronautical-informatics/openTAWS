rm -r screenshots
termtosvg screenshots --still-frames --template terminal_app --command 'cargo test cucumber'

for f in $(find screenshots -name '*.svg')
do
	echo $f
	rsvg-convert -f pdf -o ${f%%.svg}.pdf $f
done
