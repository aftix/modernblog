#!/bin/sh

[ -f "/app/data.db" ] || cp ./data.db /app/.
[ -f "/app/Rocket.toml" ] || cp Rocket.toml /app/.

cd /app
aftback
