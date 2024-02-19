@Library('jenkins-library@feature/dops-2942-update_rust_lib') _

def pipeline = new org.rust.AppPipeline(steps: this,
      initSubmodules: true,
      envImageName: 'docker.soramitsu.co.jp/sora2/env:env',
      appImageName: 'docker.soramitsu.co.jp/sora2/liberland',
      disableCodeCoverage: true,
      pushTags: ['feature/dops-2837': 'dev'],
      buildTestCmds: ['housekeeping/build.sh'],
      secretScannerExclusion: 'specs/mainnet.json|.*Cargo.toml',
      deepSecretScannerExclusion: ['specs/mainnet.json'],
      buildArtifacts: "target/release/wbuild/kitchensink-runtime/kitchensink_runtime.compact.compressed.wasm"
)
pipeline.runPipeline()