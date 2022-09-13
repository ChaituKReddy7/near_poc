# ERC721 Implementation on Near

<p> Simple rust based contract to implement ERC721 standard on Near protocol. </p>

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
   near dev-deploy --wasmFile target/wasm32-unknown-unknown/release/NFT.wasm --initFunction new_default_meta --initArgs '{"owner_id": "{local_testnet_account}"}'
   ```

5. Export the account id to a local variable

   ```
   export contract=dev-1663052824614-10654695719967
   ```

6. To mint new NFT to an account,

   ```
   near call $contract nft_mint '{"receiver_id": "{local_testnet_account}", "token_metadata": {
       "title": "This is first NFT",
       "description": "This is first description",
       "media": "https://cdn.pixabay.com/photo/2022/09/01/09/31/sunset-glow-7425170_1280.jpg",
       "copies": 100
   }}' --accountId {local_testnet_account} --deposit 0.0125
   ```

7. To transfer the NFT to an account,

   ```
   near call $contract nft_transfer '{"receiver_id": "{local_testnet_account2}", "token_id": "1"}' --accountId {local_testnet_account} --depositYocto 1
   ```

8. To view the NFTs held by an account,

   ```
   near view $contract nft_tokens_for_owner '{"account_id": "{local_testnet_account}"}'
   ```

9. To view the total supply for the ERC721,

   ```
   near view $contract nft_total_supply '{}'
   ```

10. To view all NFTs,

    ```
    near view $contract nft_tokens '{}'
    ```

This call may fail due to gas limit, pass `from_index` and `limit` to limit the list.
