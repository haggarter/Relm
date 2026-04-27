# Relm
An SDDE Operating System for Individuals and Organizations

## SDDE Philosophy
- Declarative desktop environment
- Tiered security
- Centralized controller for organizations

## Design
- Arch-based
- A Realm = A desktop environment
- Realms defined by YAML
    - storage
        - direct: Direct access to root storage
        - persistent: Isolated to Realm, persists between sessions
        - ephemeral: Isolated to Realm, 
    - apps
        - core: Directly installed in Realm
            - App 1
            - App 2
            - ...
            - immutable: True/False
        - vault: Installed within an isolated VM
            - App 1
            - App 2
            - ...
    - network
        - allow:
            - Protocol 1:Port 1
            - Protocol 2:Port 2
            - ...
- Commands:
```
relm create <path to .yaml>
relm delete <Realm name>
relm start <Realm name>
relm switch <Realm name>
relm list
```