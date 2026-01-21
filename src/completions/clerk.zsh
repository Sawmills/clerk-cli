#compdef clerk

autoload -U is-at-least

_clerk_orgs() {
    local -a orgs
    local line
    while IFS=: read -r slug name; do
        orgs+=("${slug}:${name}")
    done < <(clerk complete-orgs 2>/dev/null)
    _describe -t orgs 'organization' orgs
}

_clerk_orgs_and_subcommands() {
    _alternative \
        'commands:subcommand:_clerk_orgs_subcommands' \
        'orgs:organization:_clerk_orgs'
}

_clerk_users_and_subcommands() {
    _alternative \
        'commands:subcommand:_clerk_users_subcommands' \
        'users:user:_clerk_all_users'
}

_clerk() {
    typeset -A opt_args
    typeset -a _arguments_options
    local ret=1

    if is-at-least 5.2; then
        _arguments_options=(-s -S -C)
    else
        _arguments_options=(-s -C)
    fi

    local context curcontext="$curcontext" state line
    _arguments "${_arguments_options[@]}" : \
        '-h[Print help]' \
        '--help[Print help]' \
        '-V[Print version]' \
        '--version[Print version]' \
        ":: :_clerk_commands" \
        "*::: :->clerk" \
        && ret=0

    case $state in
    (clerk)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:clerk-command-$line[1]:"

        case $line[1] in
            (users)
                local first_arg="${words[2]}"
                local second_arg="${words[3]}"
                case $first_arg in
                    (list)
                        # Shift context for _arguments to work with "list" subcommand
                        local -a list_words=("list" "${words[@]:2}")
                        local list_current=$((CURRENT - 1))
                        words=("${list_words[@]}")
                        CURRENT=$list_current
                        _arguments "${_arguments_options[@]}" : \
                            '-l+[Number of users to fetch]:LIMIT:' \
                            '--limit=[Number of users to fetch]:LIMIT:' \
                            '-q+[Search query (email/name)]:QUERY:' \
                            '--query=[Search query (email/name)]:QUERY:' \
                            '-h[Print help]' \
                            '--help[Print help]' \
                            && ret=0
                        ;;
                    (create)
                        # Shift context for _arguments to work with "create" subcommand
                        local -a create_words=("create" "${words[@]:2}")
                        local create_current=$((CURRENT - 1))
                        words=("${create_words[@]}")
                        CURRENT=$create_current
                        _arguments "${_arguments_options[@]}" : \
                            '-e+[Email address]:EMAIL:' \
                            '--email=[Email address]:EMAIL:' \
                            '-f+[First name]:FIRST_NAME:' \
                            '--first-name=[First name]:FIRST_NAME:' \
                            '-l+[Last name]:LAST_NAME:' \
                            '--last-name=[Last name]:LAST_NAME:' \
                            '-p+[Password]:PASSWORD:' \
                            '--password=[Password]:PASSWORD:' \
                            '-h[Print help]' \
                            '--help[Print help]' \
                            && ret=0
                        ;;
                    (*)
                        case $second_arg in
                            (impersonate)
                                _arguments "${_arguments_options[@]}" : \
                                    '-h[Print help]' \
                                    '--help[Print help]' \
                                    && ret=0
                                ;;
                            (jwt)
                                if (( CURRENT == 4 )); then
                                    _clerk_jwt_templates && ret=0
                                fi
                                ;;
                            (add-to-org)
                                local -a cmd_words=("add-to-org" "${words[@]:3}")
                                local cmd_current=$((CURRENT - 2))
                                words=("${cmd_words[@]}")
                                CURRENT=$cmd_current
                                _arguments "${_arguments_options[@]}" : \
                                    '-o+[Organization slug or ID]:ORG:_clerk_orgs' \
                                    '--org=[Organization slug or ID]:ORG:_clerk_orgs' \
                                    '-r+[Role]:ROLE:(org:admin org:member)' \
                                    '--role=[Role]:ROLE:(org:admin org:member)' \
                                    '-h[Print help]' \
                                    '--help[Print help]' \
                                    && ret=0
                                ;;
                            (remove-from-org)
                                local -a cmd_words=("remove-from-org" "${words[@]:3}")
                                local cmd_current=$((CURRENT - 2))
                                words=("${cmd_words[@]}")
                                CURRENT=$cmd_current
                                _arguments "${_arguments_options[@]}" : \
                                    '-o+[Organization slug or ID]:ORG:_clerk_orgs' \
                                    '--org=[Organization slug or ID]:ORG:_clerk_orgs' \
                                    '-h[Print help]' \
                                    '--help[Print help]' \
                                    && ret=0
                                ;;
                            (*)
                                if (( CURRENT == 2 )); then
                                    _clerk_users_and_subcommands && ret=0
                                elif (( CURRENT == 3 )); then
                                    _clerk_user_actions && ret=0
                                fi
                                ;;
                        esac
                        ;;
                esac
                ;;
            (orgs)
                # After word manipulation: words=(orgs <arg1> <arg2> ...)
                # words[2]=first_arg (org slug or subcommand), words[3]=second_arg
                local first_arg="${words[2]}"
                local second_arg="${words[3]}"
                case $first_arg in
                    (list)
                        # Shift context for _arguments to work with "list" subcommand
                        local -a list_words=("list" "${words[@]:2}")
                        local list_current=$((CURRENT - 1))
                        words=("${list_words[@]}")
                        CURRENT=$list_current
                        _arguments "${_arguments_options[@]}" : \
                            '-l+[Number of orgs to fetch]:LIMIT:' \
                            '--limit=[Number of orgs to fetch]:LIMIT:' \
                            '-f+[Fuzzy search by name]:FUZZY:_clerk_orgs' \
                            '--fuzzy=[Fuzzy search by name]:FUZZY:_clerk_orgs' \
                            '-i[Output only org IDs]' \
                            '--ids-only[Output only org IDs]' \
                            '-h[Print help]' \
                            '--help[Print help]' \
                            && ret=0
                        ;;
                    (pick)
                        # Shift context for _arguments to work with "pick" subcommand
                        local -a pick_words=("pick" "${words[@]:2}")
                        local pick_current=$((CURRENT - 1))
                        words=("${pick_words[@]}")
                        CURRENT=$pick_current
                        _arguments "${_arguments_options[@]}" : \
                            '-h[Print help]' \
                            '--help[Print help]' \
                            && ret=0
                        ;;
                    (create)
                        # Shift context for _arguments to work with "create" subcommand
                        # words=(orgs create ...) -> words=(create ...)
                        local -a create_words=("create" "${words[@]:2}")
                        local create_current=$((CURRENT - 1))
                        words=("${create_words[@]}")
                        CURRENT=$create_current
                        _arguments "${_arguments_options[@]}" : \
                            '-n+[Organization name]:NAME:' \
                            '--name=[Organization name]:NAME:' \
                            '-s+[Organization slug]:SLUG:' \
                            '--slug=[Organization slug]:SLUG:' \
                            '-h[Print help]' \
                            '--help[Print help]' \
                            && ret=0
                        ;;
                    (*)
                        case $second_arg in
                            (members)
                                # clerk orgs <org> members <user> <action> [template]
                                # clerk orgs <org> members add -u <user> -r <role>
                                # words=(orgs <org> members [user|add] [action] [template])
                                # words[4]=user|add, words[5]=action, CURRENT=position being completed
                                local member_arg="${words[4]}"
                                local action_arg="${words[5]}"
                                if [[ "$member_arg" == "add" ]]; then
                                    # Shift context for _arguments to work with "add" subcommand
                                    local -a add_words=("add" "${words[@]:4}")
                                    local add_current=$((CURRENT - 3))
                                    words=("${add_words[@]}")
                                    CURRENT=$add_current
                                    _arguments "${_arguments_options[@]}" : \
                                        '-u+[User ID to add]:USER_ID:_clerk_all_users' \
                                        '--user=[User ID to add]:USER_ID:_clerk_all_users' \
                                        '-r+[Role for the new member]:ROLE:(org:admin org:member)' \
                                        '--role=[Role for the new member]:ROLE:(org:admin org:member)' \
                                        '-h[Print help]' \
                                        '--help[Print help]' \
                                        && ret=0
                                elif (( CURRENT == 4 )); then
                                    _alternative \
                                        'actions:action:(add)' \
                                        'members:member:_clerk_members' \
                                    && ret=0
                                elif (( CURRENT == 5 )); then
                                    # Position after <user> - show member actions
                                    _clerk_member_actions && ret=0
                                elif (( CURRENT == 6 )) && [[ "$action_arg" == "jwt" ]]; then
                                    # Position after "jwt" - show templates
                                    _clerk_jwt_templates && ret=0
                                fi
                                ;;
                            (delete)
                                # Shift context for _arguments
                                local -a del_words=("delete" "${words[@]:3}")
                                local del_current=$((CURRENT - 2))
                                words=("${del_words[@]}")
                                CURRENT=$del_current
                                _arguments "${_arguments_options[@]}" : \
                                    '-f[Skip confirmation prompt]' \
                                    '--force[Skip confirmation prompt]' \
                                    '-h[Print help]' \
                                    '--help[Print help]' \
                                    && ret=0
                                ;;
                            (sso)
                                local sso_cmd="${words[4]}"
                                if (( CURRENT == 4 )); then
                                    _clerk_sso_subcommands && ret=0
                                elif [[ "$sso_cmd" == "list" ]]; then
                                    _arguments "${_arguments_options[@]}" : \
                                        '-h[Print help]' \
                                        '--help[Print help]' \
                                        && ret=0
                                elif [[ "$sso_cmd" == "add" ]]; then
                                    local -a add_words=("add" "${words[@]:4}")
                                    local add_current=$((CURRENT - 3))
                                    words=("${add_words[@]}")
                                    CURRENT=$add_current
                                    _arguments "${_arguments_options[@]}" : \
                                        '-n+[Connection name]:NAME:' \
                                        '--name=[Connection name]:NAME:' \
                                        '-p+[SAML provider]:PROVIDER:_clerk_saml_providers' \
                                        '--provider=[SAML provider]:PROVIDER:_clerk_saml_providers' \
                                        '-d+[Domain]:DOMAIN:' \
                                        '--domain=[Domain]:DOMAIN:' \
                                        '--entity-id=[IdP Entity ID]:ENTITY_ID:' \
                                        '--sso-url=[IdP SSO URL]:SSO_URL:' \
                                        '--certificate=[IdP Certificate]:CERTIFICATE:' \
                                        '--metadata-url=[IdP Metadata URL]:METADATA_URL:' \
                                        '-h[Print help]' \
                                        '--help[Print help]' \
                                        && ret=0
                                elif [[ "$sso_cmd" == "update" ]]; then
                                    local -a upd_words=("update" "${words[@]:4}")
                                    local upd_current=$((CURRENT - 3))
                                    words=("${upd_words[@]}")
                                    CURRENT=$upd_current
                                    _arguments "${_arguments_options[@]}" : \
                                        '1:CONNECTION:_clerk_sso_connections' \
                                        '-n+[New connection name]:NAME:' \
                                        '--name=[New connection name]:NAME:' \
                                        '-p+[SAML provider]:PROVIDER:_clerk_saml_providers' \
                                        '--provider=[SAML provider]:PROVIDER:_clerk_saml_providers' \
                                        '-d+[Domain]:DOMAIN:' \
                                        '--domain=[Domain]:DOMAIN:' \
                                        '--active=[Set active state]:ACTIVE:(true false)' \
                                        '--entity-id=[IdP Entity ID]:ENTITY_ID:' \
                                        '--sso-url=[IdP SSO URL]:SSO_URL:' \
                                        '--certificate=[IdP Certificate]:CERTIFICATE:' \
                                        '--metadata-url=[IdP Metadata URL]:METADATA_URL:' \
                                        '-h[Print help]' \
                                        '--help[Print help]' \
                                        && ret=0
                                elif [[ "$sso_cmd" == "delete" ]]; then
                                    local -a del_words=("delete" "${words[@]:4}")
                                    local del_current=$((CURRENT - 3))
                                    words=("${del_words[@]}")
                                    CURRENT=$del_current
                                    _arguments "${_arguments_options[@]}" : \
                                        '1:CONNECTION:_clerk_sso_connections' \
                                        '-f[Skip confirmation prompt]' \
                                        '--force[Skip confirmation prompt]' \
                                        '-h[Print help]' \
                                        '--help[Print help]' \
                                        && ret=0
                                fi
                                ;;
                            (*)
                                _arguments "${_arguments_options[@]}" : \
                                    '-h[Print help]' \
                                    '--help[Print help]' \
                                    '1::ORG or COMMAND:_clerk_orgs_and_subcommands' \
                                    '2::ACTION:_clerk_org_actions' \
                                    && ret=0
                                ;;
                        esac
                        ;;
                esac
                ;;
            (impersonate)
                _arguments "${_arguments_options[@]}" : \
                    '-h[Print help]' \
                    '--help[Print help]' \
                    '::USER_ID:' \
                    && ret=0
                ;;
            (jwt)
                _arguments "${_arguments_options[@]}" : \
                    '-t+[JWT template name]:TEMPLATE:' \
                    '--template=[JWT template name]:TEMPLATE:' \
                    '--list[List available templates]' \
                    '-h[Print help]' \
                    '--help[Print help]' \
                    '::USER_ID:' \
                    && ret=0
                ;;
            (completions)
                _arguments "${_arguments_options[@]}" : \
                    '-h[Print help]' \
                    '--help[Print help]' \
                    ':SHELL:(bash elvish fish powershell zsh)' \
                    && ret=0
                ;;
            (sso)
                local sso_cmd="${words[2]}"
                if (( CURRENT == 2 )); then
                    _clerk_top_sso_subcommands && ret=0
                elif [[ "$sso_cmd" == "list" ]]; then
                    _arguments "${_arguments_options[@]}" : \
                        '-h[Print help]' \
                        '--help[Print help]' \
                        && ret=0
                fi
                ;;
        esac
        ;;
    esac

    return ret
}

_clerk_commands() {
    local commands
    commands=(
        'users:Manage users'
        'orgs:Manage organizations'
        'sso:Manage SSO connections'
        'impersonate:Generate a sign-in link to impersonate a user'
        'jwt:Generate a JWT for API testing'
        'completions:Generate shell completions'
    )
    _describe -t commands 'clerk commands' commands
}

_clerk_users_subcommands() {
    local commands
    commands=(
        'list:List and search users'
        'create:Create a new user'
    )
    _describe -t commands 'subcommand' commands
}

_clerk_user_actions() {
    local commands
    commands=(
        'impersonate:Impersonate this user'
        'jwt:Generate a JWT for this user'
        'add-to-org:Add this user to an organization'
        'remove-from-org:Remove this user from an organization'
    )
    _describe -t commands 'action' commands
}

_clerk_orgs_subcommands() {
    local commands
    commands=(
        'list:List all organizations'
        'create:Create a new organization'
        'pick:Interactively pick an organization'
        'members:List members of the organization'
        'delete:Delete this organization'
        'sso:Manage SSO connections'
    )
    _describe -t commands 'subcommand' commands
}

_clerk_org_actions() {
    local commands
    commands=(
        'members:List members of this organization'
        'delete:Delete this organization'
        'sso:Manage SSO connections'
    )
    _describe -t commands 'action' commands
}

_clerk_sso_subcommands() {
    local commands
    commands=(
        'list:List SSO connections'
        'add:Add a SAML connection'
        'update:Update a SAML connection'
        'delete:Delete a SAML connection'
    )
    _describe -t commands 'subcommand' commands
}

_clerk_top_sso_subcommands() {
    local commands
    commands=(
        'list:List all SSO connections'
    )
    _describe -t commands 'subcommand' commands
}

_clerk_sso_connections() {
    local org_slug="${words[2]}"
    local -a connections
    local line
    while IFS=: read -r name desc; do
        connections+=("${name}:${desc}")
    done < <(clerk complete-sso-connections --org "$org_slug" 2>/dev/null)
    _describe -t connections 'connection' connections
}

_clerk_saml_providers() {
    local providers
    providers=(
        'saml_okta:Okta'
        'saml_google:Google Workspace'
        'saml_microsoft:Microsoft Entra ID (Azure AD)'
        'saml_custom:Custom SAML IdP'
    )
    _describe -t providers 'provider' providers
}

_clerk_members() {
    local org_slug="${words[2]}"
    local -a members
    local line
    while IFS=: read -r user_id desc; do
        members+=("${user_id}:${desc}")
    done < <(clerk complete-users --org "$org_slug" 2>/dev/null)
    _describe -t members 'member' members
}

_clerk_all_users() {
    local -a users
    local line
    while IFS=: read -r user_id desc; do
        users+=("${user_id}:${desc}")
    done < <(clerk complete-users 2>/dev/null)
    _describe -t users 'user' users
}

_clerk_member_actions() {
    local commands
    commands=(
        'impersonate:Impersonate this user'
        'jwt:Generate a JWT for this user'
    )
    _describe -t commands 'action' commands
}

_clerk_jwt_templates() {
    local -a templates
    local line
    while IFS=: read -r name desc; do
        templates+=("${name}:${desc}")
    done < <(clerk complete-jwt-templates 2>/dev/null)
    _describe -t templates 'template' templates
}

if [ "$funcstack[1]" = "_clerk" ]; then
    _clerk "$@"
else
    compdef _clerk clerk
fi
