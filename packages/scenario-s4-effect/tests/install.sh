#!/usr/bin/env bash
# Run from the repository root. One Cargo invocation; no server dispatch or real service.
D=packages/scenario-s4-effect
P=38143
L=$(mktemp -d) || exit 1
S=
cleanup() {
  if [ -n "$S" ]; then
    kill "$S" 2>/dev/null || true
    wait "$S" 2>/dev/null; echo "serve exit=$? (intentional shutdown)"; tail -n 1 "$L/serve.log"
  fi
}
trap cleanup EXIT
step() {
  local name=$1
  shift
  local status=0
  "$@" > "$L/$name.log" 2>&1 || status=$?
  echo "$name exit=$status"
  tail -n 1 "$L/$name.log"
  if [ "$status" -ne 0 ]; then
    tail -n 20 "$L/$name.log"
    tail -n 20 "$L/serve.log" 2>/dev/null || true
    exit "$status"
  fi
}
build() { (cd engine && cargo build --release); }
step build build
E=$PWD/engine/target/release/conduit
H=$(cat "$D/dist/scenario-s4-effect.tar.gz.sha256") || exit 1
mkdir "$L/project" || exit 1
cp "$D"/tests/fixtures/project/*.yaml "$L/project/" || exit 1
sed -i.orig "s/sha256:PLACEHOLDER/sha256:$H/" "$L/project/conduit.yaml" || exit 1
rm "$L/project/conduit.yaml.orig" || exit 1
if (exec 3<>/dev/tcp/127.0.0.1/$P) 2>/dev/null; then
  echo "port $P already answers; stopped before init"
  exit 1
fi
step init "$E" init --store "$L/store" --url "http://127.0.0.1:$P/mcp"
"$E" serve --store "$L/store" > "$L/serve.log" 2>&1 &
S=$!
ready=false
for i in $(seq 30); do
  kill -0 "$S" 2>/dev/null || break
  if (exec 3<>/dev/tcp/127.0.0.1/$P) 2>/dev/null; then ready=true; break; fi
  sleep 1
done
if ! kill -0 "$S" 2>/dev/null || [ "$ready" != true ]; then
  echo "serve did not remain alive and become ready on port $P"
  tail -n 20 "$L/serve.log"
  exit 1
fi
step install "$E" install --store "$L/store" "$PWD/$D/dist/scenario-s4-effect.tar.gz#$H"
step submit "$E" submit --store "$L/store" "$L/project"
echo "installation passed on loopback port $P; submit registered templates only"
