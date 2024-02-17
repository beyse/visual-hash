# Visual Hashing

Visual Hashing Using Rust

## Getting Started

These instructions will guide you through setting up your development environment and running the Rust application.

### Prerequisites

- Docker: Ensure Docker is installed and running on your system. Visit [Docker's official website](https://www.docker.com/get-started) for installation instructions.
- Visual Studio Code: Install VS Code from [the official website](https://code.visualstudio.com/).
- Remote - Containers extension for VS Code: Install the extension from the VS Code Marketplace or search for "Remote - Containers" within VS Code extensions.

### Setting Up the Development Container

1. **Open the Project in VS Code**: Start VS Code and open the project folder.

2. **Reopen in Container**: With the project folder open, a popup might suggest reopening the project in a container. If so, select "Reopen in Container". Alternatively, use the Command Palette (`Ctrl+Shift+P` or `Cmd+Shift+P` on macOS) and select "Remote-Containers: Reopen in Container".

3. **Wait for the Container to Build**: The first time you open the project in a container, Docker will build the development environment. This process can take a few minutes. Subsequent loads will be much faster.

### Running the Rust Application

After setting up the development container, you can run the Rust application as follows:

1. **Open a Terminal in VS Code**: Use the integrated terminal in VS Code (`Terminal > New Terminal`).

2. **Build the Project**: Compile your Rust project with Cargo, Rust's package manager and build system, by running:

   ```bash
   cargo build
   ```

3. **Run the Application**: Execute your application with Cargo:

   ```bash
   cargo run
   ```

   Replace `cargo run` with any additional parameters your application requires, such as input files or command-line arguments.

### Additional Information

- **Developing Inside the Container**: With the development container setup, you can install additional VS Code extensions, debug the application, and make changes to the code. These changes will be persisted in your project folder.

- **Troubleshooting**: If you encounter any issues with the development container, refer to the [VS Code Remote - Containers documentation](https://code.visualstudio.com/docs/remote/containers) for troubleshooting tips.

## Contributing

No Guidelines exist.

## License

All Rights Reserved.