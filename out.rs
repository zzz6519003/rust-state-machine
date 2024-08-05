#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
mod balances {
    use num::traits::{CheckedAdd, CheckedSub, Zero};
    use std::collections::BTreeMap;
    /// The configuration trait for the Balances Module.
    /// Contains the basic types needed for handling balances.
    pub trait Config: crate::system::Config {
        /// A type which can represent the balance of an account.
        /// Usually this is a large unsigned integer.
        type Balance: Zero + CheckedSub + CheckedAdd + Copy;
    }
    /// This is the Balances Module.
    /// It is a simple module which keeps track of how much balance each account has in this state
    /// machine.
    pub struct Pallet<T: Config> {
        balances: BTreeMap<T::AccountId, T::Balance>,
    }
    #[automatically_derived]
    impl<T: ::core::fmt::Debug + Config> ::core::fmt::Debug for Pallet<T>
    where
        T::AccountId: ::core::fmt::Debug,
        T::Balance: ::core::fmt::Debug,
    {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "Pallet",
                "balances",
                &&self.balances,
            )
        }
    }
    impl<T: Config> Pallet<T> {
        pub fn new() -> Self {
            Self { balances: BTreeMap::new() }
        }
        /// Set the balance of an account `who` to some `amount`.
        pub fn set_balance(&mut self, who: &T::AccountId, amount: T::Balance) {
            self.balances.insert(who.clone(), amount);
        }
        /// Get the balance of an account `who`.
        /// If the account has no stored balance, we return zero.
        pub fn balance(&self, who: &T::AccountId) -> T::Balance {
            *self.balances.get(who).unwrap_or(&T::Balance::zero())
        }
    }
    impl<T: Config> Pallet<T> {
        /// Transfer `amount` from one account to another.
        /// This function verifies that `from` has at least `amount` balance to transfer,
        /// and that no mathematical overflows occur.
        pub fn transfer(
            &mut self,
            caller: T::AccountId,
            to: T::AccountId,
            amount: T::Balance,
        ) -> crate::support::DispatchResult {
            let caller_balance = self.balance(&caller);
            let to_balance = self.balance(&to);
            let new_caller_balance = caller_balance
                .checked_sub(&amount)
                .ok_or("Not enough funds.")?;
            let new_to_balance = to_balance.checked_add(&amount).ok_or("Overflow")?;
            self.balances.insert(caller, new_caller_balance);
            self.balances.insert(to, new_to_balance);
            Ok(())
        }
    }
    #[allow(non_camel_case_types)]
    pub enum Call<T: Config> {
        transfer { to: T::AccountId, amount: T::Balance },
    }
    impl<T: Config> crate::support::Dispatch for Pallet<T> {
        type Caller = T::AccountId;
        type Call = Call<T>;
        fn dispatch(
            &mut self,
            caller: Self::Caller,
            call: Self::Call,
        ) -> crate::support::DispatchResult {
            match call {
                Call::transfer { to, amount } => {
                    self.transfer(caller, to, amount)?;
                }
            }
            Ok(())
        }
    }
}
mod system {
    use core::ops::AddAssign;
    use num::traits::{One, Zero};
    use std::collections::BTreeMap;
    pub trait Config {
        type AccountId: Ord + Clone;
        type BlockNumber: Zero + One + AddAssign + Copy;
        type Nonce: Zero + One + Copy;
    }
    /// This is the System Pallet.
    /// It handles low level state needed for your blockchain.
    pub struct Pallet<T: Config> {
        block_number: T::BlockNumber,
        nonce: BTreeMap<T::AccountId, T::Nonce>,
    }
    #[automatically_derived]
    impl<T: ::core::fmt::Debug + Config> ::core::fmt::Debug for Pallet<T>
    where
        T::BlockNumber: ::core::fmt::Debug,
        T::AccountId: ::core::fmt::Debug,
        T::Nonce: ::core::fmt::Debug,
    {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "Pallet",
                "block_number",
                &self.block_number,
                "nonce",
                &&self.nonce,
            )
        }
    }
    impl<T: Config> Pallet<T> {
        /// Create a new instance of the System Pallet.
        pub fn new() -> Self {
            Self {
                block_number: T::BlockNumber::zero(),
                nonce: BTreeMap::new(),
            }
        }
        pub fn block_number(&self) -> T::BlockNumber {
            self.block_number
        }
        pub fn inc_block_number(&mut self) {
            self.block_number += T::BlockNumber::one();
        }
        pub fn inc_nonce(&mut self, who: &T::AccountId) {
            let nonce: T::Nonce = *self.nonce.get(who).unwrap_or(&T::Nonce::zero());
            let new_nonce = nonce + T::Nonce::one();
            self.nonce.insert(who.clone(), new_nonce);
        }
    }
}
mod support {
    /// The most primitive representation of a Blockchain block.
    pub struct Block<Header, Extrinsic> {
        /// The block header contains metadata about the block.
        pub header: Header,
        /// The extrinsics represent the state transitions to be executed in this block.
        pub extrinsics: Vec<Extrinsic>,
    }
    /// We are using an extremely simplified header which only contains the current block number.
    /// On a real blockchain, you would expect to also find:
    /// - parent block hash
    /// - state root
    /// - extrinsics root
    /// - etc...
    pub struct Header<BlockNumber> {
        pub block_number: BlockNumber,
    }
    /// This is an "extrinsic": literally an external message from outside of the blockchain.
    /// This simplified version of an extrinsic tells us who is making the call, and which call they are
    /// making.
    pub struct Extrinsic<Caller, Call> {
        pub caller: Caller,
        pub call: Call,
    }
    /// The Result type for our runtime. When everything completes successfully, we return `Ok(())`,
    /// otherwise return a static error message.
    pub type DispatchResult = Result<(), &'static str>;
    /// A trait which allows us to dispatch an incoming extrinsic to the appropriate state transition
    /// function call.
    pub trait Dispatch {
        /// The type used to identify the caller of the function.
        type Caller;
        /// The state transition function call the caller is trying to access.
        type Call;
        /// This function takes a `caller` and the `call` they want to make, and returns a `Result`
        /// based on the outcome of that function call.
        fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> DispatchResult;
    }
}
mod proof_of_existence {
    use crate::support::DispatchResult;
    use core::fmt::Debug;
    use std::collections::BTreeMap;
    pub trait Config: crate::system::Config {
        /// The type which represents the content that can be claimed using this pallet.
        /// Could be the content directly as bytes, or better yet the hash of that content.
        /// We leave that decision to the runtime developer.
        type Content: Debug + Ord;
    }
    /// This is the Proof of Existence Module.
    /// It is a simple module that allows accounts to claim existence of some data.
    pub struct Pallet<T: Config> {
        /// A simple storage map from content to the owner of that content.
        /// Accounts can make multiple different claims, but each claim can only have one owner.
        claims: BTreeMap<T::Content, T::AccountId>,
    }
    #[automatically_derived]
    impl<T: ::core::fmt::Debug + Config> ::core::fmt::Debug for Pallet<T>
    where
        T::Content: ::core::fmt::Debug,
        T::AccountId: ::core::fmt::Debug,
    {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "Pallet",
                "claims",
                &&self.claims,
            )
        }
    }
    impl<T: Config> Pallet<T> {
        /// Create a new instance of the Proof of Existence Module.
        pub fn new() -> Self {
            Self { claims: BTreeMap::new() }
        }
        /// Get the owner (if any) of a claim.
        pub fn get_claim(&self, claim: &T::Content) -> Option<&T::AccountId> {
            self.claims.get(&claim)
        }
    }
    impl<T: Config> Pallet<T> {
        /// Create a new claim on behalf of the `caller`.
        /// This function will return an error if someone already has claimed that content.
        pub fn create_claim(
            &mut self,
            caller: T::AccountId,
            claim: T::Content,
        ) -> DispatchResult {
            if self.claims.contains_key(&claim) {
                return Err(&"this content is already claimed");
            }
            self.claims.insert(claim, caller);
            Ok(())
        }
        /// Revoke an existing claim on some content.
        /// This function should only succeed if the caller is the owner of an existing claim.
        /// It will return an error if the claim does not exist, or if the caller is not the owner.
        pub fn revoke_claim(
            &mut self,
            caller: T::AccountId,
            claim: T::Content,
        ) -> DispatchResult {
            let owner = self.get_claim(&claim).ok_or("claim does not exist")?;
            if caller != *owner {
                return Err(&"this content is owned by someone else");
            }
            self.claims.remove(&claim);
            Ok(())
        }
    }
    #[allow(non_camel_case_types)]
    pub enum Call<T: Config> {
        create_claim { claim: T::Content },
        revoke_claim { claim: T::Content },
    }
    impl<T: Config> crate::support::Dispatch for Pallet<T> {
        type Caller = T::AccountId;
        type Call = Call<T>;
        fn dispatch(
            &mut self,
            caller: Self::Caller,
            call: Self::Call,
        ) -> crate::support::DispatchResult {
            match call {
                Call::create_claim { claim } => {
                    self.create_claim(caller, claim)?;
                }
                Call::revoke_claim { claim } => {
                    self.revoke_claim(caller, claim)?;
                }
            }
            Ok(())
        }
    }
}
use crate::support::Dispatch;
mod types {
    pub type AccountId = String;
    pub type Balance = u128;
    pub type BlockNumber = u32;
    pub type Nonce = u32;
    pub type Extrinsic = crate::support::Extrinsic<AccountId, crate::RuntimeCall>;
    pub type Header = crate::support::Header<BlockNumber>;
    pub type Block = crate::support::Block<Header, Extrinsic>;
    pub type Content = &'static str;
}
pub enum RuntimeCall {
    Balances(balances::Call<Runtime>),
    ProofOfExistence(proof_of_existence::Call<Runtime>),
}
pub struct Runtime {
    system: system::Pallet<Self>,
    balances: balances::Pallet<Self>,
    proof_of_existence: proof_of_existence::Pallet<Self>,
}
#[automatically_derived]
impl ::core::fmt::Debug for Runtime {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "Runtime",
            "system",
            &self.system,
            "balances",
            &self.balances,
            "proof_of_existence",
            &&self.proof_of_existence,
        )
    }
}
impl system::Config for Runtime {
    type AccountId = types::AccountId;
    type BlockNumber = types::BlockNumber;
    type Nonce = types::Nonce;
}
impl balances::Config for Runtime {
    type Balance = types::Balance;
}
impl proof_of_existence::Config for Runtime {
    type Content = types::Content;
}
impl Runtime {
    fn new() -> Self {
        Self {
            system: system::Pallet::new(),
            balances: balances::Pallet::new(),
            proof_of_existence: proof_of_existence::Pallet::new(),
        }
    }
    fn execute_block(&mut self, block: types::Block) -> support::DispatchResult {
        self.system.inc_block_number();
        if block.header.block_number != self.system.block_number() {
            return Err(&"block number does not match what is expected");
        }
        for (i, support::Extrinsic { caller, call }) in block
            .extrinsics
            .into_iter()
            .enumerate()
        {
            self.system.inc_nonce(&caller);
            let _res = self
                .dispatch(caller, call)
                .map_err(|e| {
                    {
                        ::std::io::_eprint(
                            format_args!(
                                "Extrinsic Error\n\tBlock Number: {0}\n\tExtrinsic Number: {1}\n\tError: {2}\n",
                                block.header.block_number,
                                i,
                                e,
                            ),
                        );
                    }
                });
        }
        Ok(())
    }
}
impl crate::support::Dispatch for Runtime {
    type Caller = <Runtime as system::Config>::AccountId;
    type Call = RuntimeCall;
    fn dispatch(
        &mut self,
        caller: Self::Caller,
        runtime_call: Self::Call,
    ) -> support::DispatchResult {
        match runtime_call {
            RuntimeCall::Balances(call) => {
                self.balances.dispatch(caller, call)?;
            }
            RuntimeCall::ProofOfExistence(call) => {
                self.proof_of_existence.dispatch(caller, call)?;
            }
        }
        Ok(())
    }
}
fn main() {
    let mut runtime = Runtime::new();
    let alice = "alice".to_string();
    let bob = "bob".to_string();
    let charlie = "charlie".to_string();
    runtime.balances.set_balance(&alice, 100);
    let block_1 = types::Block {
        header: support::Header { block_number: 1 },
        extrinsics: <[_]>::into_vec(
            #[rustc_box]
            ::alloc::boxed::Box::new([
                support::Extrinsic {
                    caller: alice.clone(),
                    call: RuntimeCall::Balances(balances::Call::transfer {
                        to: bob.clone(),
                        amount: 30,
                    }),
                },
                support::Extrinsic {
                    caller: alice.clone(),
                    call: RuntimeCall::Balances(balances::Call::transfer {
                        to: charlie,
                        amount: 20,
                    }),
                },
            ]),
        ),
    };
    let block_2 = types::Block {
        header: support::Header { block_number: 2 },
        extrinsics: <[_]>::into_vec(
            #[rustc_box]
            ::alloc::boxed::Box::new([
                support::Extrinsic {
                    caller: alice.clone(),
                    call: RuntimeCall::ProofOfExistence(proof_of_existence::Call::create_claim {
                        claim: &"Hello, world!",
                    }),
                },
                support::Extrinsic {
                    caller: bob.clone(),
                    call: RuntimeCall::ProofOfExistence(proof_of_existence::Call::create_claim {
                        claim: &"Hello, world!",
                    }),
                },
            ]),
        ),
    };
    let block_3 = types::Block {
        header: support::Header { block_number: 3 },
        extrinsics: <[_]>::into_vec(
            #[rustc_box]
            ::alloc::boxed::Box::new([
                support::Extrinsic {
                    caller: alice,
                    call: RuntimeCall::ProofOfExistence(proof_of_existence::Call::revoke_claim {
                        claim: &"Hello, world!",
                    }),
                },
                support::Extrinsic {
                    caller: bob,
                    call: RuntimeCall::ProofOfExistence(proof_of_existence::Call::create_claim {
                        claim: &"Hello, world!",
                    }),
                },
            ]),
        ),
    };
    runtime.execute_block(block_1).expect("invalid block");
    runtime.execute_block(block_2).expect("invalid block");
    runtime.execute_block(block_3).expect("invalid block");
    {
        ::std::io::_print(format_args!("{0:#?}\n", runtime));
    };
}
