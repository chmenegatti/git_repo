
<div align="center">
<h1 align="center">
<img src="https://raw.githubusercontent.com/PKief/vscode-material-icon-theme/ec559a9f6bfd399b82bb44393651661b08aaf7ba/icons/folder-markdown-open.svg" width="100" />
<br>git_repo
</h1>
<h3>◦ Unleash the power of collaboration with git_repo!</h3>
<h3>◦ Developed with the software and tools listed below.</h3>

<p align="center">
<img src="https://img.shields.io/badge/SemVer-3F4551.svg?style&logo=SemVer&logoColor=white" alt="SemVer" />
<img src="https://img.shields.io/badge/curl-073551.svg?style&logo=curl&logoColor=white" alt="curl" />
<img src="https://img.shields.io/badge/JSON-000000.svg?style&logo=JSON&logoColor=white" alt="JSON" />
<img src="https://img.shields.io/badge/Rust-000000.svg?style&logo=Rust&logoColor=white" alt="Rust" />
</p>
<img src="https://img.shields.io/github/languages/top/chmenegatti/git_repo.git?style&color=5D6D7E" alt="GitHub top language" />
<img src="https://img.shields.io/github/languages/code-size/chmenegatti/git_repo.git?style&color=5D6D7E" alt="GitHub code size in bytes" />
<img src="https://img.shields.io/github/commit-activity/m/chmenegatti/git_repo.git?style&color=5D6D7E" alt="GitHub commit activity" />
<img src="https://img.shields.io/github/license/chmenegatti/git_repo.git?style&color=5D6D7E" alt="GitHub license" />
</div>

---

## 📒 Table of Contents
- [📒 Table of Contents](#-table-of-contents)
- [📍 Overview](#-overview)
- [⚙️ Features](#-features)
- [📂 Project Structure](#project-structure)
- [🧩 Modules](#modules)
- [🚀 Getting Started](#-getting-started)
- [🗺 Roadmap](#-roadmap)
- [🤝 Contributing](#-contributing)
- [📄 License](#-license)
- [👏 Acknowledgments](#-acknowledgments)

---


## 📍 Overview

The project is a command-line program that fetches and displays a GitHub user's information. It provides a convenient way for users to retrieve and view key details about a GitHub user, such as their name, location, company, number of public repositories, and bio. This tool adds value by simplifying the process of accessing and retrieving user information from the GitHub API.

---

## ⚙️ Features

| Feature                | Description                           |
| ---------------------- | ------------------------------------- |
| **⚙️ Architecture**     | The codebase follows a simple command-line architecture design. The main.rs file contains the entry point of the program and handles the user interaction. The core functionality of fetching and displaying GitHub user information is encapsulated in separate functions or modules. The codebase utilizes HTTP requests to interact with the GitHub API and retrieve user data. Overall, the architecture is straightforward and focused on the specific task of retrieving and displaying user information. |
| **📖 Documentation**   | The codebase lacks comprehensive documentation. While the main.rs file provides some comments explaining the purpose of different sections, there could be more detailed documentation to clarify the functionality of individual functions, modules, and data structures. This would make it easier for developers to understand and modify the codebase. Additional documentation, including a README file, would also be helpful for providing an overview of the project and its usage. |
| **🔗 Dependencies**    | The codebase has a few dependencies. It relies on the reqwest library to handle HTTP requests and communicate with the GitHub API. This library simplifies the process of sending requests and receiving responses. Additionally, the codebase utilizes the serde and serde_json crates for JSON serialization and deserialization. These dependencies are crucial for parsing the response from the GitHub API and extracting relevant user information. Overall, the codebase has minimal external dependencies, which helps keep it lightweight. |
| **🧩 Modularity**      | The codebase demonstrates a moderate level of modularity. The main functionality is split into separate functions or modules, each responsible for a specific task, such as prompting for user input, sending HTTP requests, and printing user information. This division of responsibilities allows for easier maintenance and testing of individual components. However, there is room for improvement in terms of further modularizing the codebase. Breaking down complex functions into smaller, reusable components would enhance the overall modularity. |
| **✔️ Testing**          | The codebase does not have any testing strategies or tools implemented. This is a significant drawback as having a comprehensive testing suite ensures the reliability and correctness of the code. Adding unit tests for each function or module would improve the overall quality of the project and help catch any potential bugs or regressions. Implementing a testing framework, such as Rust's built-in testing framework or a third-party library like `rusty` or `mocktopus`, would facilitate the creation and execution of tests. |
| **⚡️ Performance**      | The codebase's performance can be considered satisfactory. The program sends a single HTTP request to the GitHub API and parses the response to extract user information. As long as the network connection is stable, the program's speed and efficiency depend on the responsiveness of the GitHub API. The codebase itself doesn't have any performance optimizations, but it doesn't perform any computationally intensive tasks that would require specific optimizations

---


## 📂 Project Structure




---

## 🧩 Modules

<details closed><summary>Src</summary>

| File                                                                         | Summary                                                                                                                                                                                                                                                                                                                                                                                                                   |
| ---                                                                          | ---                                                                                                                                                                                                                                                                                                                                                                                                                       |
| [main.rs](https://github.com/chmenegatti/git_repo.git/blob/main/src/main.rs) | This code snippet is a command-line program that fetches and displays a GitHub user's information. It prompts the user for a GitHub username, sends an HTTP request to the GitHub API with the username and an authentication token, and then prints the user's name, location, company, number of public repositories, and bio. If there's an error or the user doesn't exist, it displays an appropriate error message. |

</details>

---

## 🚀 Getting Started

### ✔️ Prerequisites

Before you begin, ensure that you have the following prerequisites installed:
> - `ℹ️ Requirement 1`
> - `ℹ️ Requirement 2`
> - `ℹ️ ...`

### 📦 Installation

1. Clone the git_repo repository:
```sh
git clone https://github.com/chmenegatti/git_repo.git
```

2. Change to the project directory:
```sh
cd git_repo
```

3. Install the dependencies:
```sh
cargo build
```

### 🎮 Using git_repo

```sh
cargo run
```

### 🧪 Running Tests
```sh
cargo test
```

---


## 🗺 Roadmap

> - [X] `ℹ️  Task 1: Implement X`
> - [ ] `ℹ️  Task 2: Refactor Y`
> - [ ] `ℹ️ ...`


---

## 🤝 Contributing

Contributions are always welcome! Please follow these steps:
1. Fork the project repository. This creates a copy of the project on your account that you can modify without affecting the original project.
2. Clone the forked repository to your local machine using a Git client like Git or GitHub Desktop.
3. Create a new branch with a descriptive name (e.g., `new-feature-branch` or `bugfix-issue-123`).
```sh
git checkout -b new-feature-branch
```
4. Make changes to the project's codebase.
5. Commit your changes to your local branch with a clear commit message that explains the changes you've made.
```sh
git commit -m 'Implemented new feature.'
```
6. Push your changes to your forked repository on GitHub using the following command
```sh
git push origin new-feature-branch
```
7. Create a new pull request to the original project repository. In the pull request, describe the changes you've made and why they're necessary.
The project maintainers will review your changes and provide feedback or merge them into the main branch.

---

## 📄 License

This project is licensed under the `ℹ️  INSERT-LICENSE-TYPE` License. See the [LICENSE](https://docs.github.com/en/communities/setting-up-your-project-for-healthy-contributions/adding-a-license-to-a-repository) file for additional info.

---

## 👏 Acknowledgments

> - `ℹ️  List any resources, contributors, inspiration, etc.`

---
