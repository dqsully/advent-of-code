#!/bin/bash -e


CURRENT_YEAR="$(TZ=America/New_York date +%Y)"
CURRENT_MONTH="$(TZ=America/New_York date +%m)"
CURRENT_DAY="$(TZ=America/New_York date +%d)"

ensure_date() {
  local YEAR="$1"
  local DAY="$2"
  local MESSAGE="$3"
  shift 2

  if [[ "$CURRENT_YEAR" -lt "$YEAR" ]]; then
    if [[ -n "$MESSAGE" ]]; then
      echo "$MESSAGE: it's not $YEAR in New York yet"
    fi

    return 1
  elif [[ "$CURRENT_YEAR" -gt "$YEAR" ]]; then
    return 0
  fi

  if [[ "$CURRENT_MONTH" -lt 12 ]]; then
    if [[ ! -z "$MESSAGE" ]]; then
      echo "$MESSAGE: it's not December in New York yet"
    fi

    return 1
  elif [[ "$CURRENT_MONTH" -gt 12 ]]; then
    return 0
  fi

  if [[ "${CURRENT_DAY#0}" -lt "${DAY#0}" ]]; then
    if [[ ! -z "$MESSAGE" ]]; then
      echo "$MESSAGE: it's not Dec $DAY in New York yet"
    fi

    return 1
  fi
}

download_problem() {
  local YEAR="$1"
  local DAY="$2"
  shift 2

  ensure_date "$YEAR" "$DAY" "Unable to download problem"

  if [[ ! -s "session.txt" ]]; then
    cat <<EOF
Please log into Advent of Code first (https://adventofcode.com/), open your
browser's dev tools, go to the "Storage" tab in Firefox or the "Application" tab
in Chrome/Edge/etc., copy the 'session' cookie's value, and paste it into the
'session.txt' file.
EOF
    exit 1
  fi

  if ! curl -f "https://adventofcode.com/$YEAR/day/${DAY#0}/input" -H "Cookie: session=$(cat "session.txt")" -o "$YEAR/$DAY/input.txt"; then
    cat <<EOF
Failed to download problem input for year $YEAR day $DAY, see error above. Most
likely the problem isn't open yet or your session token expired. Please
double-check your system's time and your session token.
EOF
    exit 1
  fi
}

prepare_day() {
  local YEAR="$1"
  local DAY="$2"
  shift

  mkdir -p "$YEAR/$DAY"

  if [[ ! -e "$YEAR/$DAY/input.txt" ]] && ensure_date "$YEAR" "$DAY" "Skipping downloading problem"; then
    download_problem "$YEAR" "$DAY"
  fi
}

run_day() {
  local YEAR="$1"
  local DAY="$2"
  shift 2

  local LANGS
  local LANG

  if [[ ! -d "$YEAR/$DAY" ]]; then
    return
  fi

  if [[ ! -e "$YEAR/$DAY/input.txt" ]]; then
    download_problem "$YEAR" "$DAY"
  fi

  if [[ "$#" -eq 0 ]]; then
    LANGS=(rust go ts js)
  else
    LANGS=("${@[@]}")
  fi

  for LANG in "${LANGS[@]}"; do
    if [[ -d "$YEAR/$DAY/$LANG" ]]; then
      pushd "$YEAR/$DAY/$LANG" >/dev/null

      case "$LANG" in
        rust)
          echo "$YEAR/12/$DAY - Rust"
          cargo run --release -q
          echo
          ;;

        go)
          echo "$YEAR/12/$DAY - Go"
          go run .
          echo
          ;;

        ts )
          echo "$YEAR/12/$DAY - TypeScript"
          npm run start
          echo
          ;;

        js )
          echo "$YEAR/12/$DAY - JavaScript"
          npm run start
          echo
          ;;

        *)
          echo "Unknown language '$LANG'"
          echo
          return 1
          ;;
      esac

      popd >/dev/null
    fi
  done
}

test_day() {
  local YEAR="$1"
  local DAY="$2"
  shift 2

  local LANGS
  local LANG

  if [[ ! -d "$YEAR/$DAY" ]]; then
    return
  fi

  if [[ "$#" -eq 0 ]]; then
    LANGS=(rust go ts js)
  else
    LANGS=("${@[@]}")
  fi

  for LANG in "${LANGS[@]}"; do
    if [[ -d "$YEAR/$DAY/$LANG" ]]; then
      pushd "$YEAR/$DAY/$LANG" >/dev/null

      case "$LANG" in
        rust)
          echo "$YEAR/12/$DAY - Rust"
          cargo test -q
          echo
          ;;

        go)
          echo "$YEAR/12/$DAY - Go"
          go test ./...
          echo
          ;;

        ts )
          echo "$YEAR/12/$DAY - TypeScript"
          npm run test
          echo
          ;;

        js )
          echo "$YEAR/12/$DAY - JavaScript"
          npm run test
          echo
          ;;

        *)
          echo "Unknown language '$LANG'"
          return 1
          ;;
      esac

      popd >/dev/null
    fi
  done
}

for_each_day() {
  local YEAR="$1"
  shift

  local DAYS
  local DAY

  if [[ "$CURRENT_YEAR" -lt "$YEAR" ]]; then
    echo "It's not $YEAR in New York yet"
    return 1
  elif [[ "$CURRENT_YEAR" -gt "$YEAR" ]]; then
    # shellcheck disable=SC2207
    DAYS=($(seq -f "%02g" 1 25))
  else
    if [[ "$CURRENT_MONTH" -lt 12 ]]; then
      echo "It's not Decempter in New York yet"
      return 1
    elif [[ "$CURRENT_MONTH" -gt 12 ]]; then
      # shellcheck disable=SC2207
      DAYS=($(seq -f "%02g" 1 25))
    else
      if [[ "$CURRENT_DAY" -lt 25 ]]; then
        # shellcheck disable=SC2207
        DAYS=($(seq -f "%02g" 1 "$CURRENT_DAY"))
      else
        # shellcheck disable=SC2207
        DAYS=($(seq -f "%02g" 1 25))
      fi
    fi
  fi

  local COMMAND="$1"
  shift

  for DAY in "${DAYS[@]}"; do
    "$COMMAND" "$YEAR" "$DAY" "$@"
  done
}

for_each_year() {
  local YEAR

  local COMMAND="$1"
  shift

  for YEAR in $(seq 2015 "$CURRENT_YEAR"); do
    "$COMMAND" "$YEAR" "$@"
  done
}

with_day() {
  local YEAR="$1"
  local COMMAND="$2"
  local DAY="$CURRENT_DAY"
  shift 2

  if [[ "$#" -gt 0 ]]; then
    DAY="$1"
    shift
  fi

  if [[ "$DAY" == "all" ]]; then
    for_each_day "$YEAR" "$COMMAND" "$@"
  else
    "$COMMAND" "$YEAR" "$(printf "%02d" "${DAY#0}")" "$@"
  fi
}

prepare() {
  YEAR="$1"
  shift

  with_day "$YEAR" prepare_day "$@"
}

run() {
  YEAR="$1"
  shift

  with_day "$YEAR" run_day "$@"
}

test() {
  YEAR="$1"
  shift

  with_day "$YEAR" test_day "$@"
}

with_year() {
  local COMMAND="$1"
  local YEAR="$CURRENT_YEAR"
  shift

  if [[ "$#" -gt 0 ]]; then
    YEAR="$1"
    shift
  fi

  if [[ "$YEAR" == "all" ]]; then
    for_each_year "$COMMAND" "$@"
  else
    "$COMMAND" "$YEAR" "$@"
  fi
}

help() {
  cat <<EOF
Usage:

./advent.sh prepare [year] [day]
./advent.sh run [year] [day] [langs...]
./advent.sh test [year] [day] [langs...]
EOF

  return 1
}

if [[ "$#" -eq 0 ]]; then
  help
fi

COMMAND="$1"
shift

case "$COMMAND" in
  prepare)
    with_year prepare "$@"
    ;;

  run)
    with_year run "$@"
    ;;

  test)
    with_year test "$@"
    ;;

  *)
    help
    ;;
esac
