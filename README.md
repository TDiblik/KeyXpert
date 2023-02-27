TODO:

- While preventing recursive remap and remapping from normal key to sys-key. It currentlly works by repeated firing an event of pressing the mapped button. This is bad performance wise, and use-case wise
- Remap shortcuts
- Read settings from json
- Add settings UI
  - base UI
    - ability to remap keys
    - ability to remap shortcuts
    - Ability to select whether or not to allow reccursive remapping
    - After remapping, restart mapper_service windows service
  - UI installs latest mapper_service from github
  - UI registers mapper_service as a windows service that starts after startup
  - Select keyboard layout or ability to install new keyboard layout
