# Changelog
This file lists all significant changes of this library.

## v0.2.4 (2023-03-18)

### Chore

 - <csr-id-4f1fa99729454eb2eb22b52d0f98ba0ca1503f68/> Run rustfmt

### Bug Fixes

 - <csr-id-a150472016ae7a9f29787ba8dca684ad83e3f90b/> Replace the library we use for cron parsing, it relied on a quite old nom, which was throwing clippy future incompatibility warnings
   Also, by this change, we get rid of nom entirely.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 59 calendar days.
 - 59 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
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

