#! /usr/bin/env zsh

# setopt verbose

daynum=$(( $(basename day-*/ | sed -e 's/day-//' | tail -n 1) + 1))

printf -v newday -- 'day-%02d' "$daynum"

cp -R template "$newday"

sed -i '.bak' -e "s/template/$newday/" "$newday/Cargo.toml"

printf -v newdayversion -- 's/^version =.*$/version = "0.%d.0"/' "$daynum"

printf -v newtoday -- 's|^path = "day-.*$|path = "%s/src/main.rs"|' "$newday"

sed -i '.bak' -e "$newdayversion" -e "$newtoday" Cargo.toml

rm "Cargo.toml.bak" "$newday/Cargo.toml.bak"

git add "Cargo.toml" "$newday"
git commit -am "day-$daynum"
