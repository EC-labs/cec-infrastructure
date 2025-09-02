set -e

script_d="$(cd $(dirname ${BASH_SOURCE[0]}) && pwd)"
creds_d="${script_d}/../creds/groups"

cd "$creds_d"

while read -r file; do
    zip -r "${file}.zip" "${file}"
done < <(find . -maxdepth 1 -mindepth 1 -type d)
