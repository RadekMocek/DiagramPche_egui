Write-Host "== DiagramPche_egui benchmarks =="
Write-Host "`nThis script:`n`t* will run all the possible benchmarks`n`t* expects to be run from the .\assets\scripts folder."

Write-Host "`n[!] Epilepsy/seizure warning: benchmarks (especially the last one!) contain flashing images.`n"
Write-Host "Press enter to continue. Press Ctrl+C to cancel."
Read-Host

$i = 1
$j = 10
cd..
cd..
Set-Location target
Set-Location release

Write-Host "[${i}/${j}] BENCH NODES LIGHT NO_SYNTAX_HIGHLIGHT"
.\diagram_pche_egui.exe b 0 0 | Out-Null
$i++

Write-Host "[${i}/${j}] BENCH NODES LIGHT YES_SYNTAX_HIGHLIGHT"
.\diagram_pche_egui.exe b 0 1 | Out-Null
$i++

Write-Host "[${i}/${j}] BENCH NODES LIGHT NO_TEXT_EDITOR"
.\diagram_pche_egui.exe b 0 2 | Out-Null
$i++

Write-Host "[${i}/${j}] BENCH NODES HEAVY NO_SYNTAX_HIGHLIGHT"
.\diagram_pche_egui.exe b 1 0 | Out-Null
$i++

Write-Host "[${i}/${j}] BENCH NODES HEAVY YES_SYNTAX_HIGHLIGHT"
.\diagram_pche_egui.exe b 1 1 | Out-Null
$i++

Write-Host "[${i}/${j}] BENCH NODES HEAVY NO_TEXT_EDITOR"
.\diagram_pche_egui.exe b 1 2 | Out-Null
$i++

Write-Host "[${i}/${j}] BENCH NODES GRADUAL NO_SYNTAX_HIGHLIGHT"
.\diagram_pche_egui.exe b 2 0 | Out-Null
$i++

Write-Host "[${i}/${j}] BENCH NODES GRADUAL YES_SYNTAX_HIGHLIGHT"
.\diagram_pche_egui.exe b 2 1 | Out-Null
$i++

Write-Host "[${i}/${j}] BENCH NODES GRADUAL NO_TEXT_EDITOR"
.\diagram_pche_egui.exe b 2 2 | Out-Null
$i++

Write-Host "[${i}/${j}] BENCH WIDGETS"
.\diagram_pche_egui.exe w | Out-Null

Write-Host "====================`nAll benchmarks done.`nPress enter to exit."
Read-Host
