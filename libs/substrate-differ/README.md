# Substrate Differ

## Overview

This crate helps comparing two runtimes by diffing their metadata. Historycally, a few approaches have
been tested. The newest `reduced differ` superseded all other approaches. This is now the only differ
that remains.

## Context

The Substrate metadata format is versioned. The current version is `V14`.
Whether the metadata change or not is critical. Adding new modules or calls has usually a low impact. However, removing, modifying or reoredring modules and/or their content is critical to the various client APIs.

## Diff methods

Historically, this crate offered 3 methods to compare metadata. You can search Github for the older doc :)
The newer `reduced differ` is much better in all aspects so it has been defined as the only one provided in `subwasm`.

## Reduced differ

Here we are diffing **the metadata**.

The `reduced_differ` works in 3 steps:
- reducing each runtimes
- diffing the reduced runtimes
- analizing the differences and reporting the results

The first step is to convert the runtime metadata into a `ReducedRuntime`.
The `ReducedRuntime` as its name suggest has bee reduced and does not contain all the information available in the metadata.

`subwasm` and its `show` command allow showing a `ReducedRuntime` either as text or json.
Here is an extract of how it looks like (the output has been trucated as it is pretty lenghty):

```
$ subwasm show --chain polkadot
```

```
Pallet #71: Slots
  calls:
    -  0: force_lease ( para: ParaId, leaser: T::AccountId, amount: BalanceOf<T>, period_begin: LeasePeriodOf<T>, period_count: LeasePeriodOf<T>, )  )
    -  1: clear_all_leases ( para: ParaId, )  )
    -  2: trigger_onboard ( para: ParaId, )  )
  events:
    -  0: NewLeasePeriod ( lease_period: LeasePeriodOf<T>, )  )
    -  1: Leased ( para_id: ParaId, leaser: T::AccountId, period_begin: LeasePeriodOf<T>, period_count: LeasePeriodOf<T>, extra_reserved: BalanceOf<T>, total_amount: BalanceOf<T>, )  )
  errors:
    -  0: ParaNotOnboarding
    -  1: LeaseError
  constants:
    - LeaseOffset: [0, 16, 14, 0]
    - LeasePeriod: [0, 117, 18, 0]
  storages:
    - Default  Leases: [0]

Pallet #10: Preimage
  calls:
    -  0: note_preimage ( bytes: Vec<u8>, )  )
    -  1: unnote_preimage ( hash: T::Hash, )  )
    -  2: request_preimage ( hash: T::Hash, )  )
    -  3: unrequest_preimage ( hash: T::Hash, )  )
  events:
    -  0: Noted ( hash: T::Hash, )  )
    -  1: Requested ( hash: T::Hash, )  )
    -  2: Cleared ( hash: T::Hash, )  )
  errors:
    -  0: TooBig
    -  1: AlreadyNoted
    -  2: NotAuthorized
    -  3: NotNoted
    -  4: Requested
    -  5: NotRequested
  storages:
    - Optional PreimageFor: [0]
    - Optional StatusFor: [0]

...
```

Using the `--summary` flag show a more compact view:
```
                            NAME   ID       CALLS   EVENTS   ERRORS CONSTANTS  STORAGE
            ---------------------------------------------------------------------------
                          System - 0            8        6        6        6       16
                       Scheduler - 1            6        6        5        2        3
                            Babe - 2            3        0        4        3       16
                       Timestamp - 3            1        0        0        1        2
                         Indices - 4            5        3        5        1        1
                        Balances - 5            6       10        8        3        5
                      Authorship - 6            1        0        7        1        3
                         Staking - 7           26       15       25        7       37
                        Offences - 8            0        1        0        0        3
                         Session - 9            2        1        5        0        7
[...]
```

The `ReducedRuntime` format is easier to manage and it means we only need to convert new Runtime Metadata versions to a
`ReducedRuntime` to have it supported by `subwasm`.

Once two runtimes have been reduced, we can diff them and here is an extract of the output (check out the `subwasm`
documentation for the latest renders):

```
$ subwasm diff runtime_a.wasm runtime_b.wasm
```

```
!!! THE SUBWASM REDUCED DIFFER IS EXPERIMENTAL, DOUBLE CHECK THE RESULTS !!!
[≠] pallet 0: System -> 3 change(s)
  - constants changes:
    [≠] BlockWeights: [ 7, 112, 46, 48, 218, 1, 0, 11, 0, 32, 74, 169, 209, 1, 19, 255, 255, 255, 255, 255, 255, 255, 255, 66, 23, 161, 22, 0, 1, 11, 48, 138, ... ]
    [≠] Version: [ 32, 112, 111, 108, 107, 97, 100, 111, 116, 60, 112, 97, 114, 105, 116, 121, 45, 112, 111, 108, 107, 97, 100, 111, 116, 0, 0, 0, 0, 154, 36, 0, ... ]

  - storages changes:
    [≠] Default  Account: [0; 80]

[≠] pallet 2: Babe -> 1 change(s)
  - storages changes:
    [+] StorageDesc { name: "SkippedEpochs", modifier: "Default", default_value: [0] }

[≠] pallet 5: Balances -> 25 change(s)
  - calls changes:
    [≠]  0: transfer ( dest: AccountIdLookupOf<T>, value: T::Balance, )  )
    [≠]  1: set_balance ( who: AccountIdLookupOf<T>, new_free: T::Balance, new_reserved: T::Balance, )  )
    [+] CallDesc { index: 6, name: "upgrade_accounts", signature: SignatureDesc { args: [ArgDesc { name: "who", ty: "Vec<T::AccountId>" }] } }
[...]
[≠] pallet 6: Authorship -> 11 change(s)
  - calls changes:
    [-] "set_uncles"

  - errors changes:
    [-] "InvalidUncleParent"
    [-] "UnclesAlreadySet"
    [-] "TooManyUncles"
    [-] "GenesisUncle"
[...]
```

The analisis of the diff is shown at the bottom:
```
SUMMARY:
- Compatible.......................: false
- Require transaction_version bump.: false
```

## Diff legend

- `[+]`: This item is new
- `[≠]`: This item has changed
- `[-]`: This item has been removed

NOTE: Items are identified by their `id`s. For instance,
If a call `id` changes from 7 to 42, it will be shown as removed (id: 7) **and** added (id: 42).
