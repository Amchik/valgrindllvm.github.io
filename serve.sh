#!/bin/bash

if [[ " $@ " == *"autoreload"* ]]; then
    export SERVE_AUTORELOAD=1
fi

if ! [[ "$_SERVE_STATUS" = "run" ]]; then
	export _SERVE_STATUS=run
	port=$1
	[[ "$port" = "" ]] && port=8080
	echo "Running server on http://localhost:$port/"
	while true; do
		nc -l -p $port -e "$0" || exit 0
	done

	echo "Please fix you'r /bin/true"
	exit 1
fi

read ctx
path="$(echo "$ctx" | sed 's/GET \/\([^ \?\#]*\)[^ ]* HTTP\/1\.1/\1/' | sed 's/\r//g')"

status="200 OK"
if [[ "${SERVE_AUTORELOAD}" = "1" ]]; then
    make all > /dev/null 2>&1 || status="500 INTERNAL SERVER ERROR"
fi

[[ "$path" = "" ]] && path=index.html
#path="out/$path"
path="$(echo "$path" | sed 's/\.\(txt\)$/.html/')"
[[ "${path##*.}" = "$path" ]] && path="$path.html"
path="out/$path"

if [[ "$status" == "200 OK" ]] && (! test -e "$path" || [[ "$path" = "out/404.html" ]]); then
	status="404 NOT FOUND"
	path="out/404.html"
fi

len=$(wc -c "$path")

case "$path" in
    *.jpg)
        type="image/jpeg"
        cc="max-age=604800"
        ;;
	*.json)
		type="application/json"
		;;
	*.html)
		type="text/html"
		;;
	*.css)
		type="text/css"
		;;
	*)
		type="text/plain"
		;;
esac

echo "HTTP/1.1 $status"
echo "Content-Type: $type"
echo "Content-Length: ${len%% *}"
[[ "$cc" = "" ]] || echo "Cache-Control: $cc"
echo
cat "$path" 2>/dev/null

exit 0

