This is minimal template for swap based Solana BPF programs.

Hangs as: 
```
 data.len: 0 owner: 11111111111111111111111111111111 executable: false rent_epoch: 0 } } }, KeyedAccount { is_signer: true, is_writable: true, key: GXeCHtrorkfT1DSz7SmCsafjqJEEgymiJRGZXAdhZ1Kn, account: RefCell { value: Account { lamports: 0 data.len: 0 owner: 11111111111111111111111111111111 executable: false rent_epoch: 0 } } }]
[2021-04-06T16:33:56.561221300Z DEBUG solana_runtime::message_processor] Program 11111111111111111111111111111111 success
[2021-04-06T16:33:56.561773800Z DEBUG solana_runtime::message_processor] Program log: Instruction: Init
[2021-04-06T16:33:56.562316300Z DEBUG solana_runtime::message_processor] Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [1]
[2021-04-06T16:33:56.562819600Z DEBUG solana_runtime::message_processor] Program log: Instruction: MintTo
test uniswap_like_setup ... test uniswap_like_setup has been running for over 60 seconds
test uniswap_like_setup ... FAILED

failures:

---- uniswap_like_setup stdout ----
thread 'solana-bank-forks-client' panicked at 'Account data resizing not supported yet: 82 -> 0. Consider making this test conditional on `#[cfg(feature = "test-bpf")]`', C:\Users\dz\.cargo\registry\src\github.com-1ecc6299db9ec823\solana-program-test-1.6.4\src\lib.rs:316:25
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'uniswap_like_setup' panicked at 'called `Result::unwrap()` on an `Err` value: IoError(Custom { kind: TimedOut, error: "Client dropped expired request." })', tests\tests.rs:231:57


failures:
    uniswap_like_setup

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 60.81s
```

### Build and test for program compiled natively
```
$ cargo build
```

### Build and test the program compiled for BPF
```
$ cargo build-bpf
$ cargo test-bpf
```