# bash completion for deadheat
_deadheat() {
  local cur
  COMPREPLY=()
  cur="${COMP_WORDS[COMP_CWORD]}"
  if [[ $COMP_CWORD -eq 1 ]]; then
    COMPREPLY=( $(compgen -W "run race closecall adopt discard standings series flip championship totals pairing comeback distance rally deadlock streak cluster closing opening turns lead crossings tenure depths spells reigns place peak recovery laststand firststand firststrike firstreign lastreign leadchanges outset tail crosshead -h --help" -- "$cur") )
    return 0
  fi
  if [[ "${COMP_WORDS[1]}" == "race" && "$cur" == -* ]]; then
    COMPREPLY=( $(compgen -W "--standings -h --help" -- "$cur") )
    return 0
  fi
  if [[ "$cur" == -* ]]; then
    COMPREPLY=( $(compgen -W "-h --help" -- "$cur") )
    return 0
  fi
  return 0
}
complete -o bashdefault -o default -F _deadheat deadheat
