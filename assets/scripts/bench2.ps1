Write-Host "== DiagramPche_egui benchmarks =="
Write-Host "`nThis script:`n`t* will run the complete 3in1 benchmark with SHON and TXOFF`n`t* expects to be run from the .\assets\scripts folder."
Write-Host "Press enter to continue. Press Ctrl+C to cancel."
Read-Host

cd..
cd..
Set-Location target
Set-Location release

Write-Host "[1/2] BENCH NODES COMPLETE 3in1 YES_SYNTAX_HIGHLIGHT"
.\diagram_pche_egui.exe b 3 1 | Out-Null

Write-Host "[2/2] BENCH NODES COMPLETE 3in1 NO_TEXT_EDITOR"
.\diagram_pche_egui.exe b 3 2 | Out-Null

Write-Host "====================`nAll benchmarks done.`nPress enter to exit."
Read-Host
