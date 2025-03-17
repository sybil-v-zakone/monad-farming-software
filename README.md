# Testnet Monad Retroactive soft

## **Installing Rust**

### **For MacOS and Linux**

1. Open the terminal.
2. Install Rust using the following command:

    ```
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

3. After the installation is complete, update the environment variables:

    ```
    source $HOME/.cargo/env
    ```

4. Verify the installation:

    ```
    rustc --version
    ```

    If you see the Rust version (e.g., `rustc 1.65.0`), the installation was successful.

### **For Windows**

1. Download the Rust installer from the official website: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).
2. Run the installer and follow the on-screen instructions.
3. After installation, open the command prompt and verify the installation:

    ```
    rustc --version
    ```

If you see the Rust version, the installation was successful.

---

## **Soft Setup**

### **1. Fill in Private Keys and Proxies**

1. Navigate to the `data` folder in the root directory of the project.
2. Open the `private_keys.txt` file.
3. Add your private keys to this file, one key per line. Example:

    ```
    0xYourPrivateKey1
    0xYourPrivateKey2
    0xYourPrivateKey3
    ```

4. Open the `proxies.txt` file.
5. Add your proxies to this file, one proxy per line like private keys earlier.

### **2. Configure the Settings**

1. In the `data` folder, open the `config.toml` file.
2. Fill in the settings according to your requirements.

---

## **Running the Code**

1. Ensure Rust is installed.
2. Download or clone the repository with the code.
3. Navigate to the project directory:

    ```
    cd /path/to/your/project
    ```

4. Run the project:

    ```
    cargo build --release
    ```

    The application will start running using the settings from the configuration file.

---

# **Configuration Settings Documentation**

This document outlines the configuration settings used in the application. Each setting is defined as a range `[x, y]`, from which a random value is selected at runtime. These settings control various aspects of the application, including swap counts, deposit counts, NFT minting, delays, and transaction ratios.

## **Swap Settings**

These settings determine the number of swaps that will be performed for each swap type. A random value is selected from the specified range.

- **`ambient_swap_count`** : The number of swaps to perform on the Ambient protocol.
- **`hashflow_swap_count`** : The number of swaps to perform on the Hashflow protocol.
- **`bean_swap_count`** : The number of swaps to perform on the Bean protocol.

## **Deposit Settings**

These settings define the number of deposits to be made for each deposit type. A random value is selected from the specified range.

- **`apriori_deposit_count`** : The number of deposits to make on the Apriori protocol.
- **`kinza_deposit_count`** : The number of deposits to make on the Kinza protocol.
- **`shmonad_deposit_count`** : The number of deposits to make on the Shmonad protocol.

## **NFT Settings**

This setting determines the number of NFTs to mint. A random value is selected from the specified range.

- **`nad_domains_count`** : The number of NAD domains (NFTs) to mint.

## **Bridge settings**

- **`need_bridge`** : Set `true` If you want bridge some MON tokens from Base to Monad using GasZip before performing warmup, or use default `false`.
- **`bridge_amount_range`** : Range of eth amount of ETH that will be bridging using GasZip from Base ETH token to Monad MON token.

## **Miscellaneous Settings**

These settings control delays, ratios, and other runtime behaviors.

- **`thread_delay`** : The base delay value (in seconds) for staggering the processing of accounts.
- **`restart_thread_delay`** : The delay before restarting a thread that has encountered an error.
- **`action_delay`** : The random delay (in seconds) between consecutive actions on a single account. This helps mimic realistic user behavior.
- **`deposit_ratio`** : The percentage of the account's balance to use during a deposit action. A random percentage within this range is selected for each deposit.
- **`swap_ratio`** : The percentage of the account's balance to use during a swap action. A random percentage within this range is selected for each swap.

## **RPC URL**

- **`monad_rpc_url`** : The RPC endpoint used for interacting with the Monad blockchain.
- **`base_rpc_url`** : The RPC endpoint used for interacting with the Base blockchain.

---

## **Example**

For example, if `ambient_swap_count` is set to `[1, 3]`, the application will randomly select a value between 1 and 3 (inclusive) for the number of Ambient swaps to perform. Similarly, if `action_delay` is set to `[5, 10]`, the application will wait for a random duration between 5 and 10 seconds between consecutive actions on a single account.
