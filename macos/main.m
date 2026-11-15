#import <AppKit/AppKit.h>
#import <AVFoundation/AVFoundation.h>
#import <Carbon/Carbon.h>
#import <ServiceManagement/ServiceManagement.h>

#import "ClockView.h"

static NSString *const SavedClockKey = @"SavedClock";
static const NSSize ClockSize = {400, 575};
static const OSType HotkeySignature = 0x4B4F4F4B; // KOOK
static const UInt32 LetterKeyCodes[] = {
    kVK_ANSI_A, kVK_ANSI_B, kVK_ANSI_C, kVK_ANSI_D, kVK_ANSI_E, kVK_ANSI_F, kVK_ANSI_G,
    kVK_ANSI_H, kVK_ANSI_I, kVK_ANSI_J, kVK_ANSI_K, kVK_ANSI_L, kVK_ANSI_M, kVK_ANSI_N,
    kVK_ANSI_O, kVK_ANSI_P, kVK_ANSI_Q, kVK_ANSI_R, kVK_ANSI_S, kVK_ANSI_T, kVK_ANSI_U,
    kVK_ANSI_V, kVK_ANSI_W, kVK_ANSI_X, kVK_ANSI_Y, kVK_ANSI_Z,
};

@interface ClockPanel : NSPanel
@end
@implementation ClockPanel
- (BOOL)canBecomeKeyWindow { return NO; }
- (BOOL)canBecomeMainWindow { return NO; }
@end

@interface ClockApp : NSObject <NSApplicationDelegate, NSMenuDelegate, NSTextFieldDelegate>
@property(nonatomic) BOOL smokeTest;
- (void)showClock:(id)sender;
- (BOOL)handleHotkey:(EventHotKeyID)identifier;
@end

static OSStatus HotkeyPressed(EventHandlerCallRef next, EventRef event, void *context) {
    EventHotKeyID identifier;
    if (GetEventParameter(event, kEventParamDirectObject, typeEventHotKeyID, NULL,
                          sizeof(identifier), NULL, &identifier) != noErr) {
        return eventNotHandledErr;
    }
    return [(__bridge ClockApp *)context handleHotkey:identifier] ? noErr : eventNotHandledErr;
}

static NSMenuItem *MenuItem(NSMenu *menu, NSString *title, id target, SEL action, NSInteger tag) {
    NSMenuItem *item = [menu addItemWithTitle:title action:action keyEquivalent:@""];
    item.target = target;
    item.tag = tag;
    return item;
}

static NSTextField *Label(NSString *text) {
    return [NSTextField labelWithString:text];
}

static NSStackView *Stack(NSArray<NSView *> *views, NSUserInterfaceLayoutOrientation orientation) {
    NSStackView *stack = [NSStackView stackViewWithViews:views];
    stack.orientation = orientation;
    stack.alignment = orientation == NSUserInterfaceLayoutOrientationVertical
        ? NSLayoutAttributeLeading : NSLayoutAttributeCenterY;
    stack.spacing = 9;
    return stack;
}

static NSWindow *ControlsWindow(NSString *title, NSView *view) {
    NSSize size = view.fittingSize;
    NSWindow *window = [[NSWindow alloc] initWithContentRect:NSMakeRect(0, 0, size.width, size.height)
        styleMask:NSWindowStyleMaskTitled | NSWindowStyleMaskClosable
        backing:NSBackingStoreBuffered defer:NO];
    window.releasedWhenClosed = NO;
    window.title = title;
    window.contentView = view;
    [window center];
    return window;
}

static NSStackView *WindowContent(NSArray<NSView *> *views) {
    NSStackView *stack = Stack(views, NSUserInterfaceLayoutOrientationVertical);
    stack.edgeInsets = NSEdgeInsetsMake(20, 22, 20, 22);
    return stack;
}

@implementation ClockApp {
    KCClock *_clock;
    KCFrame _frame;
    NSDictionary *_settings;
    NSArray<NSArray<NSString *> *> *_cast;
    ClockPanel *_panel;
    ClockView *_view;
    NSStatusItem *_status;
    NSTimer *_timer;
    AVAudioPlayer *_player;
    NSWindow *_settingsWindow;
    NSWindow *_galleryWindow;
    NSPopUpButton *_friends;
    NSImageView *_portrait;
    NSTextField *_performance;
    NSPopUpButton *_shortcutLetter;
    NSButton *_shortcutShift;
    NSTextField *_shortcutStatus;
    NSButton *_loginItem;
    NSTextField *_loginStatus;
    EventHotKeyRef _hotkey;
    EventHandlerRef _hotkeyHandler;
    UInt32 _hotkeyIdentifier;
    int _activeLetter;
    BOOL _activeShift;
    uint64_t _visit;
    uint64_t _savedRevision;
    BOOL _manuallyFront;
    BOOL _scheduledFront;
    BOOL _suspended;
    BOOL _smokeSawVisitor;
    BOOL _smokeSawReturn;
    BOOL _smokeDecodedAudio;
    BOOL _smokePlayedAudio;
    BOOL _smokeSawOpenDoor;
    BOOL _smokeSawClosedDoor;
    NSTimeInterval _smokeStart;
}

- (NSData *)clockJSON:(int)kind {
    size_t length;
    const uint8_t *bytes = kc_json(_clock, kind, &length);
    return [NSData dataWithBytes:bytes length:length];
}

- (id)clockObject:(int)kind {
    return [NSJSONSerialization JSONObjectWithData:[self clockJSON:kind] options:0 error:NULL];
}

- (void)saveClock {
    if (!self.smokeTest) {
        [NSUserDefaults.standardUserDefaults setObject:[self clockJSON:KCSaved]
                                                 forKey:SavedClockKey];
    }
}

- (void)update:(NSDictionary *)changes {
    NSMutableDictionary *values = [_settings mutableCopy];
    [values addEntriesFromDictionary:changes];
    NSData *data = [NSJSONSerialization dataWithJSONObject:values options:0 error:NULL];
    if (data.length && kc_settings(_clock, data.bytes, data.length)) {
        _settings = [self clockObject:KCSettings];
        _view.settings = _settings;
        _view.needsDisplay = YES;
        [self saveClock];
    }
}

- (NSMenu *)makeControlsMenu {
    NSMenu *menu = [NSMenu new];
    menu.delegate = self;
    MenuItem(menu, @"Bring Clock Forward", self, @selector(showClock:), 0);
    MenuItem(menu, @"Send Behind Windows", self, @selector(sendBehind:), 0);
    [menu addItem:NSMenuItem.separatorItem];
    NSMenu *sound = [[NSMenu alloc] initWithTitle:@"Sound"];
    NSArray *modes = @[@"Character Calls", @"Zen Bells", @"Silent"];
    for (NSInteger i = 0; i < (NSInteger)modes.count; ++i) {
        MenuItem(sound, modes[i], self, @selector(changeSound:), i);
    }
    [sound addItem:NSMenuItem.separatorItem];
    MenuItem(sound, @"Snooze Sound for 1 Hour", self, @selector(snooze:), 3600);
    MenuItem(sound, @"Resume Sound", self, @selector(snooze:), 0);
    [menu setSubmenu:sound forItem:MenuItem(menu, @"Sound", nil, NULL, 0)];
    NSMenu *visit = [[NSMenu alloc] initWithTitle:@"Visit"];
    MenuItem(visit, @"Hourly Visitor", self, @selector(preview:), 0);
    MenuItem(visit, @"Half-Hour Visitor", self, @selector(preview:), 1);
    MenuItem(visit, @"Meet the Visitors…", self, @selector(showGallery:), 0);
    MenuItem(visit, @"Stop Visit", self, @selector(stopVisit:), 0);
    [menu setSubmenu:visit forItem:MenuItem(menu, @"Visit", nil, NULL, 0)];
    MenuItem(menu, @"Settings…", self, @selector(showSettings:), 0);
    [menu addItem:NSMenuItem.separatorItem];
    MenuItem(menu, @"Quit KookieClock", NSApp, @selector(terminate:), 0);
    return menu;
}

- (void)menuNeedsUpdate:(NSMenu *)menu {
    NSMenu *sound = [menu itemWithTitle:@"Sound"].submenu;
    NSArray *modes = @[@"Cuckoo", @"Zen", @"Silent"];
    for (NSInteger i = 0; i < (NSInteger)modes.count; ++i) {
        [sound itemAtIndex:i].state = [_settings[@"mode"] isEqual:modes[i]]
            ? NSControlStateValueOn : NSControlStateValueOff;
    }
}

- (void)resizeClockResetPosition:(BOOL)reset {
    NSRect screen = (_panel.screen ?: NSScreen.mainScreen).visibleFrame;
    if (NSIsEmptyRect(screen)) return;
    CGFloat factor = ((CGFloat[]){.65, .85, 1, 1.2})[[_settings[@"size"] unsignedIntegerValue]];
    factor = MIN(factor, MIN(screen.size.width / ClockSize.width, screen.size.height / ClockSize.height));
    NSRect frame = _panel.frame;
    CGFloat top = NSMaxY(frame);
    frame.size = NSMakeSize(ClockSize.width * factor, ClockSize.height * factor);
    if (reset) {
        frame.origin.x = NSMaxX(screen) - frame.size.width - 20;
        top = NSMaxY(screen) - 20;
    }
    frame.origin.x = MAX(NSMinX(screen), MIN(frame.origin.x, NSMaxX(screen) - frame.size.width));
    frame.origin.y = MAX(NSMinY(screen), MIN(top, NSMaxY(screen)) - frame.size.height);
    [_panel setFrame:frame display:YES];
}

- (void)applicationDidFinishLaunching:(NSNotification *)notification {
    NSData *saved = self.smokeTest ? nil
        : [NSUserDefaults.standardUserDefaults dataForKey:SavedClockKey];
    _clock = kc_create(saved.bytes, saved.length);
    _settings = [self clockObject:KCSettings];
    _cast = [self clockObject:KCCast];
    _panel = [[ClockPanel alloc] initWithContentRect:NSMakeRect(0, 0, ClockSize.width, ClockSize.height)
        styleMask:NSWindowStyleMaskBorderless | NSWindowStyleMaskNonactivatingPanel
        backing:NSBackingStoreBuffered defer:NO];
    _panel.title = @"KookieClock";
    _panel.opaque = NO;
    _panel.backgroundColor = NSColor.clearColor;
    _panel.hasShadow = NO;
    _panel.hidesOnDeactivate = NO;
    _panel.becomesKeyOnlyIfNeeded = YES;
    _panel.animationBehavior = NSWindowAnimationBehaviorNone;
    _panel.collectionBehavior = NSWindowCollectionBehaviorCanJoinAllSpaces
        | NSWindowCollectionBehaviorStationary | NSWindowCollectionBehaviorFullScreenAuxiliary
        | NSWindowCollectionBehaviorIgnoresCycle;
    _view = [[ClockView alloc] initWithFrame:_panel.contentView.bounds];
    _view.settings = _settings;
    _view.menu = [self makeControlsMenu];
    _panel.contentView = _view;
    BOOL restored = !self.smokeTest && [_panel setFrameUsingName:@"Clock"];
    [self resizeClockResetPosition:!restored];
    if (!self.smokeTest) [_panel setFrameAutosaveName:@"Clock"];
    [self sendBehind:nil];

    _status = [NSStatusBar.systemStatusBar statusItemWithLength:NSVariableStatusItemLength];
    _status.button.image = [NSImage imageWithSystemSymbolName:@"clock" accessibilityDescription:@"KookieClock"];
    _status.button.toolTip = @"KookieClock";
    _status.menu = [self makeControlsMenu];
    [self installHotkey];
    NSNotificationCenter *center = NSWorkspace.sharedWorkspace.notificationCenter;
    [center addObserver:self selector:@selector(sleep:) name:NSWorkspaceWillSleepNotification object:nil];
    [center addObserver:self selector:@selector(wake:) name:NSWorkspaceDidWakeNotification object:nil];
    [center addObserver:self selector:@selector(sleep:) name:NSWorkspaceSessionDidResignActiveNotification object:nil];
    [center addObserver:self selector:@selector(wake:) name:NSWorkspaceSessionDidBecomeActiveNotification object:nil];

    _timer = [NSTimer timerWithTimeInterval:1.0 / 30 target:self
                                  selector:@selector(tick:) userInfo:nil repeats:YES];
    _timer.tolerance = .004;
    [NSRunLoop.mainRunLoop addTimer:_timer forMode:NSRunLoopCommonModes];
    [self tick:nil];
    if (self.smokeTest) [self startSmokeTest];
}

- (NSString *)shortcutLabel {
    int letter = [_settings[@"hotkey_letter"] intValue];
    BOOL shift = [_settings[@"hotkey_shift"] boolValue];
    return [NSString stringWithFormat:@"Control–Option–%@%c", shift ? @"Shift–" : @"", letter];
}

- (void)installHotkey {
    EventTypeSpec event = {kEventClassKeyboard, kEventHotKeyPressed};
    InstallEventHandler(GetApplicationEventTarget(), HotkeyPressed, 1, &event,
                        (__bridge void *)self, &_hotkeyHandler);
    [self registerHotkeyLetter:[_settings[@"hotkey_letter"] intValue]
                         shift:[_settings[@"hotkey_shift"] boolValue]];
    [self refreshShortcutStatus];
}

- (BOOL)registerHotkeyLetter:(int)letter shift:(BOOL)shift {
    if (!_hotkeyHandler || letter < 'A' || letter > 'Z') return NO;
    if (_hotkey && letter == _activeLetter && shift == _activeShift) return YES;
    EventHotKeyID identifier = {HotkeySignature, _hotkeyIdentifier + 1};
    EventHotKeyRef candidate = NULL;
    OSStatus result = RegisterEventHotKey(LetterKeyCodes[letter - 'A'],
        controlKey | optionKey | (shift ? shiftKey : 0), identifier,
        GetApplicationEventTarget(), kEventHotKeyExclusive, &candidate);
    if (result != noErr) return NO;
    if (_hotkey) UnregisterEventHotKey(_hotkey);
    _hotkey = candidate;
    _hotkeyIdentifier = identifier.id;
    _activeLetter = letter;
    _activeShift = shift;
    return YES;
}

- (BOOL)handleHotkey:(EventHotKeyID)identifier {
    if (identifier.signature != HotkeySignature || identifier.id != _hotkeyIdentifier || !_hotkey) {
        return NO;
    }
    [self showClock:nil];
    return YES;
}

- (void)refreshShortcutStatus {
    NSString *shortcut = _hotkey ? [self shortcutLabel] : @"Shortcut unavailable. Choose another letter.";
    _shortcutStatus.stringValue = shortcut;
    _status.button.toolTip = [NSString stringWithFormat:@"KookieClock · %@", shortcut];
}

- (BOOL)applyShortcutSelection {
    int letter = 'A' + (int)_shortcutLetter.indexOfSelectedItem;
    BOOL shift = _shortcutShift.state == NSControlStateValueOn;
    if (![self registerHotkeyLetter:letter shift:shift]) {
        [_shortcutLetter selectItemAtIndex:[_settings[@"hotkey_letter"] intValue] - 'A'];
        _shortcutShift.state = [_settings[@"hotkey_shift"] boolValue]
            ? NSControlStateValueOn : NSControlStateValueOff;
        _shortcutStatus.stringValue = _hotkey
            ? [NSString stringWithFormat:@"Unavailable. Still using %@.", [self shortcutLabel]]
            : @"Shortcut unavailable. Choose another letter.";
        return NO;
    }
    [self update:@{@"hotkey_letter": @(letter), @"hotkey_shift": @(shift)}];
    [self refreshShortcutStatus];
    return YES;
}

- (void)changeShortcut:(id)sender {
    [self applyShortcutSelection];
}

- (void)refreshLoginItem {
    if (!_loginItem) return;
    switch (SMAppService.mainApp.status) {
        case SMAppServiceStatusEnabled:
            _loginItem.state = NSControlStateValueOn;
            _loginStatus.stringValue = @"Enabled";
            break;
        case SMAppServiceStatusRequiresApproval:
            _loginItem.state = NSControlStateValueOn;
            _loginStatus.stringValue = @"Approval needed in Login Items";
            break;
        case SMAppServiceStatusNotFound:
            _loginItem.state = NSControlStateValueOff;
            _loginStatus.stringValue = @"Unavailable from this location";
            break;
        default:
            _loginItem.state = NSControlStateValueOff;
            _loginStatus.stringValue = @"Off";
            break;
    }
}

- (void)changeLoginItem:(NSButton *)sender {
    SMAppService *service = SMAppService.mainApp;
    BOOL enabled = sender.state == NSControlStateValueOn;
    NSError *error = nil;
    BOOL changed = enabled ? [service registerAndReturnError:&error]
                           : [service unregisterAndReturnError:&error];
    [self refreshLoginItem];
    if (!changed && !(enabled && service.status == SMAppServiceStatusRequiresApproval)) {
        _loginStatus.stringValue = @"Could not change login item";
        NSAlert *alert = error ? [NSAlert alertWithError:error] : [NSAlert new];
        alert.messageText = @"KookieClock could not change launch at login.";
        [alert beginSheetModalForWindow:_settingsWindow completionHandler:nil];
    }
}

- (void)openLoginItems:(id)sender {
    [SMAppService openSystemSettingsLoginItems];
}

- (void)applicationDidBecomeActive:(NSNotification *)notification {
    [self refreshLoginItem];
}

- (void)applyPlacement {
    _panel.level = (_scheduledFront || _manuallyFront)
        ? NSFloatingWindowLevel : CGWindowLevelForKey(kCGDesktopIconWindowLevelKey) + 1;
    [_panel orderFrontRegardless];
}

- (void)showClock:(id)sender {
    _manuallyFront = YES;
    [self applyPlacement];
}

- (void)sendBehind:(id)sender {
    _manuallyFront = NO;
    _scheduledFront = NO;
    [self applyPlacement];
}

- (void)stopVisit:(id)sender {
    kc_stop(_clock);
    [self finishAudio];
    _visit = 0;
    _frame.generation = 0;
    _frame.visitor = -1;
    _frame.door = 0;
    _view.frameState = _frame;
    _view.needsDisplay = YES;
}

- (void)finishAudio {
    [_player stop];
    _player = nil;
    if (_scheduledFront) {
        _scheduledFront = NO;
        [self applyPlacement];
    }
}

- (void)sleep:(NSNotification *)note {
    _suspended = YES;
    [self stopVisit:nil];
}

- (void)wake:(NSNotification *)note {
    kc_resume(_clock);
    _suspended = NO;
}

- (void)tick:(NSTimer *)timer {
    if (_suspended) return;
    int lastSecond = _frame.second;
    kc_tick(_clock, &_frame, _player.isPlaying ? _player.currentTime : -1);
    if (_frame.revision != _savedRevision) {
        _savedRevision = _frame.revision;
        [self saveClock];
    }
    BOOL changed = _visit != _frame.generation;
    if (changed) {
        [self finishAudio];
        _visit = _frame.generation;
        if (_visit) {
            [_view prepareVisit:_frame.character];
            if (_frame.scheduled) {
                _scheduledFront = YES;
                [self applyPlacement];
            }
            size_t length;
            const uint8_t *wave = kc_audio(_clock, &length);
            if (length) {
                NSError *error = nil;
                NSData *data = [NSData dataWithBytes:wave length:length];
                _player = [[AVAudioPlayer alloc] initWithData:data error:&error];
                BOOL prepared = [_player prepareToPlay];
                BOOL played = [_player play];
                if (self.smokeTest) {
                    _smokeDecodedAudio = _player.duration > 12;
                    _smokePlayedAudio = prepared && played;
                }
                if (!played) NSLog(@"KookieClock sound playback unavailable: %@", error ?: @"no output");
            }
            kc_started(_clock);
        }
    }
    if (changed || _visit || lastSecond != _frame.second || [_settings[@"motion"] boolValue]) {
        _view.frameState = _frame;
        _view.needsDisplay = YES;
    }
    if (self.smokeTest && _smokeStart) [self checkSmokeTest];
}

- (void)preview:(NSMenuItem *)item {
    [self showClock:nil];
    kc_visit(_clock, -1, item.tag == 1, false);
    [self tick:nil];
}

- (void)changeSound:(NSMenuItem *)item {
    [self stopVisit:nil];
    [self update:@{@"mode": (@[@"Cuckoo", @"Zen", @"Silent"])[item.tag]}];
    [self refreshSettingsWindow];
}

- (void)snooze:(NSMenuItem *)item {
    [self stopVisit:nil];
    long long until = item.tag ? (long long)NSDate.date.timeIntervalSince1970 + item.tag : 0;
    [self update:@{@"snooze_until": @(until)}];
}

- (NSButton *)check:(NSString *)title setting:(NSString *)key {
    NSButton *button = [NSButton checkboxWithTitle:title target:self action:@selector(saveCheck:)];
    button.identifier = key;
    button.state = [_settings[key] boolValue] ? NSControlStateValueOn : NSControlStateValueOff;
    return button;
}

- (NSPopUpButton *)picker:(NSArray<NSString *> *)titles setting:(NSString *)key index:(NSInteger)index {
    NSPopUpButton *picker = [[NSPopUpButton alloc] initWithFrame:NSZeroRect pullsDown:NO];
    [picker addItemsWithTitles:titles];
    [picker selectItemAtIndex:index];
    picker.identifier = key;
    picker.target = self;
    picker.action = @selector(savePicker:);
    return picker;
}

- (void)saveCheck:(NSButton *)button {
    [self update:@{button.identifier: @(button.state == NSControlStateValueOn)}];
}

- (void)savePicker:(NSPopUpButton *)picker {
    NSString *key = picker.identifier;
    NSInteger index = picker.indexOfSelectedItem;
    if (index < 0) return;
    if ([key isEqual:@"mode"]) {
        [self stopVisit:nil];
        [self update:@{key: (@[@"Cuckoo", @"Zen", @"Silent"])[index]}];
    } else {
        [self update:@{key: @(index)}];
    }
    if ([key isEqual:@"size"]) [self resizeClockResetPosition:NO];
}

- (void)saveVolume:(NSSlider *)slider {
    if (slider.integerValue == 0) [self stopVisit:nil];
    [self update:@{@"volume": @(slider.integerValue)}];
}

- (NSTextField *)quietTime:(NSString *)key label:(NSString *)label {
    NSInteger minute = [_settings[key] integerValue];
    NSTextField *field = [NSTextField textFieldWithString:
        [NSString stringWithFormat:@"%02ld:%02ld", minute / 60, minute % 60]];
    field.identifier = key;
    field.accessibilityLabel = label;
    field.delegate = self;
    [field.widthAnchor constraintEqualToConstant:65].active = YES;
    return field;
}

- (void)controlTextDidEndEditing:(NSNotification *)note {
    NSTextField *field = note.object;
    NSString *text = field.stringValue;
    NSInteger minute = [_settings[field.identifier] integerValue];
    if ([text rangeOfString:@"^[0-9]{2}:[0-9]{2}$" options:NSRegularExpressionSearch].location != NSNotFound) {
        NSInteger h = [[text substringToIndex:2] integerValue];
        NSInteger m = [[text substringFromIndex:3] integerValue];
        if (h < 24 && m < 60) {
            minute = h * 60 + m;
            [self update:@{field.identifier: @(minute)}];
        } else {
            NSBeep();
        }
    } else {
        NSBeep();
    }
    field.stringValue = [NSString stringWithFormat:@"%02ld:%02ld", minute / 60, minute % 60];
}

- (void)resetPosition:(id)sender {
    [self resizeClockResetPosition:YES];
}

- (void)applicationDidChangeScreenParameters:(NSNotification *)notification {
    [self resizeClockResetPosition:NO];
}

- (NSStackView *)settingsContent {
    NSArray *modes = @[@"Cuckoo", @"Zen", @"Silent"];
    NSInteger mode = [modes indexOfObject:_settings[@"mode"]];
    NSSlider *volume = [NSSlider sliderWithValue:[_settings[@"volume"] doubleValue]
        minValue:0 maxValue:100 target:self action:@selector(saveVolume:)];
    volume.continuous = NO;
    volume.accessibilityLabel = @"Volume";
    [volume.widthAnchor constraintGreaterThanOrEqualToConstant:190].active = YES;
    NSGridView *form = [NSGridView gridViewWithViews:@[
        @[Label(@"Sound"), [self picker:@[@"Character calls", @"Zen bells", @"Silent"] setting:@"mode" index:mode]],
        @[Label(@"Cuckoo voice"), [self picker:@[@"Individual voices", @"Classic cuckoo",
           @"Soft wooden cuckoo", @"Forest whistle"] setting:@"voice" index:[_settings[@"voice"] integerValue]]],
        @[Label(@"Zen bell"), [self picker:@[@"Warm singing bowl", @"Bright singing bowl",
           @"Temple bell", @"Variety"] setting:@"bell" index:[_settings[@"bell"] integerValue]]],
        @[Label(@"Volume"), volume],
        @[Label(@"Quiet time"), Stack(@[[self quietTime:@"quiet_start" label:@"Quiet start, 24-hour time"],
           Label(@"to"), [self quietTime:@"quiet_end" label:@"Quiet end, 24-hour time"],
           Label(@"24-hour time")], NSUserInterfaceLayoutOrientationHorizontal)],
        @[Label(@"Clock size"), [self picker:@[@"Small", @"Medium", @"Large", @"Extra large"]
           setting:@"size" index:[_settings[@"size"] integerValue]]],
    ]];
    form.rowSpacing = 7;
    form.columnSpacing = 16;
    form.xPlacement = NSGridCellPlacementLeading;
    NSStackView *sound = Stack(@[
        [self check:@"Hourly visits" setting:@"hourly"],
        [self check:@"Half-hour visits" setting:@"half_hourly"],
        [self check:@"Chime the hour count" setting:@"count_hour"],
        [self check:@"Use quiet hours" setting:@"quiet"],
        [self check:@"Skip visits during quiet hours" setting:@"quiet_motion"],
    ], NSUserInterfaceLayoutOrientationVertical);
    NSStackView *appearance = Stack(@[
        [self check:@"Animate clock and visitors" setting:@"motion"],
        [self check:@"Show second hand" setting:@"seconds"],
        [self check:@"Show digital time" setting:@"digital"],
        [self check:@"Use 24-hour digital time" setting:@"clock_24h"],
        [self check:@"Lock clock position" setting:@"locked"],
    ], NSUserInterfaceLayoutOrientationVertical);
    NSStackView *columns = Stack(@[sound, appearance], NSUserInterfaceLayoutOrientationHorizontal);
    columns.alignment = NSLayoutAttributeTop;
    columns.spacing = 25;
    NSButton *reset = [NSButton buttonWithTitle:@"Reset Position" target:self action:@selector(resetPosition:)];
    _shortcutLetter = [[NSPopUpButton alloc] initWithFrame:NSZeroRect pullsDown:NO];
    for (int letter = 'A'; letter <= 'Z'; ++letter) {
        [_shortcutLetter addItemWithTitle:[NSString stringWithFormat:@"%c", letter]];
    }
    [_shortcutLetter selectItemAtIndex:[_settings[@"hotkey_letter"] intValue] - 'A'];
    _shortcutLetter.accessibilityLabel = @"Bring clock forward shortcut letter";
    _shortcutLetter.target = self;
    _shortcutLetter.action = @selector(changeShortcut:);
    _shortcutShift = [NSButton checkboxWithTitle:@"Add Shift" target:self action:@selector(changeShortcut:)];
    _shortcutShift.state = [_settings[@"hotkey_shift"] boolValue] ? NSControlStateValueOn : NSControlStateValueOff;
    _shortcutStatus = Label(@"");
    [self refreshShortcutStatus];
    NSStackView *shortcut = Stack(@[Label(@"Bring Forward"), Label(@"Control–Option"),
        _shortcutLetter, _shortcutShift], NSUserInterfaceLayoutOrientationHorizontal);
    _loginItem = [NSButton checkboxWithTitle:@"Launch at Login" target:self action:@selector(changeLoginItem:)];
    _loginStatus = Label(@"");
    [self refreshLoginItem];
    NSButton *loginSettings = [NSButton buttonWithTitle:@"Login Items…" target:self action:@selector(openLoginItems:)];
    NSStackView *login = Stack(@[_loginItem, loginSettings, _loginStatus], NSUserInterfaceLayoutOrientationHorizontal);
    return WindowContent(@[form, columns, reset, shortcut, _shortcutStatus, login]);
}

- (void)refreshSettingsWindow {
    if (_settingsWindow.isVisible) _settingsWindow.contentView = [self settingsContent];
}

- (void)showSettings:(id)sender {
    if (!_settingsWindow.isVisible) {
        _settingsWindow = ControlsWindow(@"KookieClock Settings", [self settingsContent]);
    }
    [NSApp activateIgnoringOtherApps:YES];
    [self refreshLoginItem];
    [_settingsWindow makeKeyAndOrderFront:nil];
}

- (void)selectFriend:(id)sender {
    NSInteger index = _friends.indexOfSelectedItem;
    if (index < 0) return;
    _portrait.image = [_view art:[NSString stringWithFormat:@"visitor-%02ld", index]];
    _portrait.accessibilityLabel = _friends.titleOfSelectedItem;
    _performance.stringValue = _cast[index][2];
}

- (void)hearFriend:(id)sender {
    NSInteger index = _friends.indexOfSelectedItem;
    if (index < 0) return;
    [self showClock:nil];
    kc_visit(_clock, (int32_t)index, false, false);
    [self tick:nil];
}

- (void)showGallery:(id)sender {
    if (!_galleryWindow) {
        _friends = [[NSPopUpButton alloc] initWithFrame:NSZeroRect pullsDown:NO];
        for (NSUInteger i = 0; i < _cast.count; ++i) {
            NSArray *friend = _cast[i];
            [_friends addItemWithTitle:[NSString stringWithFormat:@"%@ · %@ · %@",
                i < 12 ? @"Hour" : @"Half-hour", friend[0], friend[1]]];
        }
        _friends.target = self;
        _friends.action = @selector(selectFriend:);
        _friends.accessibilityLabel = @"Visitor";
        _portrait = [[NSImageView alloc] initWithFrame:NSZeroRect];
        _portrait.imageScaling = NSImageScaleProportionallyUpOrDown;
        [_portrait.widthAnchor constraintEqualToConstant:192].active = YES;
        [_portrait.heightAnchor constraintEqualToConstant:192].active = YES;
        _performance = Label(@"");
        NSButton *hear = [NSButton buttonWithTitle:@"Hear and Visit" target:self action:@selector(hearFriend:)];
        NSButton *stop = [NSButton buttonWithTitle:@"Stop" target:self action:@selector(stopVisit:)];
        NSStackView *buttons = Stack(@[hear, stop], NSUserInterfaceLayoutOrientationHorizontal);
        _galleryWindow = ControlsWindow(@"Meet the Visitors", WindowContent(@[_friends, _portrait,
            _performance, buttons]));
        [self selectFriend:nil];
    }
    [NSApp activateIgnoringOtherApps:YES];
    [_galleryWindow makeKeyAndOrderFront:nil];
}

- (void)failSmoke:(NSString *)reason {
    NSLog(@"KookieClock smoke test failed: %@", reason);
    exit(1);
}

- (BOOL)captureSmoke:(NSString *)filename {
    NSBitmapImageRep *rep = [_view bitmapImageRepForCachingDisplayInRect:_view.bounds];
    if (!rep) return NO;
    [_view cacheDisplayInRect:_view.bounds toBitmapImageRep:rep];
    NSColor *center = [rep colorAtX:rep.pixelsWide / 2 y:rep.pixelsHigh / 2];
    if (!center || center.alphaComponent < .9) return NO;
    NSData *png = [rep representationUsingType:NSBitmapImageFileTypePNG properties:@{}];
    return [png writeToFile:filename atomically:YES];
}

- (void)startSmokeTest {
    if (_cast.count != 24 || ![_view art:@"house"] || ![_view art:@"pendulum"]) {
        [self failSmoke:@"missing cast or clock assets"];
    }
    for (int i = 0; i < 24; ++i) {
        NSImage *atlas = [_view art:[NSString stringWithFormat:@"visitor-atlas-%02d", i]];
        CGImageRef image = [atlas CGImageForProposedRect:NULL context:nil hints:nil];
        if (!image || CGImageGetWidth(image) != 960 || CGImageGetHeight(image) != 960 ||
            ![_view art:[NSString stringWithFormat:@"visitor-%02d", i]]) {
            [self failSmoke:@"missing or malformed visitor assets"];
        }
    }
    for (int i = 0; i <= 16; ++i) {
        if (![_view art:[NSString stringWithFormat:@"doors-%02d", i]]) {
            [self failSmoke:@"missing door assets"];
        }
    }
    [self showSettings:nil];
    if (!_settingsWindow.isVisible) [self failSmoke:@"settings did not open"];
    [_shortcutLetter selectItemAtIndex:'K' - 'A'];
    _shortcutShift.state = NSControlStateValueOn;
    if (![self applyShortcutSelection] || [_settings[@"hotkey_letter"] intValue] != 'K' ||
        ![_settings[@"hotkey_shift"] boolValue]) {
        [self failSmoke:@"customized shortcut did not register or save"];
    }
    [self sendBehind:nil];
    EventHotKeyID custom = {HotkeySignature, _hotkeyIdentifier};
    if (![self handleHotkey:custom] || _panel.level != NSFloatingWindowLevel) {
        [self failSmoke:@"shortcut did not bring the clock forward"];
    }
    SMAppService *login = SMAppService.mainApp;
    SMAppServiceStatus before = login.status;
    if (before == SMAppServiceStatusNotRegistered || before == SMAppServiceStatusNotFound) {
        NSError *error = nil;
        BOOL registered = [login registerAndReturnError:&error];
        SMAppServiceStatus after = login.status;
        if (registered || after == SMAppServiceStatusEnabled || after == SMAppServiceStatusRequiresApproval) {
            [self refreshLoginItem];
            BOOL reflected = _loginItem.state == NSControlStateValueOn;
            if (![login unregisterAndReturnError:&error]) [self failSmoke:@"could not remove test login item"];
            [self refreshLoginItem];
            if (!reflected || _loginItem.state != NSControlStateValueOff) {
                [self failSmoke:@"login item status did not match Service Management"];
            }
            NSLog(@"KookieClock login-item registration and removal passed. Registered status: %ld.", (long)after);
        } else {
            NSLog(@"KookieClock login-item registration unavailable in this runner (status %ld): %@.", (long)after, error);
        }
    }
    [_settingsWindow close];
    [self showGallery:nil];
    [_friends selectItemAtIndex:7];
    [self selectFriend:nil];
    if (!_galleryWindow.isVisible || !_portrait.image || _friends.numberOfItems != 24) {
        [self failSmoke:@"visitor gallery did not display Dot"];
    }
    [_galleryWindow close];
    [self sendBehind:nil];
    if (![self captureSmoke:@"clock-macos.png"]) [self failSmoke:@"idle clock did not draw"];
    [self update:@{@"quiet": @NO, @"count_hour": @NO, @"hourly": @NO, @"half_hourly": @NO}];
    _smokeStart = NSProcessInfo.processInfo.systemUptime;
    kc_visit(_clock, 7, false, true);
}

- (void)checkSmokeTest {
    if (NSProcessInfo.processInfo.systemUptime - _smokeStart > 26) {
        [self failSmoke:@"visit timed out"];
    }
    if (_frame.generation) {
        if (!_panel.isVisible || _panel.level != NSFloatingWindowLevel) {
            [self failSmoke:@"scheduled clock did not float"];
        }
        if (_frame.door == 16) _smokeSawOpenDoor = YES;
        if (_frame.visitor == 7 && _frame.depth >= .99 && !_smokeSawVisitor) {
            _smokeSawVisitor = [self captureSmoke:@"clock-macos-visit.png"];
        }
        if (_smokeSawVisitor && _frame.visitor == 7 && _frame.sprite == 24 && _frame.lateral > 0) {
            _smokeSawReturn = YES;
        }
        if (_smokeSawReturn && _frame.visitor < 0 && _frame.door == 0) _smokeSawClosedDoor = YES;
        return;
    }
    if (!_smokeSawOpenDoor || !_smokeSawVisitor || !_smokeSawReturn || !_smokeSawClosedDoor ||
        !_smokeDecodedAudio || _panel.level == NSFloatingWindowLevel) {
        [self failSmoke:[NSString stringWithFormat:
            @"visit incomplete (opened=%d visitor=%d returned=%d closed=%d audio=%d floating=%d)",
            _smokeSawOpenDoor, _smokeSawVisitor, _smokeSawReturn, _smokeSawClosedDoor,
            _smokeDecodedAudio, _panel.level == NSFloatingWindowLevel]];
    }
    NSLog(@"KookieClock smoke test passed: settings, shortcut, cast, rendered visit, door timing, sound decode, desktop placement. Playback accepted: %@.",
          _smokePlayedAudio ? @"yes" : @"no audio device");
    [NSApp terminate:nil];
}

- (void)applicationWillTerminate:(NSNotification *)notification {
    [_timer invalidate];
    [self finishAudio];
    [NSWorkspace.sharedWorkspace.notificationCenter removeObserver:self];
    if (_hotkey) UnregisterEventHotKey(_hotkey);
    if (_hotkeyHandler) RemoveEventHandler(_hotkeyHandler);
    kc_destroy(_clock);
}
@end

int main(int argc, const char *argv[]) {
    @autoreleasepool {
        NSApplication *app = NSApplication.sharedApplication;
        [app setActivationPolicy:NSApplicationActivationPolicyAccessory];
        ClockApp *delegate = [ClockApp new];
        delegate.smokeTest = argc == 2 && strcmp(argv[1], "--smoke-test") == 0;
        app.delegate = delegate;
        [app run];
    }
    return 0;
}
