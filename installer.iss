; Gurista Sov Map — Windows installer (Inno Setup 6). build.ps1 runs this for you.
#define AppName "Gurista Sov Map"
#ifndef AppVersion
  #define AppVersion "0.2.4"
#endif

[Setup]
AppId={{6E1C2B7A-4F3D-4B9E-9A1E-5C0D9B7F2A11}
AppName={#AppName}
AppVersion={#AppVersion}
AppPublisher=gurista.co.uk
AppPublisherURL=https://gurista.co.uk/sov-map/
AppSupportURL=https://gurista.co.uk/sov-map/
DefaultDirName={autopf}\Gurista Sov Map
DefaultGroupName=Gurista Sov Map
DisableProgramGroupPage=yes
; Installs for the current user without admin rights (users can choose all users).
PrivilegesRequired=lowest
PrivilegesRequiredOverridesAllowed=dialog
OutputDir=dist
OutputBaseFilename=GuristaSovMap-Setup-{#AppVersion}
SetupIconFile=assets\icon.ico
UninstallDisplayIcon={app}\GuristaSovMap.exe
Compression=lzma2/max
SolidCompression=yes
WizardStyle=modern
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible

[Tasks]
Name: "desktopicon"; Description: "Create a &desktop shortcut"; GroupDescription: "Shortcuts:"

[Files]
Source: "target\release\sovmap.exe"; DestDir: "{app}"; DestName: "GuristaSovMap.exe"; Flags: ignoreversion
Source: "README.md"; DestDir: "{app}"; Flags: ignoreversion
Source: "assets\fonts\*"; DestDir: "{app}\assets\fonts"; Flags: ignoreversion recursesubdirs skipifsourcedoesntexist

[Icons]
Name: "{autoprograms}\Gurista Sov Map"; Filename: "{app}\GuristaSovMap.exe"
Name: "{autodesktop}\Gurista Sov Map"; Filename: "{app}\GuristaSovMap.exe"; Tasks: desktopicon

[Run]
Filename: "{app}\GuristaSovMap.exe"; Description: "Launch Gurista Sov Map"; Flags: nowait postinstall skipifsilent
