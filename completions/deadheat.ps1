# PowerShell completion for deadheat
Register-ArgumentCompleter -Native -CommandName deadheat -ScriptBlock {
  param($wordToComplete, $commandAst, $cursorPosition)
  $subcommands = @('run', 'race', 'closecall', 'adopt', 'discard', 'standings', 'series', 'flip', 'championship', 'totals', 'pairing', 'comeback', 'distance', 'rally', 'deadlock', 'streak', 'cluster', 'closing', 'opening', 'turns', 'lead', 'crossings', 'tenure', 'depths', 'spells', 'reigns', 'place', 'peak', 'recovery', 'laststand', 'firststand', 'firststrike', 'firstreign', 'lastreign', 'leadchanges', 'outset', 'tail', 'crosshead', '-h', '--help')
  $elements = $commandAst.CommandElements
  $argIndex = 0
  for ($i = 0; $i -lt $elements.Count; $i++) {
    $ext = $elements[$i].Extent
    if ($cursorPosition -le $ext.EndOffset) {
      $argIndex = $i
      break
    }
    $argIndex = $i + 1
  }
  if ($argIndex -eq 1) {
    $subcommands | Where-Object { $_.StartsWith($wordToComplete, [System.StringComparison]::Ordinal) } | ForEach-Object {
      [System.Management.Automation.CompletionResult]::new($_, $_, 'ParameterValue', $_)
    }
    return
  }
  if ($argIndex -ge 2 -and $wordToComplete.StartsWith('-')) {
    $sub = if ($elements.Count -ge 2) { $elements[1].Extent.Text } else { '' }
    $flags = if ($sub -eq 'race') { @('--standings', '-h', '--help') } else { @('-h', '--help') }
    $flags | Where-Object { $_.StartsWith($wordToComplete, [System.StringComparison]::Ordinal) } | ForEach-Object {
      [System.Management.Automation.CompletionResult]::new($_, $_, 'ParameterName', $_)
    }
    return
  }
}
