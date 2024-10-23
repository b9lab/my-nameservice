# My Name Service

This project is built as a companion project of the CosmWasm tutorials. Its object is to show various features of CosmWasm, along with the progression of the code as elements and features are added to demonstrate how a smart contract can be built with code and libraries.

The progression of the code is demonstrated via the help of branches and diffs.

## Progressive feature branches

The project is created with a clean list of commits in order to demonstrate the natural progression of the project. In this sense, there is no late commit that fixes an error introduced earlier. If there is a fix for an error introduced earlier, the fix should be squashed with the earlier commit that introduced the error. This may require some conflict resolution.

Having a clean list of commits makes it possible to do meaningful `compare`s.

To make it easier to link to the content at the different stages of the project's progression, a number of branches have been created at commits that have `Add branch the-branch-name.` as message. Be careful with the commit message as it depends on it matching the `"Add branch [0-9a-zA-Z\-]*\."` regular expression.

The script `push-branches.sh` is used to extract these commits and force push them to the appropriate branch in the repository. After having made changes, you should run this script, and also manually force push to `main`.

Versions used here are:

* Rust: 1.80.1 edition 2021

Branches:

* [`initial-cargo`](../../tree/initial-cargo)
* [`instantiation-message`](../../tree/instantiation-message), [diff](../../compare/initial-cargo..instantiation-message)
* [`instantiation-function`](../../tree/instantiation-function), [diff](../../compare/instantiation-message..instantiation-function)
* [`improve-error-reporting`](../../tree/improve-error-reporting), [diff](../../compare/instantiation-function..improve-error-reporting)
* [`compilation-elements`](../../tree/compilation-elements), [diff](../../compare/improve-error-reporting..compilation-elements)
* [`first-unit-test`](../../tree/first-unit-test), [diff](../../compare/compilation-elements..first-unit-test)
* [`first-execute-message`](../../tree/first-execute-message), [diff](../../compare/first-unit-test..first-execute-message)
* [`first-query-message`](../../tree/first-query-message), [diff](../../compare/first-execute-message..first-query-message)
* [`first-multi-test`](../../tree/first-multi-test), [diff](../../compare/first-query-message..first-multi-test)
* [`first-event`](../../tree/first-event), [diff](../../compare/first-multi-test..first-event)
* [`add-first-library`](../../tree/add-first-library), [diff](../../compare/first-event..add-first-library)
* [`add-nft-library`](../../tree/add-nft-library), [diff](../../compare/add-first-library..add-nft-library)
* [`execute-return-data`](../../tree/execute-return-data), [diff](../../compare/add-nft-library..execute-return-data)
