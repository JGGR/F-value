!include MUI2.nsh

;--------------------------------
; Installer metadata
;--------------------------------
Name "F-value"
OutFile "F-value-installer.exe"
InstallDir "$PROGRAMFILES\F_value"
RequestExecutionLevel admin
VIProductVersion "0.0.32.0"
VIAddVersionKey "CompanyName"     "GtoR"

;--------------------------------
; --- UI Pages ---
;--------------------------------
!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_LICENSE "../LICENSE"
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_PAGE_FINISH

!insertmacro MUI_LANGUAGE "Italian"

;--------------------------------
; Default section: files to install
;--------------------------------
Section "Install"

  ; Create installation directory
  CreateDirectory "$INSTDIR"

  SetOutPath "$INSTDIR\templates"
  File /r "..\templates\*"

  ; Copy the binary
  SetOutPath "$INSTDIR"
  File "/oname=F-value.exe" "..\target\x86_64-pc-windows-gnu\release\esox.exe"
  File "..\LICENSE"

  ; Create a desktop shortcut
  CreateShortCut "$DESKTOP\F-value.lnk" "$INSTDIR\F-value.exe"

  ; Write the uninstaller
  WriteUninstaller "$INSTDIR\uninstall.exe"

  ; Add uninstall entry to Windows
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\F-value" "DisplayName" "F-value"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\F-value" "Publisher" "GtoR"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\F-value" "DisplayVersion" "0.0.32"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\F-value" "UninstallString" "$INSTDIR\uninstall.exe"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\F-value" "InstallLocation" "$INSTDIR"

SectionEnd

;--------------------------------
; Uninstaller
;--------------------------------
Section "Uninstall"

  ; Remove files
  Delete "$INSTDIR\F-value.exe"
  ; Delete "$INSTDIR\config\*.*"

  ; Remove uninstaller
  Delete "$INSTDIR\uninstall.exe"

  ; Remove folder
  RMDir /r "$INSTDIR"

  ; Remove uninstall registry entries
  DeleteRegKey HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\F-value"

  ; Remove shortcut
  Delete "$DESKTOP\F-value.lnk"

SectionEnd
