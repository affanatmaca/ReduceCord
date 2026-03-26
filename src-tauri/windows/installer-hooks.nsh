; Her kurulumda masaüstü kısayolu oluştur (yalnızca bitiş sayfasındaki kutuya güvenilmesin).
!macro NSIS_HOOK_POSTINSTALL
  Call CreateOrUpdateDesktopShortcut
!macroend
