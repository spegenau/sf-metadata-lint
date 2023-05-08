# Salesforce Metadata Lint

Checks the Salesforce Metadata in a project for common mistakes to avoid time-consuming deployment errors, like

- Missed parts in your commits, i.e. you try to deploy a page layout without the field
- Destroyed XMLs for example when merging XML-files or selecting only parts of it

## How to use it...

### Manually

1. Execute the file for your OS in the release section
2. Execute the file in a SFDX project folder or pass the path manually:
   ```
   ./sf-metadata-lint ~/projects/your_amazing_project
   ```

### in a CI/CD Pipeline

If you use GitHub Actions you could use the following snippet to always use the most recent release:

```
    - name: 'Run Linter'
        run: |
            wget https://github.com/spegenau/sf-metadata-lint/releases/latest/download/sf-metadata-lint-x86_64-unknown-linux-gnu.tar.gz -O linter.tar.gz -o wget.log
            tar xf linter.tar.gz
            chmod +x sf-metadata-lint
            ./sf-metadata-lint
```

🔥 ATTENTION: For more stability you should use a certain version as I have no idea what will come to my mind and what I will break in the future! 😈

### in a Git Hook

_your need to figure that yourself...Sorry..._

## What errors can it find?

| Metadata Type   | Error                                                                                                                                      |
| --------------- | ------------------------------------------------------------------------------------------------------------------------------------------ |
| All XML-Files   | - unparseable files                                                                                                                        |
| Permission Sets | - permissions for non-existing fields                                                                                                      |
|                 | - permissions for required fields                                                                                                          |
| Profiles        | - assignments of layouts that are not available                                                                                            |
|                 | - Best practice: Use Permission Sets instead to give access to Record Types, Apex Classes, Pages, Objects, Fields, Flows, User Permissions |
| Translations    | - Empty app translations                                                                                                                   |
|                 | - Empty label translations                                                                                                                 |
|                 | - Empty tab translations                                                                                                                   |
|                 | - Empty quick action translations                                                                                                          |
|                 | - Empty flow translations                                                                                                                  |
