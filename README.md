# fsd

## Usage
- `fsd --directory someDir --extensions js css` remove files from someDir with extensions js and css.

- `fsd --directory someDir --extensions js css --filenames .DS_Store readme.md --folders build dist` remove from someDir files with the extension css and js, with files named .DS_Store and readme.md and remove the folders build and sti along with all files in those directories.

- `fsd --directory node_modules --preset node_modules` remove from node_modules using the node_modules preset which removes unnecessary files and folders when using node_modules in production.