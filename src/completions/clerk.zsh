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
        'orgs:organization:_clerk_orgs' \
        'commands:subcommand:_clerk_orgs_subcommands'
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
                _arguments "${_arguments_options[@]}" : \
                    '-l+[Number of users to fetch]:LIMIT:' \
                    '--limit=[Number of users to fetch]:LIMIT:' \
                    '-q+[Search query (email/name)]:QUERY:' \
                    '--query=[Search query (email/name)]:QUERY:' \
                    '-h[Print help]' \
                    '--help[Print help]' \
                    && ret=0
                ;;
            (orgs)
                _arguments "${_arguments_options[@]}" : \
                    '-h[Print help]' \
                    '--help[Print help]' \
                    '1::ORG or COMMAND:_clerk_orgs_and_subcommands' \
                    '*::: :->orgs_arg' \
                    && ret=0
                case $state in
                (orgs_arg)
                    case $line[1] in
                        (list)
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
                            _arguments "${_arguments_options[@]}" : \
                                '-h[Print help]' \
                                '--help[Print help]' \
                                && ret=0
                            ;;
                        (members)
                            ;;
                        (*)
                            _arguments "${_arguments_options[@]}" : \
                                '1::SUBCOMMAND:_clerk_org_actions' \
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
        esac
        ;;
    esac

    return ret
}

_clerk_commands() {
    local commands
    commands=(
        'users:List and search users'
        'orgs:Manage organizations'
        'impersonate:Generate a sign-in link to impersonate a user'
        'jwt:Generate a JWT for API testing'
        'completions:Generate shell completions'
    )
    _describe -t commands 'clerk commands' commands
}

_clerk_orgs_subcommands() {
    local commands
    commands=(
        'list:List all organizations'
        'pick:Interactively pick an organization'
        'members:List members of the organization'
    )
    _describe -t commands 'subcommand' commands
}

_clerk_org_actions() {
    local commands
    commands=(
        'members:List members of this organization'
    )
    _describe -t commands 'action' commands
}

if [ "$funcstack[1]" = "_clerk" ]; then
    _clerk "$@"
else
    compdef _clerk clerk
fi
