# SOLANA SOR SmartContract

## Overview

The SOLANA SOR (Smart Order Router) SmartContract repository implements a decentralized exchange (DEX) on the Solana blockchain. This project leverages the Solana Program Library (SPL) and Anchor framework to provide robust and efficient token swap functionality.

## Repository Structure

- `programs/`: Contains the Solana smart contracts (on-chain programs) for the DEX.
  - `src/`:
    - **adapters/**: Provides abstraction layers for integrating various external protocols and utilities into the DEX, ensuring a modular and extensible architecture.
    - **instructions/**: Contains the core program logic, handling essential DEX functionalities such as token swaps, liquidity management, and fee calculations.
    - **utils/**: Includes utility functions and helper methods used across the program.
- `Anchor.toml`: Configuration file for Anchor projects.
- `Cargo.toml`: Rust project configuration file.
- `package.json`: Contains dependencies and scripts for JavaScript-based tests or utilities.

## Key Features

- **Split Trading**: The DexRouter allows for split trading, enabling users to execute trades across multiple liquidity sources.
- **Security**: The contracts have been audited by okx inner audit team.
- **Extensible Architecture**: Easily integrates with other Solana-based programs and tokens.
- **Anchor Framework**: Utilizes Anchor for seamless program development and deployment.
- **High Performance**: Built on Solana, ensuring high throughput and low latency.


## Prerequisites

- **Rust**: Install Rust from [rustup.rs](https://rustup.rs/).
- **Solana CLI**: Install the Solana command-line tools from [Solana CLI Documentation](https://docs.solana.com/cli/install-solana-cli-tools) (Recommended version: **1.18.26** for compatibility).
- **Anchor Framework**: Install Anchor by running:

  ```bash
  cargo install --git https://github.com/coral-xyz/anchor avm --force
  ```

  Alternatively, follow the installation guide from the [Solana Documentation](https://solana.com/docs/intro/installation) to ensure compatibility and proper setup.(Recommended version: **0.30.0** and **0.30.1** for compatibility)
- **Node.js** and **npm** (or **yarn**) for JavaScript utilities.

---

## Installation and Usage

1. **Clone the Repository**:
   ```bash
   git clone https://github.com/okx/WEB3-DEX-SOLANA-OPENSOURCE.git
   cd WEB3-DEX-SOLANA-OPENSOURCE
   ```

2. **Install Dependencies**:
- With Yarn:
  ```bash
  yarn install
  ```

- With npm:
  ```bash
  npm install
  ```

3. **Build the Project**:
   Use Anchor to build the smart contracts:
   ```bash
   anchor build
   ```

4. **Deploy the Smart Contracts**:
   Configure your `Anchor.toml` with the appropriate cluster URL and deploy:
   ```bash
   anchor deploy
   ```

5. **Run Tests**:
   Use Anchor's test suite to ensure functionality:
   ```bash
   anchor test
   ```

---

# Contributing

There are several ways you can contribute to the SOR SmartContract project:

## Ways to Contribute

### Join Community Discussions
Join our [Discord community](https://discord.gg/3N9PHeNn) to help other developers troubleshoot their integration issues and share your experience with the SOR SmartContract. Our Discord is the main hub for technical discussions, questions, and real-time support.

### Open an Issue
- Open [issues](https://github.com/okx/WEB3-DEX-SOLANA-OPENSOURCE/issues) to suggest features or report minor bugs
- Before opening a new issue, search existing issues to avoid duplicates
- When requesting features, include details about use cases and potential impact

### Submit Pull Requests
1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests
5. Submit a pull request

### Pull Request Guidelines
- Discuss non-trivial changes in an issue first
- Include tests for new functionality
- Update documentation as needed
- Add a changelog entry describing your changes in the PR
- PRs should be focused and preferably address a single concern

## First Time Contributors
- Look for issues labeled "good first issue"
- Read through our documentation
- Set up your local development environment following the Installation guide

## Code Review Process
1. A maintainer will review your PR
2. Address any requested changes
3. Once approved, your PR will be merged

## Questions?
- Open a discussion [issue](https://github.com/okx/WEB3-DEX-SOLANA-OPENSOURCE/issues) for general questions
- Join our [community](https://discord.gg/3N9PHeNn) for real-time discussions
- Review existing issues and discussions

Thank you for contributing to the SOLANA SOR SmartContract repo!