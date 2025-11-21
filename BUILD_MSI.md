# Build Instructions for Windows MSI

This project is configured to use `cargo-wix` to generate a Windows MSI installer.
Because the build process produces a Windows executable (`.exe`) and requires the WiX Toolset, **these steps must be run on a Windows machine**.

## Prerequisites

1.  **Rust**: Ensure Rust is installed (`rustup`).
2.  **Qt6**: Install Qt6. Ensure `windeployqt` is in your PATH.
3.  **WiX Toolset**: Install the WiX Toolset v3.11 or v4.
    *   Download from: https://wixtoolset.org/
    *   Add the WiX `bin` directory to your PATH.
4.  **cargo-wix**: Install the Cargo subcommand.
    ```powershell
    cargo install cargo-wix
    ```

## Building the Installer

The process involves building the binary, deploying dependencies, modifying the installer configuration to include those dependencies, and then packaging it.

### Step 1: Build and Deploy Dependencies

1.  Open a terminal (PowerShell or Command Prompt) in the `rustguard-vision` directory.
2.  Build the project in release mode:
    ```powershell
    cargo build --release
    ```
3.  Run `windeployqt` to copy the necessary Qt DLLs next to the executable:
    ```powershell
    windeployqt target/release/rustguard-vision.exe
    ```
    *This command will populate `target/release/` with files like `Qt6Core.dll`, `Qt6Gui.dll`, `plugins/`, etc.*

### Step 2: Configure the Installer (One-Time Setup)

The provided `wix/main.wxs` file is a template. It only packages the main executable. **You must update it to include the Qt dependencies generated in Step 1.**

1.  Open `wix/main.wxs` in a text editor.
2.  Look for the `<!-- Qt Dependency Placeholders -->` comment inside the `<Directory Id='Bin'...>` section.
3.  For every DLL (e.g., `Qt6Core.dll`) and folder (e.g., `plugins`) in your `target/release` folder, you need to add a corresponding WiX `<Component>` and `<File>` entry.

    **Example for a DLL:**
    ```xml
    <Component Id='Qt6Core' Guid='*'>
        <File Id='Qt6CoreDll' Name='Qt6Core.dll' DiskId='1' Source='$(var.CargoTargetBinDir)\Qt6Core.dll' KeyPath='yes'/>
    </Component>
    ```

4.  Then, scroll down to the `<Feature Id='Binaries'...>` section and add a reference to that component:
    ```xml
    <ComponentRef Id='Qt6Core'/>
    ```

    *Note: For directories like `plugins/`, you will need to create nested `<Directory>` elements in WiX. This can be tedious to do manually. Advanced users may use the WiX tool `heat.exe` to harvest these files automatically, but that is outside the scope of this basic guide.*

### Step 3: Generate the MSI

Once `wix/main.wxs` accurately reflects the files in `target/release`:

1.  Run the following command:
    ```powershell
    cargo wix
    ```

2.  The output MSI file will be located at:
    `target/wix/rustguard-vision-0.1.0-x86_64-pc-windows-msvc.msi`

This MSI will now install your executable along with all the required Qt libraries.
