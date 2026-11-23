; Gurista Sov Map — Windows installer (Inno Setup 6). build.ps1 runs this for you.
#define AppName "Gurista Sov Map"
#ifndef AppVersion
  #define AppVersion "0.2.5"
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

[Code]
{ The app runs on Microsoft Edge WebView2, part of Windows 11 and of up-to-date Windows 10.
  If it's missing, fetch Microsoft's small official installer and run it silently. }
const
  WV2 = '{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}';

function HasVer(Root: Integer; Key: String): Boolean;
var
  V: String;
begin
  Result := RegQueryStringValue(Root, Key, 'pv', V) and (V <> '') and (V <> '0.0.0.0');
end;

function WebView2Installed(): Boolean;
begin
  Result := HasVer(HKLM, 'SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\' + WV2) or
            HasVer(HKLM, 'SOFTWARE\Microsoft\EdgeUpdate\Clients\' + WV2) or
            HasVer(HKCU, 'Software\Microsoft\EdgeUpdate\Clients\' + WV2);
end;

function PrepareToInstall(var NeedsRestart: Boolean): String;
var
  Code: Integer;
begin
  Result := '';
  if WebView2Installed() then Exit;
  try
    DownloadTemporaryFile('https://go.microsoft.com/fwlink/p/?LinkId=2124703', 'MicrosoftEdgeWebview2Setup.exe', '', nil);
    if not Exec(ExpandConstant('{tmp}\MicrosoftEdgeWebview2Setup.exe'), '/silent /install', '', SW_HIDE, ewWaitUntilTerminated, Code) then
      Log('WebView2 installer could not start');
  except
    Log('WebView2 download failed: ' + GetExceptionMessage);
  end;
  if not WebView2Installed() then
    MsgBox('Gurista Sov Map needs Microsoft Edge WebView2, and it could not be installed automatically.' + #13#10 +
           'The app will open Microsoft''s download page for it the first time you run it.', mbInformation, MB_OK);
end;
