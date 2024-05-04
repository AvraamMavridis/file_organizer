# File Categorizer

<img src="./logo.webp" alt="File Organizer Logo" width="300" height="300">

This project is a Rust application that categorizes files in a directory based on their file types. It creates a new directory for each category and moves the files into their respective directories.

## Example

### Before Running File Categorizer

```
file1.txt
file2.jpg
file3.pdf
file4.txt
file5.jpg
```

### After Running File Categorizer

```
- images/
-- file2.jpg
-- file5.jpg
- documents/
-- file1.txt
-- file3.pdf
-- file4.txt
```

# How to Use

```
cargo run /path/to/directory
```

By default it will only move the files in the root directory, if you want to move also the files in subdirectories pass the `-r` flag

```
cargo run /path/to/directory -r
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
