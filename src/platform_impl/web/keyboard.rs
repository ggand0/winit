use smol_str::SmolStr;

use crate::keyboard::{Key, KeyCode, NamedKey, NativeKey, NativeKeyCode, PhysicalKey};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) struct KeyEventExtra;

impl Key {
    pub(crate) fn from_key_attribute_value(kav: &str) -> Self {
        Key::Named(match kav {
            "Unidentified" => return Key::Unidentified(NativeKey::Web(SmolStr::new(kav))),
            "Dead" => return Key::Dead(None),
            "Alt" => NamedKey::Alt,
            "AltGraph" => NamedKey::AltGraph,
            "CapsLock" => NamedKey::CapsLock,
            "Control" => NamedKey::Control,
            "Fn" => NamedKey::Fn,
            "FnLock" => NamedKey::FnLock,
            "NumLock" => NamedKey::NumLock,
            "ScrollLock" => NamedKey::ScrollLock,
            "Shift" => NamedKey::Shift,
            "Symbol" => NamedKey::Symbol,
            "SymbolLock" => NamedKey::SymbolLock,
            "Hyper" => NamedKey::Hyper,
            "Meta" => NamedKey::Super,
            "Enter" => NamedKey::Enter,
            "Tab" => NamedKey::Tab,
            "ArrowDown" => NamedKey::ArrowDown,
            "ArrowLeft" => NamedKey::ArrowLeft,
            "ArrowRight" => NamedKey::ArrowRight,
            "ArrowUp" => NamedKey::ArrowUp,
            "End" => NamedKey::End,
            "Home" => NamedKey::Home,
            "PageDown" => NamedKey::PageDown,
            "PageUp" => NamedKey::PageUp,
            "Backspace" => NamedKey::Backspace,
            "Clear" => NamedKey::Clear,
            "Copy" => NamedKey::Copy,
            "CrSel" => NamedKey::CrSel,
            "Cut" => NamedKey::Cut,
            "Delete" => NamedKey::Delete,
            "EraseEof" => NamedKey::EraseEof,
            "ExSel" => NamedKey::ExSel,
            "Insert" => NamedKey::Insert,
            "Paste" => NamedKey::Paste,
            "Redo" => NamedKey::Redo,
            "Undo" => NamedKey::Undo,
            "Accept" => NamedKey::Accept,
            "Again" => NamedKey::Again,
            "Attn" => NamedKey::Attn,
            "Cancel" => NamedKey::Cancel,
            "ContextMenu" => NamedKey::ContextMenu,
            "Escape" => NamedKey::Escape,
            "Execute" => NamedKey::Execute,
            "Find" => NamedKey::Find,
            "Help" => NamedKey::Help,
            "Pause" => NamedKey::Pause,
            "Play" => NamedKey::Play,
            "Props" => NamedKey::Props,
            "Select" => NamedKey::Select,
            "ZoomIn" => NamedKey::ZoomIn,
            "ZoomOut" => NamedKey::ZoomOut,
            "BrightnessDown" => NamedKey::BrightnessDown,
            "BrightnessUp" => NamedKey::BrightnessUp,
            "Eject" => NamedKey::Eject,
            "LogOff" => NamedKey::LogOff,
            "Power" => NamedKey::Power,
            "PowerOff" => NamedKey::PowerOff,
            "PrintScreen" => NamedKey::PrintScreen,
            "Hibernate" => NamedKey::Hibernate,
            "Standby" => NamedKey::Standby,
            "WakeUp" => NamedKey::WakeUp,
            "AllCandidates" => NamedKey::AllCandidates,
            "Alphanumeric" => NamedKey::Alphanumeric,
            "CodeInput" => NamedKey::CodeInput,
            "Compose" => NamedKey::Compose,
            "Convert" => NamedKey::Convert,
            "FinalMode" => NamedKey::FinalMode,
            "GroupFirst" => NamedKey::GroupFirst,
            "GroupLast" => NamedKey::GroupLast,
            "GroupNext" => NamedKey::GroupNext,
            "GroupPrevious" => NamedKey::GroupPrevious,
            "ModeChange" => NamedKey::ModeChange,
            "NextCandidate" => NamedKey::NextCandidate,
            "NonConvert" => NamedKey::NonConvert,
            "PreviousCandidate" => NamedKey::PreviousCandidate,
            "Process" => NamedKey::Process,
            "SingleCandidate" => NamedKey::SingleCandidate,
            "HangulMode" => NamedKey::HangulMode,
            "HanjaMode" => NamedKey::HanjaMode,
            "JunjaMode" => NamedKey::JunjaMode,
            "Eisu" => NamedKey::Eisu,
            "Hankaku" => NamedKey::Hankaku,
            "Hiragana" => NamedKey::Hiragana,
            "HiraganaKatakana" => NamedKey::HiraganaKatakana,
            "KanaMode" => NamedKey::KanaMode,
            "KanjiMode" => NamedKey::KanjiMode,
            "Katakana" => NamedKey::Katakana,
            "Romaji" => NamedKey::Romaji,
            "Zenkaku" => NamedKey::Zenkaku,
            "ZenkakuHankaku" => NamedKey::ZenkakuHankaku,
            "Soft1" => NamedKey::Soft1,
            "Soft2" => NamedKey::Soft2,
            "Soft3" => NamedKey::Soft3,
            "Soft4" => NamedKey::Soft4,
            "ChannelDown" => NamedKey::ChannelDown,
            "ChannelUp" => NamedKey::ChannelUp,
            "Close" => NamedKey::Close,
            "MailForward" => NamedKey::MailForward,
            "MailReply" => NamedKey::MailReply,
            "MailSend" => NamedKey::MailSend,
            "MediaClose" => NamedKey::MediaClose,
            "MediaFastForward" => NamedKey::MediaFastForward,
            "MediaPause" => NamedKey::MediaPause,
            "MediaPlay" => NamedKey::MediaPlay,
            "MediaPlayPause" => NamedKey::MediaPlayPause,
            "MediaRecord" => NamedKey::MediaRecord,
            "MediaRewind" => NamedKey::MediaRewind,
            "MediaStop" => NamedKey::MediaStop,
            "MediaTrackNext" => NamedKey::MediaTrackNext,
            "MediaTrackPrevious" => NamedKey::MediaTrackPrevious,
            "New" => NamedKey::New,
            "Open" => NamedKey::Open,
            "Print" => NamedKey::Print,
            "Save" => NamedKey::Save,
            "SpellCheck" => NamedKey::SpellCheck,
            "Key11" => NamedKey::Key11,
            "Key12" => NamedKey::Key12,
            "AudioBalanceLeft" => NamedKey::AudioBalanceLeft,
            "AudioBalanceRight" => NamedKey::AudioBalanceRight,
            "AudioBassBoostDown" => NamedKey::AudioBassBoostDown,
            "AudioBassBoostToggle" => NamedKey::AudioBassBoostToggle,
            "AudioBassBoostUp" => NamedKey::AudioBassBoostUp,
            "AudioFaderFront" => NamedKey::AudioFaderFront,
            "AudioFaderRear" => NamedKey::AudioFaderRear,
            "AudioSurroundModeNext" => NamedKey::AudioSurroundModeNext,
            "AudioTrebleDown" => NamedKey::AudioTrebleDown,
            "AudioTrebleUp" => NamedKey::AudioTrebleUp,
            "AudioVolumeDown" => NamedKey::AudioVolumeDown,
            "AudioVolumeUp" => NamedKey::AudioVolumeUp,
            "AudioVolumeMute" => NamedKey::AudioVolumeMute,
            "MicrophoneToggle" => NamedKey::MicrophoneToggle,
            "MicrophoneVolumeDown" => NamedKey::MicrophoneVolumeDown,
            "MicrophoneVolumeUp" => NamedKey::MicrophoneVolumeUp,
            "MicrophoneVolumeMute" => NamedKey::MicrophoneVolumeMute,
            "SpeechCorrectionList" => NamedKey::SpeechCorrectionList,
            "SpeechInputToggle" => NamedKey::SpeechInputToggle,
            "LaunchApplication1" => NamedKey::LaunchApplication1,
            "LaunchApplication2" => NamedKey::LaunchApplication2,
            "LaunchCalendar" => NamedKey::LaunchCalendar,
            "LaunchContacts" => NamedKey::LaunchContacts,
            "LaunchMail" => NamedKey::LaunchMail,
            "LaunchMediaPlayer" => NamedKey::LaunchMediaPlayer,
            "LaunchMusicPlayer" => NamedKey::LaunchMusicPlayer,
            "LaunchPhone" => NamedKey::LaunchPhone,
            "LaunchScreenSaver" => NamedKey::LaunchScreenSaver,
            "LaunchSpreadsheet" => NamedKey::LaunchSpreadsheet,
            "LaunchWebBrowser" => NamedKey::LaunchWebBrowser,
            "LaunchWebCam" => NamedKey::LaunchWebCam,
            "LaunchWordProcessor" => NamedKey::LaunchWordProcessor,
            "BrowserBack" => NamedKey::BrowserBack,
            "BrowserFavorites" => NamedKey::BrowserFavorites,
            "BrowserForward" => NamedKey::BrowserForward,
            "BrowserHome" => NamedKey::BrowserHome,
            "BrowserRefresh" => NamedKey::BrowserRefresh,
            "BrowserSearch" => NamedKey::BrowserSearch,
            "BrowserStop" => NamedKey::BrowserStop,
            "AppSwitch" => NamedKey::AppSwitch,
            "Call" => NamedKey::Call,
            "Camera" => NamedKey::Camera,
            "CameraFocus" => NamedKey::CameraFocus,
            "EndCall" => NamedKey::EndCall,
            "GoBack" => NamedKey::GoBack,
            "GoHome" => NamedKey::GoHome,
            "HeadsetHook" => NamedKey::HeadsetHook,
            "LastNumberRedial" => NamedKey::LastNumberRedial,
            "Notification" => NamedKey::Notification,
            "MannerMode" => NamedKey::MannerMode,
            "VoiceDial" => NamedKey::VoiceDial,
            "TV" => NamedKey::TV,
            "TV3DMode" => NamedKey::TV3DMode,
            "TVAntennaCable" => NamedKey::TVAntennaCable,
            "TVAudioDescription" => NamedKey::TVAudioDescription,
            "TVAudioDescriptionMixDown" => NamedKey::TVAudioDescriptionMixDown,
            "TVAudioDescriptionMixUp" => NamedKey::TVAudioDescriptionMixUp,
            "TVContentsMenu" => NamedKey::TVContentsMenu,
            "TVDataService" => NamedKey::TVDataService,
            "TVInput" => NamedKey::TVInput,
            "TVInputComponent1" => NamedKey::TVInputComponent1,
            "TVInputComponent2" => NamedKey::TVInputComponent2,
            "TVInputComposite1" => NamedKey::TVInputComposite1,
            "TVInputComposite2" => NamedKey::TVInputComposite2,
            "TVInputHDMI1" => NamedKey::TVInputHDMI1,
            "TVInputHDMI2" => NamedKey::TVInputHDMI2,
            "TVInputHDMI3" => NamedKey::TVInputHDMI3,
            "TVInputHDMI4" => NamedKey::TVInputHDMI4,
            "TVInputVGA1" => NamedKey::TVInputVGA1,
            "TVMediaContext" => NamedKey::TVMediaContext,
            "TVNetwork" => NamedKey::TVNetwork,
            "TVNumberEntry" => NamedKey::TVNumberEntry,
            "TVPower" => NamedKey::TVPower,
            "TVRadioService" => NamedKey::TVRadioService,
            "TVSatellite" => NamedKey::TVSatellite,
            "TVSatelliteBS" => NamedKey::TVSatelliteBS,
            "TVSatelliteCS" => NamedKey::TVSatelliteCS,
            "TVSatelliteToggle" => NamedKey::TVSatelliteToggle,
            "TVTerrestrialAnalog" => NamedKey::TVTerrestrialAnalog,
            "TVTerrestrialDigital" => NamedKey::TVTerrestrialDigital,
            "TVTimer" => NamedKey::TVTimer,
            "AVRInput" => NamedKey::AVRInput,
            "AVRPower" => NamedKey::AVRPower,
            "ColorF0Red" => NamedKey::ColorF0Red,
            "ColorF1Green" => NamedKey::ColorF1Green,
            "ColorF2Yellow" => NamedKey::ColorF2Yellow,
            "ColorF3Blue" => NamedKey::ColorF3Blue,
            "ColorF4Grey" => NamedKey::ColorF4Grey,
            "ColorF5Brown" => NamedKey::ColorF5Brown,
            "ClosedCaptionToggle" => NamedKey::ClosedCaptionToggle,
            "Dimmer" => NamedKey::Dimmer,
            "DisplaySwap" => NamedKey::DisplaySwap,
            "DVR" => NamedKey::DVR,
            "Exit" => NamedKey::Exit,
            "FavoriteClear0" => NamedKey::FavoriteClear0,
            "FavoriteClear1" => NamedKey::FavoriteClear1,
            "FavoriteClear2" => NamedKey::FavoriteClear2,
            "FavoriteClear3" => NamedKey::FavoriteClear3,
            "FavoriteRecall0" => NamedKey::FavoriteRecall0,
            "FavoriteRecall1" => NamedKey::FavoriteRecall1,
            "FavoriteRecall2" => NamedKey::FavoriteRecall2,
            "FavoriteRecall3" => NamedKey::FavoriteRecall3,
            "FavoriteStore0" => NamedKey::FavoriteStore0,
            "FavoriteStore1" => NamedKey::FavoriteStore1,
            "FavoriteStore2" => NamedKey::FavoriteStore2,
            "FavoriteStore3" => NamedKey::FavoriteStore3,
            "Guide" => NamedKey::Guide,
            "GuideNextDay" => NamedKey::GuideNextDay,
            "GuidePreviousDay" => NamedKey::GuidePreviousDay,
            "Info" => NamedKey::Info,
            "InstantReplay" => NamedKey::InstantReplay,
            "Link" => NamedKey::Link,
            "ListProgram" => NamedKey::ListProgram,
            "LiveContent" => NamedKey::LiveContent,
            "Lock" => NamedKey::Lock,
            "MediaApps" => NamedKey::MediaApps,
            "MediaAudioTrack" => NamedKey::MediaAudioTrack,
            "MediaLast" => NamedKey::MediaLast,
            "MediaSkipBackward" => NamedKey::MediaSkipBackward,
            "MediaSkipForward" => NamedKey::MediaSkipForward,
            "MediaStepBackward" => NamedKey::MediaStepBackward,
            "MediaStepForward" => NamedKey::MediaStepForward,
            "MediaTopMenu" => NamedKey::MediaTopMenu,
            "NavigateIn" => NamedKey::NavigateIn,
            "NavigateNext" => NamedKey::NavigateNext,
            "NavigateOut" => NamedKey::NavigateOut,
            "NavigatePrevious" => NamedKey::NavigatePrevious,
            "NextFavoriteChannel" => NamedKey::NextFavoriteChannel,
            "NextUserProfile" => NamedKey::NextUserProfile,
            "OnDemand" => NamedKey::OnDemand,
            "Pairing" => NamedKey::Pairing,
            "PinPDown" => NamedKey::PinPDown,
            "PinPMove" => NamedKey::PinPMove,
            "PinPToggle" => NamedKey::PinPToggle,
            "PinPUp" => NamedKey::PinPUp,
            "PlaySpeedDown" => NamedKey::PlaySpeedDown,
            "PlaySpeedReset" => NamedKey::PlaySpeedReset,
            "PlaySpeedUp" => NamedKey::PlaySpeedUp,
            "RandomToggle" => NamedKey::RandomToggle,
            "RcLowBattery" => NamedKey::RcLowBattery,
            "RecordSpeedNext" => NamedKey::RecordSpeedNext,
            "RfBypass" => NamedKey::RfBypass,
            "ScanChannelsToggle" => NamedKey::ScanChannelsToggle,
            "ScreenModeNext" => NamedKey::ScreenModeNext,
            "Settings" => NamedKey::Settings,
            "SplitScreenToggle" => NamedKey::SplitScreenToggle,
            "STBInput" => NamedKey::STBInput,
            "STBPower" => NamedKey::STBPower,
            "Subtitle" => NamedKey::Subtitle,
            "Teletext" => NamedKey::Teletext,
            "VideoModeNext" => NamedKey::VideoModeNext,
            "Wink" => NamedKey::Wink,
            "ZoomToggle" => NamedKey::ZoomToggle,
            "F1" => NamedKey::F1,
            "F2" => NamedKey::F2,
            "F3" => NamedKey::F3,
            "F4" => NamedKey::F4,
            "F5" => NamedKey::F5,
            "F6" => NamedKey::F6,
            "F7" => NamedKey::F7,
            "F8" => NamedKey::F8,
            "F9" => NamedKey::F9,
            "F10" => NamedKey::F10,
            "F11" => NamedKey::F11,
            "F12" => NamedKey::F12,
            "F13" => NamedKey::F13,
            "F14" => NamedKey::F14,
            "F15" => NamedKey::F15,
            "F16" => NamedKey::F16,
            "F17" => NamedKey::F17,
            "F18" => NamedKey::F18,
            "F19" => NamedKey::F19,
            "F20" => NamedKey::F20,
            "F21" => NamedKey::F21,
            "F22" => NamedKey::F22,
            "F23" => NamedKey::F23,
            "F24" => NamedKey::F24,
            "F25" => NamedKey::F25,
            "F26" => NamedKey::F26,
            "F27" => NamedKey::F27,
            "F28" => NamedKey::F28,
            "F29" => NamedKey::F29,
            "F30" => NamedKey::F30,
            "F31" => NamedKey::F31,
            "F32" => NamedKey::F32,
            "F33" => NamedKey::F33,
            "F34" => NamedKey::F34,
            "F35" => NamedKey::F35,
            string => return Key::Character(SmolStr::new(string)),
        })
    }
}

impl PhysicalKey {
    pub fn from_key_code_attribute_value(kcav: &str) -> Self {
        PhysicalKey::Code(match kcav {
            "Backquote" => KeyCode::Backquote,
            "Backslash" => KeyCode::Backslash,
            "BracketLeft" => KeyCode::BracketLeft,
            "BracketRight" => KeyCode::BracketRight,
            "Comma" => KeyCode::Comma,
            "Digit0" => KeyCode::Digit0,
            "Digit1" => KeyCode::Digit1,
            "Digit2" => KeyCode::Digit2,
            "Digit3" => KeyCode::Digit3,
            "Digit4" => KeyCode::Digit4,
            "Digit5" => KeyCode::Digit5,
            "Digit6" => KeyCode::Digit6,
            "Digit7" => KeyCode::Digit7,
            "Digit8" => KeyCode::Digit8,
            "Digit9" => KeyCode::Digit9,
            "Equal" => KeyCode::Equal,
            "IntlBackslash" => KeyCode::IntlBackslash,
            "IntlRo" => KeyCode::IntlRo,
            "IntlYen" => KeyCode::IntlYen,
            "KeyA" => KeyCode::KeyA,
            "KeyB" => KeyCode::KeyB,
            "KeyC" => KeyCode::KeyC,
            "KeyD" => KeyCode::KeyD,
            "KeyE" => KeyCode::KeyE,
            "KeyF" => KeyCode::KeyF,
            "KeyG" => KeyCode::KeyG,
            "KeyH" => KeyCode::KeyH,
            "KeyI" => KeyCode::KeyI,
            "KeyJ" => KeyCode::KeyJ,
            "KeyK" => KeyCode::KeyK,
            "KeyL" => KeyCode::KeyL,
            "KeyM" => KeyCode::KeyM,
            "KeyN" => KeyCode::KeyN,
            "KeyO" => KeyCode::KeyO,
            "KeyP" => KeyCode::KeyP,
            "KeyQ" => KeyCode::KeyQ,
            "KeyR" => KeyCode::KeyR,
            "KeyS" => KeyCode::KeyS,
            "KeyT" => KeyCode::KeyT,
            "KeyU" => KeyCode::KeyU,
            "KeyV" => KeyCode::KeyV,
            "KeyW" => KeyCode::KeyW,
            "KeyX" => KeyCode::KeyX,
            "KeyY" => KeyCode::KeyY,
            "KeyZ" => KeyCode::KeyZ,
            "Minus" => KeyCode::Minus,
            "Period" => KeyCode::Period,
            "Quote" => KeyCode::Quote,
            "Semicolon" => KeyCode::Semicolon,
            "Slash" => KeyCode::Slash,
            "AltLeft" => KeyCode::AltLeft,
            "AltRight" => KeyCode::AltRight,
            "Backspace" => KeyCode::Backspace,
            "CapsLock" => KeyCode::CapsLock,
            "ContextMenu" => KeyCode::ContextMenu,
            "ControlLeft" => KeyCode::ControlLeft,
            "ControlRight" => KeyCode::ControlRight,
            "Enter" => KeyCode::Enter,
            "MetaLeft" => KeyCode::SuperLeft,
            "MetaRight" => KeyCode::SuperRight,
            "ShiftLeft" => KeyCode::ShiftLeft,
            "ShiftRight" => KeyCode::ShiftRight,
            "Space" => KeyCode::Space,
            "Tab" => KeyCode::Tab,
            "Convert" => KeyCode::Convert,
            "KanaMode" => KeyCode::KanaMode,
            "Lang1" => KeyCode::Lang1,
            "Lang2" => KeyCode::Lang2,
            "Lang3" => KeyCode::Lang3,
            "Lang4" => KeyCode::Lang4,
            "Lang5" => KeyCode::Lang5,
            "NonConvert" => KeyCode::NonConvert,
            "Delete" => KeyCode::Delete,
            "End" => KeyCode::End,
            "Help" => KeyCode::Help,
            "Home" => KeyCode::Home,
            "Insert" => KeyCode::Insert,
            "PageDown" => KeyCode::PageDown,
            "PageUp" => KeyCode::PageUp,
            "ArrowDown" => KeyCode::ArrowDown,
            "ArrowLeft" => KeyCode::ArrowLeft,
            "ArrowRight" => KeyCode::ArrowRight,
            "ArrowUp" => KeyCode::ArrowUp,
            "NumLock" => KeyCode::NumLock,
            "Numpad0" => KeyCode::Numpad0,
            "Numpad1" => KeyCode::Numpad1,
            "Numpad2" => KeyCode::Numpad2,
            "Numpad3" => KeyCode::Numpad3,
            "Numpad4" => KeyCode::Numpad4,
            "Numpad5" => KeyCode::Numpad5,
            "Numpad6" => KeyCode::Numpad6,
            "Numpad7" => KeyCode::Numpad7,
            "Numpad8" => KeyCode::Numpad8,
            "Numpad9" => KeyCode::Numpad9,
            "NumpadAdd" => KeyCode::NumpadAdd,
            "NumpadBackspace" => KeyCode::NumpadBackspace,
            "NumpadClear" => KeyCode::NumpadClear,
            "NumpadClearEntry" => KeyCode::NumpadClearEntry,
            "NumpadComma" => KeyCode::NumpadComma,
            "NumpadDecimal" => KeyCode::NumpadDecimal,
            "NumpadDivide" => KeyCode::NumpadDivide,
            "NumpadEnter" => KeyCode::NumpadEnter,
            "NumpadEqual" => KeyCode::NumpadEqual,
            "NumpadHash" => KeyCode::NumpadHash,
            "NumpadMemoryAdd" => KeyCode::NumpadMemoryAdd,
            "NumpadMemoryClear" => KeyCode::NumpadMemoryClear,
            "NumpadMemoryRecall" => KeyCode::NumpadMemoryRecall,
            "NumpadMemoryStore" => KeyCode::NumpadMemoryStore,
            "NumpadMemorySubtract" => KeyCode::NumpadMemorySubtract,
            "NumpadMultiply" => KeyCode::NumpadMultiply,
            "NumpadParenLeft" => KeyCode::NumpadParenLeft,
            "NumpadParenRight" => KeyCode::NumpadParenRight,
            "NumpadStar" => KeyCode::NumpadStar,
            "NumpadSubtract" => KeyCode::NumpadSubtract,
            "Escape" => KeyCode::Escape,
            "Fn" => KeyCode::Fn,
            "FnLock" => KeyCode::FnLock,
            "PrintScreen" => KeyCode::PrintScreen,
            "ScrollLock" => KeyCode::ScrollLock,
            "Pause" => KeyCode::Pause,
            "BrowserBack" => KeyCode::BrowserBack,
            "BrowserFavorites" => KeyCode::BrowserFavorites,
            "BrowserForward" => KeyCode::BrowserForward,
            "BrowserHome" => KeyCode::BrowserHome,
            "BrowserRefresh" => KeyCode::BrowserRefresh,
            "BrowserSearch" => KeyCode::BrowserSearch,
            "BrowserStop" => KeyCode::BrowserStop,
            "Eject" => KeyCode::Eject,
            "LaunchApp1" => KeyCode::LaunchApp1,
            "LaunchApp2" => KeyCode::LaunchApp2,
            "LaunchMail" => KeyCode::LaunchMail,
            "MediaPlayPause" => KeyCode::MediaPlayPause,
            "MediaSelect" => KeyCode::MediaSelect,
            "MediaStop" => KeyCode::MediaStop,
            "MediaTrackNext" => KeyCode::MediaTrackNext,
            "MediaTrackPrevious" => KeyCode::MediaTrackPrevious,
            "Power" => KeyCode::Power,
            "Sleep" => KeyCode::Sleep,
            "AudioVolumeDown" => KeyCode::AudioVolumeDown,
            "AudioVolumeMute" => KeyCode::AudioVolumeMute,
            "AudioVolumeUp" => KeyCode::AudioVolumeUp,
            "WakeUp" => KeyCode::WakeUp,
            "Hyper" => KeyCode::Hyper,
            "Turbo" => KeyCode::Turbo,
            "Abort" => KeyCode::Abort,
            "Resume" => KeyCode::Resume,
            "Suspend" => KeyCode::Suspend,
            "Again" => KeyCode::Again,
            "Copy" => KeyCode::Copy,
            "Cut" => KeyCode::Cut,
            "Find" => KeyCode::Find,
            "Open" => KeyCode::Open,
            "Paste" => KeyCode::Paste,
            "Props" => KeyCode::Props,
            "Select" => KeyCode::Select,
            "Undo" => KeyCode::Undo,
            "Hiragana" => KeyCode::Hiragana,
            "Katakana" => KeyCode::Katakana,
            "F1" => KeyCode::F1,
            "F2" => KeyCode::F2,
            "F3" => KeyCode::F3,
            "F4" => KeyCode::F4,
            "F5" => KeyCode::F5,
            "F6" => KeyCode::F6,
            "F7" => KeyCode::F7,
            "F8" => KeyCode::F8,
            "F9" => KeyCode::F9,
            "F10" => KeyCode::F10,
            "F11" => KeyCode::F11,
            "F12" => KeyCode::F12,
            "F13" => KeyCode::F13,
            "F14" => KeyCode::F14,
            "F15" => KeyCode::F15,
            "F16" => KeyCode::F16,
            "F17" => KeyCode::F17,
            "F18" => KeyCode::F18,
            "F19" => KeyCode::F19,
            "F20" => KeyCode::F20,
            "F21" => KeyCode::F21,
            "F22" => KeyCode::F22,
            "F23" => KeyCode::F23,
            "F24" => KeyCode::F24,
            "F25" => KeyCode::F25,
            "F26" => KeyCode::F26,
            "F27" => KeyCode::F27,
            "F28" => KeyCode::F28,
            "F29" => KeyCode::F29,
            "F30" => KeyCode::F30,
            "F31" => KeyCode::F31,
            "F32" => KeyCode::F32,
            "F33" => KeyCode::F33,
            "F34" => KeyCode::F34,
            "F35" => KeyCode::F35,
            _ => return PhysicalKey::Unidentified(NativeKeyCode::Unidentified),
        })
    }
}
