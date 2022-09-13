# Status Message

<p> Simple rust based contract to set a status message for user and option to update it. </p>

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
   near dev-deploy --wasmFile target/wasm32-unknown-unknown/release/status_message.wasm
   ```

5. Export the account id to a local variable

   ```
   export contract=dev-1663052824614-10654695719967
   ```

6. To create a new message for your local account, we have to interact with `create` method in the contract.

   ```
   near call $contract create '{"message": "Hello"}' --accountId {local_testnet_account}
   ```

7. To check the stored message for an account,

   ```
   near view $contract get '{"account_id": "{local_testnet_account}"}'
   ```

8. To update the stored message,

   ```
   near call $contract update '{"message": "New message"}' --accountId {local_testnet_account}"}'
   ```
