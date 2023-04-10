Setup
  $ . ${TESTDIR}/../_helpers/setup.sh
  $ . ${TESTDIR}/setup.sh $(pwd)

  $ ${TURBO} prune --scope=web --docker
  Generating pruned monorepo for web in .*out (re)
   - Added shared
   - Added util
   - Added web
Make sure patches are part of the json output
  $ ls out/json
  apps
  package.json
  packages
  patches
  pnpm-workspace.yaml

Make sure the pnpm patches section is present
  $ cat out/json/package.json | jq '.pnpm.patchedDependencies'
  {
    "is-number@7.0.0": "patches/is-number@7.0.0.patch"
  }