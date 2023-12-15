# Changelog
This file lists all significant changes of this library.

## v0.3.0 (2023-12-15)

### Chore

 - <csr-id-024aa07a05bf578965d771031400f9deca5470dd/> update redis requirement from 0.23.3 to 0.24.0
   Updates the requirements on [redis](https://github.com/redis-rs/redis-rs) to permit the latest version.
   - [Release notes](https://github.com/redis-rs/redis-rs/releases)
   - [Commits](https://github.com/redis-rs/redis-rs/compare/redis-0.23.3...redis-0.24.0)
   
   ---
   updated-dependencies:
   - dependency-name: redis
     dependency-type: direct:production
   ...
 - <csr-id-1a66f02c93bac5839c195fdef3986654b81a6ee2/> Update dependencies
 - <csr-id-87d00a694c6cae82a73fb174f3255143a6e705f7/> bump actions/checkout from 3 to 4
   Bumps [actions/checkout](https://github.com/actions/checkout) from 3 to 4.
   - [Release notes](https://github.com/actions/checkout/releases)
   - [Changelog](https://github.com/actions/checkout/blob/main/CHANGELOG.md)
   - [Commits](https://github.com/actions/checkout/compare/v3...v4)
   
   ---
   updated-dependencies:
   - dependency-name: actions/checkout
     dependency-type: direct:production
     update-type: version-update:semver-major
   ...

### New Features

 - <csr-id-2a7bee176fa8060d974f01b6df7bde40019bddc4/> Use Duration for the queue polling timeout, not a simple number

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release over the course of 101 calendar days.
 - 248 days passed between releases.
 - 4 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Merge pull request #4 from tyrylu/dependabot/cargo/redis-0.24.0 ([`e2ffabd`](https://github.com/tyrylu/doitlater/commit/e2ffabd052d875bf5812fe787ac12963a176d62e))
    - Use Duration for the queue polling timeout, not a simple number ([`2a7bee1`](https://github.com/tyrylu/doitlater/commit/2a7bee176fa8060d974f01b6df7bde40019bddc4))
    - Update redis requirement from 0.23.3 to 0.24.0 ([`024aa07`](https://github.com/tyrylu/doitlater/commit/024aa07a05bf578965d771031400f9deca5470dd))
    - Release doitlater v0.2.7 ([`6f4dd3b`](https://github.com/tyrylu/doitlater/commit/6f4dd3b84ea17f44898b22b62326929658b264ce))
    - Merge branch 'master' of github.com:tyrylu/doitlater ([`a8f860a`](https://github.com/tyrylu/doitlater/commit/a8f860ae6f7ac25be35efd91a20441291a1e1653))
    - Update dependencies ([`1a66f02`](https://github.com/tyrylu/doitlater/commit/1a66f02c93bac5839c195fdef3986654b81a6ee2))
    - Merge pull request #3 from tyrylu/dependabot/github_actions/actions/checkout-4 ([`f64926f`](https://github.com/tyrylu/doitlater/commit/f64926f5ad58c1996a748046d5509c76f5e2e329))
    - Bump actions/checkout from 3 to 4 ([`87d00a6`](https://github.com/tyrylu/doitlater/commit/87d00a694c6cae82a73fb174f3255143a6e705f7))
</details>

## v0.2.7 (2023-10-08)

<csr-id-1a66f02c93bac5839c195fdef3986654b81a6ee2/>
<csr-id-87d00a694c6cae82a73fb174f3255143a6e705f7/>

### Chore

 - <csr-id-1a66f02c93bac5839c195fdef3986654b81a6ee2/> Update dependencies
 - <csr-id-87d00a694c6cae82a73fb174f3255143a6e705f7/> bump actions/checkout from 3 to 4
   Bumps [actions/checkout](https://github.com/actions/checkout) from 3 to 4.
   - [Release notes](https://github.com/actions/checkout/releases)
   - [Changelog](https://github.com/actions/checkout/blob/main/CHANGELOG.md)
   - [Commits](https://github.com/actions/checkout/compare/v3...v4)
   
   ---
   updated-dependencies:
   - dependency-name: actions/checkout
     dependency-type: direct:production
     update-type: version-update:semver-major
   ...

## v0.2.6 (2023-04-11)

<csr-id-ac23cb3f79c9556fb26e0ad9071e4ec246b6d44c/>

### Chore

 - <csr-id-ac23cb3f79c9556fb26e0ad9071e4ec246b6d44c/> update cron-parser requirement from 0.7.11 to 0.8.0
   Updates the requirements on [cron-parser](https://github.com/nbari/cron-parser) to permit the latest version.
   - [Release notes](https://github.com/nbari/cron-parser/releases)
   - [Commits](https://github.com/nbari/cron-parser/compare/0.7.11...0.8.0)
   
   ---
   updated-dependencies:
   - dependency-name: cron-parser
     dependency-type: direct:production
   ...

### Bug Fixes

 - <csr-id-7ffc383c621eda8e6545559ffd1cd86d8d612f2b/> Make cargo audit happy by using only needed chrono features and updated cron_parser

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 4 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release doitlater v0.2.6 ([`b8d57ca`](https://github.com/tyrylu/doitlater/commit/b8d57caaf1db4127aab170c83d25630e2139c29b))
    - Make cargo audit happy by using only needed chrono features and updated cron_parser ([`7ffc383`](https://github.com/tyrylu/doitlater/commit/7ffc383c621eda8e6545559ffd1cd86d8d612f2b))
    - Merge pull request #2 from tyrylu/dependabot/cargo/cron-parser-0.8.0 ([`44a539f`](https://github.com/tyrylu/doitlater/commit/44a539f50b1da676cfb60464ea9389c78f6e7909))
    - Update cron-parser requirement from 0.7.11 to 0.8.0 ([`ac23cb3`](https://github.com/tyrylu/doitlater/commit/ac23cb3f79c9556fb26e0ad9071e4ec246b6d44c))
</details>

## v0.2.5 (2023-04-07)

<csr-id-5b822efc841eda9208fcc054999fbd3c394c938b/>
<csr-id-7abc4e76e1ce50e712596cc026411b8525b465af/>
<csr-id-b32496f692dc95889e4f1386bd54c404aff2e675/>

### Chore

 - <csr-id-5b822efc841eda9208fcc054999fbd3c394c938b/> update redis requirement from 0.22.2 to 0.23.0
   Updates the requirements on [redis](https://github.com/redis-rs/redis-rs) to permit the latest version.
   - [Release notes](https://github.com/redis-rs/redis-rs/releases)
   - [Commits](https://github.com/redis-rs/redis-rs/compare/redis-0.22.2...redis-0.23.0)
   
   ---
   updated-dependencies:
   - dependency-name: redis
     dependency-type: direct:production
   ...
 - <csr-id-7abc4e76e1ce50e712596cc026411b8525b465af/> Rustfmt the examples after the fix
 - <csr-id-b32496f692dc95889e4f1386bd54c404aff2e675/> Run Rustfmt

### Bug Fixes

 - <csr-id-551cc4f686a7280fb2c9c3a9bc0530d27daf134d/> Fix example not to ignore a result

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 18 calendar days.
 - 19 days passed between releases.
 - 4 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release doitlater v0.2.5 ([`fc572a3`](https://github.com/tyrylu/doitlater/commit/fc572a3c97e603c9de23eb92b89ac48e491fd3fa))
    - Merge pull request #1 from tyrylu/dependabot/cargo/redis-0.23.0 ([`f9eb5d7`](https://github.com/tyrylu/doitlater/commit/f9eb5d7d718870faba3058359b264a0c50781892))
    - Update redis requirement from 0.22.2 to 0.23.0 ([`5b822ef`](https://github.com/tyrylu/doitlater/commit/5b822efc841eda9208fcc054999fbd3c394c938b))
    - Rustfmt the examples after the fix ([`7abc4e7`](https://github.com/tyrylu/doitlater/commit/7abc4e76e1ce50e712596cc026411b8525b465af))
    - Fix example not to ignore a result ([`551cc4f`](https://github.com/tyrylu/doitlater/commit/551cc4f686a7280fb2c9c3a9bc0530d27daf134d))
    - Run Rustfmt ([`b32496f`](https://github.com/tyrylu/doitlater/commit/b32496f692dc95889e4f1386bd54c404aff2e675))
</details>

## v0.2.4 (2023-03-18)

<csr-id-4f1fa99729454eb2eb22b52d0f98ba0ca1503f68/>

### Chore

 - <csr-id-4f1fa99729454eb2eb22b52d0f98ba0ca1503f68/> Run rustfmt

### Bug Fixes

 - <csr-id-a150472016ae7a9f29787ba8dca684ad83e3f90b/> Replace the library we use for cron parsing, it relied on a quite old nom, which was throwing clippy future incompatibility warnings
   Also, by this change, we get rid of nom entirely.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 59 calendar days.
 - 59 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release doitlater v0.2.4 ([`839589e`](https://github.com/tyrylu/doitlater/commit/839589e2ce659bbb4a68d9eb7da05fd266616843))
    - Replace the library we use for cron parsing, it relied on a quite old nom, which was throwing clippy future incompatibility warnings ([`a150472`](https://github.com/tyrylu/doitlater/commit/a150472016ae7a9f29787ba8dca684ad83e3f90b))
    - Run rustfmt ([`4f1fa99`](https://github.com/tyrylu/doitlater/commit/4f1fa99729454eb2eb22b52d0f98ba0ca1503f68))
    - Add dependabot config and a basic CI ([`84a659d`](https://github.com/tyrylu/doitlater/commit/84a659d82f20136ed3eb417e4b4812ee4dd897a2))
</details>

## v0.2.3 (2023-01-17)

<csr-id-50c1cb135d0d8f2a026014dce421575309499bcd/>

### Chore

 - <csr-id-50c1cb135d0d8f2a026014dce421575309499bcd/> bump dependencies

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 75 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release doitlater v0.2.3 ([`f105c58`](https://github.com/tyrylu/doitlater/commit/f105c58b6144b283c45676d9bb506fb053aa1594))
    - Bump dependencies ([`50c1cb1`](https://github.com/tyrylu/doitlater/commit/50c1cb135d0d8f2a026014dce421575309499bcd))
</details>

## v0.2.2 (2022-11-03)

<csr-id-9d6b907c3d6ee5165b94a37382de81873ef943fb/>
<csr-id-ef6fa8de6d0cbd4e66610dde2b41f47a0b76bc42/>
<csr-id-1b5ca7e5df0ed49f288f0264e52fca9d9033b643/>

### Chore

 - <csr-id-9d6b907c3d6ee5165b94a37382de81873ef943fb/> Bump dependencies
 - <csr-id-ef6fa8de6d0cbd4e66610dde2b41f47a0b76bc42/> Bump dependencies

### Other

 - <csr-id-1b5ca7e5df0ed49f288f0264e52fca9d9033b643/> Bump dependencies

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 85 calendar days.
 - 102 days passed between releases.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release doitlater v0.2.2 ([`647b5aa`](https://github.com/tyrylu/doitlater/commit/647b5aab861917acb6ba33e1b8225fe2c959ada8))
    - Bump dependencies ([`1b5ca7e`](https://github.com/tyrylu/doitlater/commit/1b5ca7e5df0ed49f288f0264e52fca9d9033b643))
    - Make clippy happy ([`68a47f0`](https://github.com/tyrylu/doitlater/commit/68a47f06bd023fe43ba70a8ad4bd82d981e5a9a1))
    - Bump dependencies ([`9d6b907`](https://github.com/tyrylu/doitlater/commit/9d6b907c3d6ee5165b94a37382de81873ef943fb))
    - Bump dependencies ([`ef6fa8d`](https://github.com/tyrylu/doitlater/commit/ef6fa8de6d0cbd4e66610dde2b41f47a0b76bc42))
</details>

## v0.2.1 (2022-07-23)

<csr-id-cfae30f74da9163baa5d015980c81eb9fb0159f5/>

### Chore

 - <csr-id-cfae30f74da9163baa5d015980c81eb9fb0159f5/> Format changelog

### New Features

 - <csr-id-c3f3922e4ac1a49375934060d39a39459f8410d7/> Re-export typetag so that the users don't have to depend on it

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 10 calendar days.
 - 10 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release doitlater v0.2.1 ([`d7ecdb9`](https://github.com/tyrylu/doitlater/commit/d7ecdb9c54bda8d59310e83ab1910c1fc19ad548))
    - Re-export typetag so that the users don't have to depend on it ([`c3f3922`](https://github.com/tyrylu/doitlater/commit/c3f3922e4ac1a49375934060d39a39459f8410d7))
    - Format changelog ([`cfae30f`](https://github.com/tyrylu/doitlater/commit/cfae30f74da9163baa5d015980c81eb9fb0159f5))
</details>

## v0.2.0 (2022-07-13)

<csr-id-5ca3ca889bd538ce4c261d0056dce1e7c1fe644a/>
<csr-id-1b3db9637b3e35df896a1a55bd07004ad83c1eb0/>

### Chore

 - <csr-id-5ca3ca889bd538ce4c261d0056dce1e7c1fe644a/> Version bump

### Chore (BREAKING)

 - <csr-id-1b3db9637b3e35df896a1a55bd07004ad83c1eb0/> Update dependencies especially type_tag

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 225 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release doitlater v0.2.0 ([`2bca0dd`](https://github.com/tyrylu/doitlater/commit/2bca0dd091c5c85e2d27bdab49cca9229e7f16ec))
    - Version bump ([`5ca3ca8`](https://github.com/tyrylu/doitlater/commit/5ca3ca889bd538ce4c261d0056dce1e7c1fe644a))
    - Update dependencies especially type_tag ([`1b3db96`](https://github.com/tyrylu/doitlater/commit/1b3db9637b3e35df896a1a55bd07004ad83c1eb0))
</details>

## v0.1.2 (2021-11-29)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 138 calendar days.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Dependency bump ([`456e90d`](https://github.com/tyrylu/doitlater/commit/456e90d355e5535acc8f646a49b89a99348b75fe))
    - Bump version and deps ([`5aa8547`](https://github.com/tyrylu/doitlater/commit/5aa854703323439f38303eb23218289d88f6474e))
    - Add missing cargo.toml keys ([`206cea8`](https://github.com/tyrylu/doitlater/commit/206cea87b76a4b6e96576054752cfc562222556f))
    - Add repository to cargo.toml ([`cfa4092`](https://github.com/tyrylu/doitlater/commit/cfa40927086e978cfb7531e6ff12c87ce33befa4))
    - Add a license ([`cd9492e`](https://github.com/tyrylu/doitlater/commit/cd9492e38ae8045fa7e9ad90bb75001b8c8c3b68))
    - Initial commit ([`a399851`](https://github.com/tyrylu/doitlater/commit/a39985128dfecd43fcfee0e5745c0a6c3be38a46))
</details>

