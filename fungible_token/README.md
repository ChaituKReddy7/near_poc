# ERC20 Implementation on Near

<p> Simple rust based contract to implement ERC20 standard on Near protocol. </p>

<p> Install <a href="https://www.npmjs.com/package/near-cli">near-cli<a> to deploy and interact with the contract functions. </p>

1. Create a new test account on near testnet environment from <a href="https://wallet.testnet.near.org/"> here </a>.

2. From the CLI, run the following command to login with the test account.

   ```
   near login
   ```

3. Compile the contract with `wasm32-unknown-unknown` as target.

   ```
   cargo build --target wasm32-unknown-unknown --release
   ```

4. Deploy the contract to a test account

   ```
   near dev-deploy --wasmFile target/wasm32-unknown-unknown/release/erc20.wasm --initFunction new --initArgs '{"owner_id": "{local_testnet_account}", "total_supply": "10000000000000000000000000000000000"}'
   ```

5. Export the account id to a local variable

   ```
   export contract=dev-1663052824614-10654695719967
   ```

6. Tokens are now minted to the above `local_testnet_account` and to view the balance,

   ```
   near view $contract ft_balance_of '{"account_id": "{local_testnet_account}"}'
   ```

7. For an account to hold, interact with the tokens, account should first be registered with the ERC20 contract.

   ```
   near call $contract storage_deposit '{"account_id": "{local_testnet_account2}"}', "registration_only": true}' --accountId {local_testnet_account} --deposit 0.0125
   ```

8. Now the tokens can be transferred to the `local_testnet_account2` account. Note that small amount of Near has to be attached with the `transfer` call.

   ```
    near call $contract ft_transfer '{"receiver_id": "{local_testnet_account2}", "amount": "10000000000000000000000", "memo": "This is first transfer"}' --accountId {local_testnet_account} --depositYocto 1
   ```

9. To view the `total_supply` of the token,

   ```
   near view $contract ft_total_supply '{}'
   ```
