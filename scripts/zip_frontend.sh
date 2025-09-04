set -e

script_d="$(cd $(dirname ${BASH_SOURCE[0]}) && pwd)"
creds_d="${script_d}/../creds-server/frontend"

cd "$creds_d"
zip -r "dist.zip" "dist"
