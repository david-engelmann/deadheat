# PowerShell completion for deadheat
Register-ArgumentCompleter -Native -CommandName deadheat -ScriptBlock {
  param($wordToComplete, $commandAst, $cursorPosition)
  $subcommands = @('run', 'race', 'closecall', 'adopt', 'discard', 'standings', 'series', 'flip', 'championship', 'totals', 'pairing', 'comeback', 'distance', 'rally', 'deadlock', 'streak', 'cluster', 'closing', 'opening', 'turns', 'lead', 'crossings', 'tenure', 'depths', 'spells', 'reigns', 'place', 'peak', 'recovery', 'laststand', 'firststand', 'firststrike', 'firstreign', 'lastreign', 'leadchanges', 'outset', 'tail', 'crosshead', '-h', '--help')
  $elements = $commandAst.CommandElements
  if ($elements.Count -eq 1 -or ($elements.Count -eq 2 -and $wordToComplete.Length -gt 0)) {
    $subcommands | Where-Object { $_ -like "$wordToComplete*" } | ForEach-Object {
      [System.Management.Automation.CompletionResult]::new($_, $_, 'ParameterValue', $_)
    }
    return
  }
  if ($elements.Count -ge 2 -and $wordToComplete.StartsWith('-')) {
    $sub = $elements[1].Value
    $flags = if ($sub -eq 'race') { @('--standings', '-h', '--help') } else { @('-h', '--help') }
    $flags | Where-Object { $_ -like "$wordToComplete*" } | ForEach-Object {
      [System.Management.Automation.CompletionResult]::new($_, $_, 'ParameterName', $_)
    }
    return
  }
}
