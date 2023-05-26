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

## Configuration

You can use the `-g, --generate-config` switch to generate a config file:

```cmd
sf-metadata-lint.exe --generate-config
```

If no config file is present in the project folder, the following standards will be used:

```json
{
  "rules": {
    "XmlFiles_no_invalid_files": "ERROR",
    "Flow_no_missing_description": "WARNING",
    "Picklist_no_missing_full_names": "ERROR",
    "CustomField_no_missing_descriptions": "WARNING",
    "Profile_no_unwanted_parts": "WARNING",
    "RecordType_no_missing_objects": "ERROR",
    "Picklist_no_empty_values": "ERROR",
    "PermissionSet_no_missing_objects": "ERROR",
    "RecordType_no_missing_fields": "ERROR",
    "PermissionSet_no_permission_on_required_field": "ERROR",
    "RecordType_no_missing_picklist_values": "ERROR",
    "Picklist_no_missing_global_value_set": "ERROR",
    "Layout_no_missing_fields": "ERROR",
    "Profile_no_missing_page_layouts": "ERROR",
    "XmlFiles_no_invalid_structs": "ERROR",
    "PermissionSet_no_missing_fields": "ERROR",
    "Translations_no_empty_translations": "WARNING",
    "CustomApplication_no_missing_description": "WARNING",
    "PermissionSet_no_invalid_field_names": "ERROR"
  }
}
```

The following report levels are available

| Level   | Description                                                                                                                  |
| ------- | ---------------------------------------------------------------------------------------------------------------------------- |
| OFF     | No reporting at all<br /> If possible the check will be skipped<br /> **Try in case of bugs with a rule**                    |
| INFO    | The error will be reported but has no influence on the return code. A script (e.g. in CI/CD) will not stop in case of infos. |
| WARNING | Will return an error code (Stops scripts), if warnings are not ignored <br/>`-i, --ignore-warnings `                         |
| ERROR   | Will always stop scripts                                                                                                     |

## Description of Rules

_Work in progress_
