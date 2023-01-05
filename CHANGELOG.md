# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.3.5 (2023-01-05)

<csr-id-c073406a43f93aa2b55da285867283a9ad28bcae/>

### Other

 - <csr-id-c073406a43f93aa2b55da285867283a9ad28bcae/> don't request ad token for version command

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 2 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - don't request ad token for version command ([`c073406`](https://github.com/jvanbuel/pattrick/commit/c073406a43f93aa2b55da285867283a9ad28bcae))
</details>

## 0.3.4 (2023-01-03)

### Bug Fixes

 - <csr-id-3519844591a06c19ac9bf89aad648d8e6979ce88/> add tokio_test blocking thread for async doc examples
 - <csr-id-f533d43867a8446ed5a48ec756791e59f03bee31/> remove trailing and leading quotes of scopes

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 1 calendar day.
 - 1 day passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - add tokio_test blocking thread for async doc examples ([`3519844`](https://github.com/jvanbuel/pattrick/commit/3519844591a06c19ac9bf89aad648d8e6979ce88))
    - remove trailing and leading quotes of scopes ([`f533d43`](https://github.com/jvanbuel/pattrick/commit/f533d43867a8446ed5a48ec756791e59f03bee31))
    - remove rustdoc missing doc code feature ([`4e41d62`](https://github.com/jvanbuel/pattrick/commit/4e41d623b41c22389fcf6c921b03b3c8f29e494e))
</details>

## 0.3.3 (2023-01-02)

<csr-id-8eb0b74ed838649f33f793012d20afe4d9ea9cf5/>

### New Features

 - <csr-id-74e3abd79cf3031f17fce0679bea6711f8dc9ee1/> use version of the implementation crate instead of clap crate

### Other

 - <csr-id-8eb0b74ed838649f33f793012d20afe4d9ea9cf5/> activate tar feature flags self_update

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 day passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - use version of the implementation crate instead of clap crate ([`74e3abd`](https://github.com/jvanbuel/pattrick/commit/74e3abd79cf3031f17fce0679bea6711f8dc9ee1))
    - activate tar feature flags self_update ([`8eb0b74`](https://github.com/jvanbuel/pattrick/commit/8eb0b74ed838649f33f793012d20afe4d9ea9cf5))
</details>

## 0.3.2 (2022-12-31)

### Bug Fixes

- Change output destination of fig autocomplete generation script to target directory

## 0.3.1 (2022-12-31)

### New Features

- add documentation and small refactorings to improve UX

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - add documentation and small refactorings to improve UX ([`6be2336`](https://github.com/jvanbuel/pattrick/commit/6be23367f377a2a7f4b07bfc04773895ff242a35))
</details>

## 0.3.0 (2022-12-30)

### New Features

 - <csr-id-b202c5ab3445cd9b73b253e3e1915e51a33b9c07/> use enums for token scope
 - <csr-id-641778af1a55c4552a00dc27ec49a7188a509c02/> possibility of removal by name instead of id
- support multiple scopes for PAT creation
- add logging
- add azure auto-login if no credentials are found

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 13 commits contributed to the release over the course of 2 calendar days.
 - 64 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - support multiple scopes for PAT creation ([`1db6804`](https://github.com/jvanbuel/pattrick/commit/1db6804a5b6259c553a17bf18142271b476b14e8))
    - rename github.rs to release.rs ([`bbbdc86`](https://github.com/jvanbuel/pattrick/commit/bbbdc86793631d60d51abacf0d3d7e9d23017a2b))
    - add tests for custom deserializer ([`3132980`](https://github.com/jvanbuel/pattrick/commit/3132980b8227c5d424073b95a820e47bc97f4f11))
    - add logging ([`13d2d91`](https://github.com/jvanbuel/pattrick/commit/13d2d91df5b061b2924595729fe3deeb2ba02d46))
    - refactor crud ([`c805e18`](https://github.com/jvanbuel/pattrick/commit/c805e1880184e901efede1a9b9d7705bc17bbb0c))
    - custom deserialization for scopes ([`9559315`](https://github.com/jvanbuel/pattrick/commit/955931562233ce9055546da2f799fac1c1cdd13a))
    - refactor ([`f32690e`](https://github.com/jvanbuel/pattrick/commit/f32690e59c380015c3374a41df099d216cadcdd7))
    - use enums for token scope ([`b202c5a`](https://github.com/jvanbuel/pattrick/commit/b202c5ab3445cd9b73b253e3e1915e51a33b9c07))
    - possibility of removal by name instead of id ([`641778a`](https://github.com/jvanbuel/pattrick/commit/641778af1a55c4552a00dc27ec49a7188a509c02))
    - implement paging for list operation ([`5c5fba1`](https://github.com/jvanbuel/pattrick/commit/5c5fba1a37c99d3710048a215afc230b1173780e))
    - remove emoji dependency ([`89a3095`](https://github.com/jvanbuel/pattrick/commit/89a309599fa99daad75aff2da07a15ac235f35c1))
    - add azure auto-login if now credentials are found ([`aa40f7a`](https://github.com/jvanbuel/pattrick/commit/aa40f7a61b21b76b756b3d34f79503011bf1c157))
    - save progress ([`dea5ce2`](https://github.com/jvanbuel/pattrick/commit/dea5ce23c62cdd896ccd8379b437e7b9a31a5196))
</details>

## v0.2.0 (2022-10-27)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 16 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - add multiple targets in makefile ([`2547137`](https://github.com/jvanbuel/pattrick/commit/2547137fca770387cd6c5f63cf52ab2472cf4a50))
    - split clap cli into different crate for fig spec generation ([`949f90b`](https://github.com/jvanbuel/pattrick/commit/949f90ba77014dde215cda1d90a283e73aa25c85))
    - fix bug with write offset ([`f97a746`](https://github.com/jvanbuel/pattrick/commit/f97a746cd1233b0e28118528ab5722af6cd932a2))
    - create .netrc if it does not exist ([`69da6d3`](https://github.com/jvanbuel/pattrick/commit/69da6d3dc917b5717d01a508a654f00deba056fa))
</details>

## v0.1.0 (2022-10-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 12 commits contributed to the release over the course of 6 calendar days.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - split between lib and cli ([`adceb87`](https://github.com/jvanbuel/pattrick/commit/adceb8755a7249ad2a4528709bf2190ffafaae0d))
    - support for .netrc file ([`ef9805b`](https://github.com/jvanbuel/pattrick/commit/ef9805b3c0bc2987e5328a148ad1c7a8b73f1915))
    - support for .env file ([`0c26794`](https://github.com/jvanbuel/pattrick/commit/0c26794c36dd58822c01f7cdfac7f1dca41033f9))
    - use chrono for timestamp ([`4fd6eef`](https://github.com/jvanbuel/pattrick/commit/4fd6eefcf05b2553c4da194f4aa4b5fc778778f3))
    - basic functionality done ([`61d0fb0`](https://github.com/jvanbuel/pattrick/commit/61d0fb079df80f0946ac92462977c967bfa301c0))
    - experiment with tabled ([`4291de0`](https://github.com/jvanbuel/pattrick/commit/4291de0531ad8631a7b7d3f8e1b78abaf32274f1))
    - progress on list ([`c8cdb95`](https://github.com/jvanbuel/pattrick/commit/c8cdb95866aeca8dcbc9eee75f6338ad355345d9))
    - progress ([`fcf43d5`](https://github.com/jvanbuel/pattrick/commit/fcf43d5e48413e86749cd26da1ee8c24db06c8a1))
    - refactor ([`d552012`](https://github.com/jvanbuel/pattrick/commit/d55201208f99be5d4291cfeeb1355bed519c888e))
    - expand clap commands ([`308b365`](https://github.com/jvanbuel/pattrick/commit/308b365a7a65d0c7b6b60ec999f4e92bb0bed742))
    - add clap config ([`9b30edd`](https://github.com/jvanbuel/pattrick/commit/9b30edd16b5462796b104ea268f6f91b4b909a3c))
    - initial commit ([`020da8c`](https://github.com/jvanbuel/pattrick/commit/020da8c903a221c7d27886684a6c147485f9716a))
</details>

