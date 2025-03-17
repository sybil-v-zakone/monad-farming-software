# **Установка Rust**

## **Installing Rust**

## **安装 Rust**

### **Для Linux/macOS**

### **For Linux/macOS**

### **对于 Linux/macOS**

1. Откройте терминал.
   Open the terminal.
   打开终端。
2. Установите Rust с помощью команды:
   Install Rust using the following command:
   使用以下命令安装 Rust：

   ```
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

3. После завершения установки, обновите переменные окружения:
   After the installation is complete, update the environment variables:
   安装完成后，更新环境变量：

   ```
   source $HOME/.cargo/env
   ```

4. Проверьте установку:
   Verify the installation:
   验证安装：

   ```
   rustc --version
   ```

   Если вы видите версию Rust (например, `rustc 1.65.0`), установка прошла успешно.
   If you see the Rust version (e.g., `rustc 1.65.0`), the installation was successful.
   如果您看到 Rust 版本（例如 `rustc 1.65.0`），则安装成功。

---

### **Для Windows**

### **For Windows**

### **对于 Windows**

1. Скачайте установщик Rust с официального сайта: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).
   Download the Rust installer from the official website: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).
   从官方网站下载 Rust 安装程序：[https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)。
2. Запустите установщик и следуйте инструкциям на экране.
   Run the installer and follow the on-screen instructions.
   运行安装程序并按照屏幕上的说明进行操作。
3. После установки откройте командную строку и проверьте установку:
   After installation, open the command prompt and verify the installation:
   安装完成后，打开命令提示符并验证安装：

   ```
   rustc --version
   ```

   Если вы видите версию Rust, установка прошла успешно.
   If you see the Rust version, the installation was successful.
   如果您看到 Rust 版本，则安装成功。

---

## **Настройка проекта**

## **Project Setup**

## **项目设置**

### **1. Заполните приватные ключи**

### **1. Fill in Private Keys**

### **1. 填写私钥**

1. Перейдите в папку `data` в корневой директории проекта.
   Navigate to the `data` folder in the root directory of the project.
   进入项目根目录中的 `data` 文件夹。
2. Откройте файл `private_keys.txt`.
   Open the `private_keys.txt` file.
   打开 `private_keys.txt` 文件。
3. Добавьте свои приватные ключи в этот файл, каждый ключ с новой строки.
   Add your private keys to this file, one key per line.
   将您的私钥添加到该文件中，每行一个私钥。
   Пример:
   Example:
   示例：

   ```
   0xYourPrivateKey1
   0xYourPrivateKey2
   0xYourPrivateKey3
   ```

---

### **2. Настройте конфигурацию**

### **2. Configure the Settings**

### **2. 配置设置**

1. В папке `data` откройте файл `config.toml`.
   In the `data` folder, open the `config.toml` file.
   在 `data` 文件夹中，打开 `config.toml` 文件。
2. Заполните настройки в соответствии с вашими требованиями.
   Fill in the settings according to your requirements.
   根据您的需求填写设置

---

## **Запуск кода**

## **Running the Code**

## **运行代码**

1. Убедитесь, что Rust установлен.
   Ensure Rust is installed.
   确保已安装 Rust。
2. Скачайте или клонируйте репозиторий с кодом.
   Download or clone the repository with the code.
   下载或克隆包含代码的仓库。
3. Перейдите в директорию проекта:
   Navigate to the project directory:
   进入项目目录：

   ```
   cd /path/to/your/project
   ```

4. Соберите проект с помощью Cargo:
   Build the project using Cargo:
   使用 Cargo 构建项目：

   ```
   cargo build
   ```

5. Запустите проект:
   Run the project:
   运行项目：

   ```
   cargo run
   ```

   Приложение начнет выполнение с использованием настроек из конфигурационного файла.
   The application will start running using the settings from the configuration file.
   应用程序将开始运行，并使用配置文件中的设置。

---

## **Документация по настройкам конфигурации**

# **Configuration Settings Documentation**

# **配置设置文档**

Этот документ описывает настройки конфигурации, используемые в приложении. Каждая настройка определена как диапазон `[x, y]`, из которого случайное значение выбирается во время выполнения. Эти настройки управляют различными аспектами приложения, включая количество свопов, депозитов, минт NFT, задержки и соотношения транзакций.

This document outlines the configuration settings used in the application. Each setting is defined as a range `[x, y]`, from which a random value is selected at runtime. These settings control various aspects of the application, including swap counts, deposit counts, NFT minting, delays, and transaction ratios.

本文档概述了应用程序中使用的配置设置。每个设置都定义为一个范围 `[x, y]`，在运行时从中选择一个随机值。这些设置控制应用程序的各个方面，包括交换次数、存款次数、NFT 铸造、延迟和交易比例。

---

## **Настройки свопов**

## **Swap Settings**

## **交换设置**

Эти настройки определяют количество свопов, которые будут выполнены для каждого типа. Случайное значение выбирается из указанного диапазона.

These settings determine the number of swaps that will be performed for each swap type. A random value is selected from the specified range.

这些设置决定了每种交换类型将执行的交换次数。从指定范围中选择一个随机值。

- **`ambient_swap_count`** : Диапазон `[0, 0]`
  Количество свопов в протоколе Ambient.
  The number of swaps to perform on the Ambient protocol.
  在 Ambient 协议上执行的交换次数。
- **`hashflow_swap_count`** : Диапазон `[0, 0]`
  Количество свопов в протоколе Hashflow.
  The number of swaps to perform on the Hashflow protocol.
  在 Hashflow 协议上执行的交换次数。
- **`bean_swap_count`** : Диапазон `[0, 0]`
  Количество свопов в протоколе Bean.
  The number of swaps to perform on the Bean protocol.
  在 Bean 协议上执行的交换次数。

---

## **Настройки депозитов**

## **Deposit Settings**

## **存款设置**

Эти настройки определяют количество депозитов для каждого типа. Случайное значение выбирается из указанного диапазона.

These settings define the number of deposits to be made for each deposit type. A random value is selected from the specified range.

这些设置定义了每种存款类型的存款次数。从指定范围中选择一个随机值。

- **`apriori_deposit_count`** : Диапазон `[0, 0]`
  Количество депозитов в протоколе Apriori.
  The number of deposits to make on the Apriori protocol.
  在 Apriori 协议上进行的存款次数。
- **`kinza_deposit_count`** : Диапазон `[0, 0]`
  Количество депозитов в протоколе Kinza.
  The number of deposits to make on the Kinza protocol.
  在 Kinza 协议上进行的存款次数。
- **`shmonad_deposit_count`** : Диапазон `[0, 0]`
  Количество депозитов в протоколе Shmonad.
  The number of deposits to make on the Shmonad protocol.
  在 Shmonad 协议上进行的存款次数。

---

## **Настройки NFT**

## **NFT Settings**

## **NFT 设置**

Эта настройка определяет количество NFT для минта. Случайное значение выбирается из указанного диапазона.

This setting determines the number of NFTs to mint. A random value is selected from the specified range.

此设置决定了要铸造的 NFT 数量。从指定范围中选择一个随机值。

- **`nad_domains_count`** : Диапазон `[0, 0]`
  Количество доменов NAD (NFT) для минта.
  The number of NAD domains (NFTs) to mint.
  要铸造的 NAD 域名（NFT）数量。

---

## **Прочие настройки**

## **Miscellaneous Settings**

## **其他设置**

Эти настройки управляют задержками, соотношениями и другими аспектами выполнения.

These settings control delays, ratios, and other runtime behaviors.

这些设置控制延迟、比例和其他运行时行为。

- **`thread_delay`** : Диапазон `[1000, 2000]` (в секундах)
  Базовое значение задержки (в секундах) для обработки аккаунтов.
  The base delay value (in seconds) for staggering the processing of accounts.
  用于交错处理账户的基本延迟值（以秒为单位）。
- **`restart_thread_delay`** : Фиксированное значение `5` (в секундах)
  Задержка перед перезапуском потока, в котором произошла ошибка.
  The delay before restarting a thread that has encountered an error.
  在遇到错误的线程重新启动之前的延迟。
- **`action_delay`** : Диапазон `[5, 5]` (в секундах)
  Случайная задержка (в секундах) между последовательными действиями на одном аккаунте. Это помогает имитировать реалистичное поведение пользователя.
  The random delay (in seconds) between consecutive actions on a single account. This helps mimic realistic user behavior.
  单个账户上连续操作之间的随机延迟（以秒为单位）。这有助于模拟真实的用户行为。
- **`deposit_ratio`** : Диапазон `[1, 2]` (в процентах)
  Процент баланса аккаунта, используемый для депозита. Случайное значение выбирается из этого диапазона для каждого депозита.
  The percentage of the account's balance to use during a deposit action. A random percentage within this range is selected for each deposit.
  在存款操作期间使用的账户余额百分比。每次存款时从此范围内选择一个随机百分比。
- **`swap_ratio`** : Диапазон `[5, 10]` (в процентах)
  Процент баланса аккаунта, используемый для свопа. Случайное значение выбирается из этого диапазона для каждого свопа.
  The percentage of the account's balance to use during a swap action. A random percentage within this range is selected for each swap.
  在交换操作期间使用的账户余额百分比。每次交换时从此范围内选择一个随机百分比。

---

## **RPC URL**

- **`rpc_url`** : `"https://testnet-rpc.monad.xyz"`
  RPC-эндпоинт для взаимодействия с блокчейном.
  The RPC endpoint used for interacting with the blockchain.
  用于与区块链交互的 RPC 端点。

---

## **Использование**

## **Usage**

## **使用说明**

1. Настройте диапазоны в файле конфигурации в соответствии с вашими потребностями.
   Adjust the ranges in the configuration file to suit your needs.
   根据您的需求调整配置文件中的范围。
2. Запустите приложение. Во время выполнения случайные значения будут выбраны из указанных диапазонов для каждой настройки.
   Run the application. At runtime, random values will be selected from the specified ranges for each setting.
   运行应用程序。在运行时，将为每个设置从指定范围中选择随机值。
3. Приложение будет использовать эти значения для определения количества свопов, депозитов, NFT, задержек и соотношений транзакций.
   The application will use these values to determine the number of swaps, deposits, NFTs, delays, and transaction ratios.
   应用程序将使用这些值来确定交换次数、存款次数、NFT、延迟和交易比例。

---

## **Пример**

## **Example**

## **示例**

Например, если `ambient_swap_count` установлен на `[1, 3]`, приложение случайным образом выберет значение от 1 до 3 (включительно) для количества свопов в Ambient. Аналогично, если `action_delay` установлен на `[5, 10]`, приложение будет ждать случайное время от 5 до 10 секунд между последовательными действиями на одном аккаунте.

For example, if `ambient_swap_count` is set to `[1, 3]`, the application will randomly select a value between 1 and 3 (inclusive) for the number of Ambient swaps to perform. Similarly, if `action_delay` is set to `[5, 10]`, the application will wait for a random duration between 5 and 10 seconds between consecutive actions on a single account.

例如，如果 `ambient_swap_count` 设置为 `[1, 3]`，应用程序将随机选择 1 到 3（包括）之间的值作为 Ambient 交换的次数。同样，如果 `action_delay` 设置为 `[5, 10]`，应用程序将在单个账户上的连续操作之间等待 5 到 10 秒的随机时间。

---

## **Примечания**

## **Notes**

## **注意事项**

- Убедитесь, что RPC URL корректен и доступен.
  Ensure that the RPC URL is correct and accessible.
  确保 RPC URL 正确且可访问。
- Настройте диапазоны осторожно, чтобы избежать неожиданного поведения, такого как чрезмерные задержки или недостаток средств для транзакций.
  Adjust the ranges carefully to avoid unexpected behavior, such as excessive delays or insufficient funds for transactions.
  仔细调整范围，以避免意外行为，例如过度延迟或交易资金不足。
- `restart_thread_delay` фиксирован на 5 секунд, чтобы обеспечить быстрый перезапуск потоков после ошибок.
  The `restart_thread_delay` is fixed to 5 seconds to ensure threads are restarted promptly after errors.
  `restart_thread_delay` 固定为 5 秒，以确保线程在错误后迅速重启。

---

Этот файл конфигурации обеспечивает гибкость и случайность для имитации реалистичного поведения пользователя в контролируемой среде. Настройте диапазоны по мере необходимости для достижения желаемых результатов.

This configuration file provides flexibility and randomness to simulate realistic user behavior in a controlled environment. Adjust the ranges as needed to achieve the desired outcomes.

此配置文件提供了灵活性和随机性，以在受控环境中模拟真实的用户行为。根据需要调整范围以实现预期结果。
