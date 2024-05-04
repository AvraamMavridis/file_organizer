# File Categorizer

This project is a Rust application that categorizes files in a directory based on their file types. It creates a new directory for each category and moves the files into their respective directories.

# How to Use

```
cargo run /path/to/directory
```

# How to add in your command line

1. Download the latest release

2. Move the executable to /usr/local/bin (or any other directory in your PATH) so you can run it from anywhere.

```
sudo mv target/release/file_organizer /usr/local/bin
```

3. Add it to your zshrc (or bash) file

Open your ~/.zshrc file in a text editor and add an alias to run your program. Replace my_project with the name of your executable:

```
echo 'alias my_project="/usr/local/bin/my_project"' >> ~/.zshrc
```

4. Source your zshrc file: To make the changes take effect, you need to source your ~/.zshrc file:

```
source ~/.zshrc
```

# Contributing

Contributions are welcome. Please feel free to fork the project and submit a pull request with your changes.

# License

This project is licensed under the MIT License.
