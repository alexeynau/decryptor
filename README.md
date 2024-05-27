# Decryptor

Decryptor is a command-line tool for processing results of event logger, extracting it and decrypting them using a provided key file.

## Usage

To use Decryptor, run the following command:

```
.\decryptor.exe <source_dir> <dest_dir> <path_to_key>
```

* `<source_dir>` - it is directory, where zip files are located (for e.g. `C:\Users\user\AppData\Local\Temp\EventLogger\session-8726aa43...`)
* `<dest_dir>` - it is a directory, where would be extracted files (for e.g. `C:\Users\user\Documents\test`)
* `<path_to_key>` - it is a path to file, that keeps a key for decryption. At this moment it usually stores in `<source_dir>\debug.bin` (for e.g. `C:\Users\user\AppData\Local\Temp\EventLogger\session-8726aa43...\debug.bin`)

When program is complited, you will have such directory:
```
<dest_dir>/
    <timestamp>.jpg
    ...
    <timestamp>.jpg
    data.bin
```