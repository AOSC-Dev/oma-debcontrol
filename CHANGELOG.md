# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.3.0 (2023-08-18)

### Chore

 - <csr-id-83d92ed68da602d44882b5afde4319c09645b584/> set version as 0.3.0
 - <csr-id-4b8fe82586a99d37284bf1a34a85fc9dbc93e2b9/> add changelog
 - <csr-id-7166b47cc3d6e19b6e2dba123f6aa03ed24ca2ad/> rename crate to oma-debcontrol and set version as 0.1.0

### Other

 - <csr-id-e92376703b20afe1360f3612472f2597029c7475/> update to nom7

### Style

 - <csr-id-8cf98434ee0d0f08c6482a928e180d7cb939ac0a/> run cargo clippy and cargo fmt to lint code

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 155 calendar days.
 - 1241 days passed between releases.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Set version as 0.3.0 ([`83d92ed`](https://github.com/AOSC-Dev/debcontrol-rs/commit/83d92ed68da602d44882b5afde4319c09645b584))
    - Add changelog ([`4b8fe82`](https://github.com/AOSC-Dev/debcontrol-rs/commit/4b8fe82586a99d37284bf1a34a85fc9dbc93e2b9))
    - Run cargo clippy and cargo fmt to lint code ([`8cf9843`](https://github.com/AOSC-Dev/debcontrol-rs/commit/8cf98434ee0d0f08c6482a928e180d7cb939ac0a))
    - Rename crate to oma-debcontrol and set version as 0.1.0 ([`7166b47`](https://github.com/AOSC-Dev/debcontrol-rs/commit/7166b47cc3d6e19b6e2dba123f6aa03ed24ca2ad))
    - Update to nom7 ([`e923767`](https://github.com/AOSC-Dev/debcontrol-rs/commit/e92376703b20afe1360f3612472f2597029c7475))
</details>

## v0.1.1 (2020-03-24)

<csr-id-25c1e037b926bcb57071f25c093d288130ce39e2/>
<csr-id-e2195bce7a58fda3a75161b8cbfd485c68d17c4a/>
<csr-id-808b3977b8b8060596dd7d00a6861afb12ccf076/>

### Other

 - <csr-id-25c1e037b926bcb57071f25c093d288130ce39e2/> move cached directories so maybe they don't interfere with cargo publish?
 - <csr-id-e2195bce7a58fda3a75161b8cbfd485c68d17c4a/> fix panic on partial UTF-8 further in
 - <csr-id-808b3977b8b8060596dd7d00a6861afb12ccf076/> add into_inner method

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 6 calendar days.
 - 37 days passed between releases.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Ignore extra cached directories so maybe they won't interfere with cargo publish? ([`ff86b70`](https://github.com/AOSC-Dev/debcontrol-rs/commit/ff86b706c6376ca087c8b456ae3226e7d5cda382))
    - Revert "ci: move cached directories so maybe they don't interfere with cargo publish?" ([`107e0af`](https://github.com/AOSC-Dev/debcontrol-rs/commit/107e0afacd0a39c75c09c50e7519cde151a7cfb3))
    - Move cached directories so maybe they don't interfere with cargo publish? ([`25c1e03`](https://github.com/AOSC-Dev/debcontrol-rs/commit/25c1e037b926bcb57071f25c093d288130ce39e2))
    - Bump version ([`3eadefa`](https://github.com/AOSC-Dev/debcontrol-rs/commit/3eadefacde3977f8f87d4916792d5466f79ec3df))
    - Fix panic on partial UTF-8 further in ([`e2195bc`](https://github.com/AOSC-Dev/debcontrol-rs/commit/e2195bce7a58fda3a75161b8cbfd485c68d17c4a))
    - Add into_inner method ([`808b397`](https://github.com/AOSC-Dev/debcontrol-rs/commit/808b3977b8b8060596dd7d00a6861afb12ccf076))
</details>

## v0.1.0 (2020-02-16)

<csr-id-8cf98434ee0d0f08c6482a928e180d7cb939ac0a/>
<csr-id-7166b47cc3d6e19b6e2dba123f6aa03ed24ca2ad/>
<csr-id-f909c78365b5f77ad6a02c0279deb55e689b5677/>
<csr-id-bf8ddb98d471d2598ab35bbcea3ffd03063f2689/>
<csr-id-0c04a5acb4e3fd63777c1fd978a513f6785d05e8/>
<csr-id-a3f5e4ac9471db7ec56b64418735ed978b733549/>
<csr-id-e92376703b20afe1360f3612472f2597029c7475/>

### Style

 - <csr-id-8cf98434ee0d0f08c6482a928e180d7cb939ac0a/> run cargo clippy and cargo fmt to lint code

### Chore

 - <csr-id-7166b47cc3d6e19b6e2dba123f6aa03ed24ca2ad/> rename crate to oma-debcontrol and set version as 0.1.0

### Other

 - <csr-id-f909c78365b5f77ad6a02c0279deb55e689b5677/> add link to repo
 - <csr-id-bf8ddb98d471d2598ab35bbcea3ffd03063f2689/> add note about releases
 - <csr-id-0c04a5acb4e3fd63777c1fd978a513f6785d05e8/> improve UTF-8 handling
 - <csr-id-a3f5e4ac9471db7ec56b64418735ed978b733549/> add gitlab-ci
 - <csr-id-e92376703b20afe1360f3612472f2597029c7475/> update to nom7

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 37 commits contributed to the release over the course of 8 calendar days.
 - 4 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Add link to repo ([`f909c78`](https://github.com/AOSC-Dev/debcontrol-rs/commit/f909c78365b5f77ad6a02c0279deb55e689b5677))
    - Add note about releases ([`bf8ddb9`](https://github.com/AOSC-Dev/debcontrol-rs/commit/bf8ddb98d471d2598ab35bbcea3ffd03063f2689))
    - Add LICENSE ([`6202d25`](https://github.com/AOSC-Dev/debcontrol-rs/commit/6202d253aa9f3b94076a1b50ac509962616b1581))
    - Improve UTF-8 handling ([`0c04a5a`](https://github.com/AOSC-Dev/debcontrol-rs/commit/0c04a5acb4e3fd63777c1fd978a513f6785d05e8))
    - Readme formatting ([`32e407c`](https://github.com/AOSC-Dev/debcontrol-rs/commit/32e407c43fa375c9bae88e6ae56f946cb2748f33))
    - Add readme and more metadata ([`6566f6b`](https://github.com/AOSC-Dev/debcontrol-rs/commit/6566f6b569f012fcdccd7c1fd3144edfa8c9521a))
    - Add streaming example ([`9cb2899`](https://github.com/AOSC-Dev/debcontrol-rs/commit/9cb2899a757f79369dc69a889fd1d0ad79064d6c))
    - Add UTF-8 handling tests ([`e357f14`](https://github.com/AOSC-Dev/debcontrol-rs/commit/e357f14afd6fb9da2de367753764f0cc1bcbcba3))
    - Refactor and add docs ([`c7b0f67`](https://github.com/AOSC-Dev/debcontrol-rs/commit/c7b0f67de0f9fc327a524bc84ff6d8fcbdf621c2))
    - Write out streaming pull-based parser ([`9f40760`](https://github.com/AOSC-Dev/debcontrol-rs/commit/9f40760cb266a61d6f85bc51772cee303cb296bf))
    - Add some module-level docs ([`7ab7bb5`](https://github.com/AOSC-Dev/debcontrol-rs/commit/7ab7bb5d9dbef006de8e6dc331244f88b520dd20))
    - Rename parse functions ([`6302ac3`](https://github.com/AOSC-Dev/debcontrol-rs/commit/6302ac3c93d13801c56fa778a7327c3a282dbca6))
    - Switch incomplete result to Ok ([`fe29a9b`](https://github.com/AOSC-Dev/debcontrol-rs/commit/fe29a9b571ba7aea1d41513f07ce0fce8b16de70))
    - Fix build ([`9d9e27b`](https://github.com/AOSC-Dev/debcontrol-rs/commit/9d9e27b46667c1410fedcb8b75ef878519bab18d))
    - Add API docs ([`56b9eb2`](https://github.com/AOSC-Dev/debcontrol-rs/commit/56b9eb2b32d9af99438c9446e858f5d66e1e0c3a))
    - Add internal docs and tests ([`a82de45`](https://github.com/AOSC-Dev/debcontrol-rs/commit/a82de452ce06ba8bfb8aa60a6ffc2b8d74819426))
    - Solve parser duplication problem with macros ([`8be4375`](https://github.com/AOSC-Dev/debcontrol-rs/commit/8be43751cc2e1123af0f79b886ed6ec2bd63a25d))
    - WIP Refactor to new API ([`36ba5a6`](https://github.com/AOSC-Dev/debcontrol-rs/commit/36ba5a6764ea994b0656bb038df46d1c2a655937))
    - WIP Write surface tests for new API ([`6ed080f`](https://github.com/AOSC-Dev/debcontrol-rs/commit/6ed080f6caff023e08a5496f215134405f06a846))
    - WIP Unhook impl and refactor API ([`f33f008`](https://github.com/AOSC-Dev/debcontrol-rs/commit/f33f008fe6ed59e567997ff683720d3acfbe8f96))
    - Fix no_std ([`c1506ee`](https://github.com/AOSC-Dev/debcontrol-rs/commit/c1506ee99567eecabb51b79542421f11cc935dcb))
    - Fix no_std ([`f9d5c6c`](https://github.com/AOSC-Dev/debcontrol-rs/commit/f9d5c6c2049b8aea8efdb752ef360fb71b2addbe))
    - Fix clippy things ([`64300a6`](https://github.com/AOSC-Dev/debcontrol-rs/commit/64300a644de0facfbc4b484b35e2a0c6201822b1))
    - Add gitlab-ci ([`a3f5e4a`](https://github.com/AOSC-Dev/debcontrol-rs/commit/a3f5e4ac9471db7ec56b64418735ed978b733549))
    - Additional refactoring ([`98fbbf9`](https://github.com/AOSC-Dev/debcontrol-rs/commit/98fbbf99b760f45a35b53b81fcbfc4a9b60d3dd3))
    - Add example ([`c7c9133`](https://github.com/AOSC-Dev/debcontrol-rs/commit/c7c91339aac8063a07ce87ad95250f75b0c43689))
    - Add iterator-based parser ([`9171e8a`](https://github.com/AOSC-Dev/debcontrol-rs/commit/9171e8a95e22cbe0cd45dd8e769819c2ed30b533))
    - Parse an entire control file as a test ([`407cca7`](https://github.com/AOSC-Dev/debcontrol-rs/commit/407cca797ae83ede11931cfc01d802d5f6a54e33))
    - Refactor parser and fix error handling ([`932bee5`](https://github.com/AOSC-Dev/debcontrol-rs/commit/932bee5f499ac1bc14dcaa66eaff2daf4a93fbb1))
    - Implement entry point ([`0453323`](https://github.com/AOSC-Dev/debcontrol-rs/commit/045332394f723931808cd3f885c8b4de519fc5f5))
    - Implement error selection feature ([`2661ecb`](https://github.com/AOSC-Dev/debcontrol-rs/commit/2661ecb5dbf5913b2df5ae07f9bd33269afd9f0b))
    - Minor cleanups ([`7c82561`](https://github.com/AOSC-Dev/debcontrol-rs/commit/7c82561d9b619aa5865b37934701a01b89d0fa48))
    - Genericize error type ([`fe06559`](https://github.com/AOSC-Dev/debcontrol-rs/commit/fe06559db5e783e043ede9370d3d8cd8cebfede0))
    - Consume all whitespace if there's no paragraph ([`c51845b`](https://github.com/AOSC-Dev/debcontrol-rs/commit/c51845bd22daa1d00ecac3d00975ccc45d692685))
    - Parse paragraph optionally ([`e116a06`](https://github.com/AOSC-Dev/debcontrol-rs/commit/e116a06271457f7d4d4f722eeaa640f700e0a8c7))
    - Ignore Cargo.lock ([`17d65aa`](https://github.com/AOSC-Dev/debcontrol-rs/commit/17d65aaed80626c231364eff81575f889b552dc1))
    - Initial commit ([`1765294`](https://github.com/AOSC-Dev/debcontrol-rs/commit/17652940e354ead58769f395ad7419457e55484d))
</details>

