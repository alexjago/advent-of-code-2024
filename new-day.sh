#! /usr/bin/env zsh

# setopt verbose

daynum=$(( $(find day-* -type d -d 0 | sed -e 's/day-//' | sort | tail -n 1) + 1))

printf -v newday -- 'day-%02d' "$daynum"

cp -R template "$newday"

sed -i '.bak' -e "s/template/$newday/" "$newday/Cargo.toml"

printf -v newdayversion -- 's/^version =.*$/version = "0.%d.0"/' "$daynum"

printf -v newtoday -- 's|^path = "day-.*$|path = "%s/src/main.rs"|' "$newday"

sed -i '.bak' -e "$newdayversion" -e "$newtoday" Cargo.toml

rm "Cargo.toml.bak" "$newday/Cargo.toml.bak"

git add "Cargo.toml" "$newday"
git commit -am "day-$daynum"

curl "https://adventofcode.com/2024/day/$daynum/input" --header "Cookie: session=$(cat .token)" > "$newday/input.txt"

open -a 'Safari' "https://adventofcode.com/2024/day/$daynum"
