# Bootstrapper

Bootstrapper is a simple tool for using GNU Stow conditionally. You can define features with a
bash conditional and stow will only be run if the condition is true. I use it for seperating
my dotfiles between macOS and linux.

**DO NOT USE THIS**, there is definitely a better way to achieve this with the --ignore
flag or .stow-local-ignore (or simply having one bash script per feature and looping over 
the folders you need), but I used this as a Rust learning exercise and like the approach,
as botched as it is.

**ONLY TESTED ON MACOS AND LINUX**

## Usage

Make sure GNU stow is installed and in your path.

In the folder that contains your stow packages, create a file called `bootstrapper.json`.
You can use the following as a template:

```json
{
  "source": "./", // optional - defaults to current directory
  "target": "~/",
  "features": [
    {
      "name": "macOS features",
      "slug": "macos", // the prefix
      "enabled_command": "[[ $OSTYPE == 'darwin'* ]]" // optional - if not set feature is always enabled 
    },
    {
      "name": "Linux features",
      "slug": "linux",
      "enabled_command": "[[ $OSTYPE == 'linux'* ]]"
    },
    {
      "name": "Linux /opt/ features",
      "slug": "opt",
      "enabled_command": "[[ $OSTYPE == 'linux'* ]]",
      "target": "/opt/" // optional - override destination for single features"
    }
  ]
}
```

To assign features, add a prefix to the folder name. For example, if you have a package called
`vim` and you want to enable it on macOS and Linux, rename it to `{macos,linux}vim`. If you
want to enable it on macOS only, rename it to `{macos}vim`.

You can also override the target-path of a specific feature by specifying another "target" field.
If a single folder has multiple features with a target override it will be skipped.

Then run `bootstrapper (stow|unstow)` and it will stow/unstow the packages that match your features.
