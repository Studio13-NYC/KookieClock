#import <AppKit/AppKit.h>
#import <AVFoundation/AVFoundation.h>
#import <Carbon/Carbon.h>

#import "ClockView.h"

static NSString *const SavedClockKey = @"SavedClock";
static const NSSize ClockSize = {400, 575};

@interface ClockPanel : NSPanel
@end
@implementation ClockPanel
- (BOOL)canBecomeKeyWindow { return NO; }
- (BOOL)canBecomeMainWindow { return NO; }
@end

@interface ClockApp : NSObject <NSApplicationDelegate, NSMenuDelegate, NSTextFieldDelegate>
@property(nonatomic) BOOL smokeTest;
- (void)showClock:(id)sender;
@end

static OSStatus HotkeyPressed(EventHandlerCallRef next, EventRef event, void *context) {
    if (GetEventKind(event) != kEventHotKeyPressed) return eventNotHandledErr;
    [(__bridge ClockApp *)context showClock:nil];
    return noErr;
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
    EventHotKeyRef _hotkey;
    EventHandlerRef _hotkeyHandler;
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
    if (!self.smokeTest) [self registerHotkey];
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

- (void)registerHotkey {
    EventTypeSpec event = {kEventClassKeyboard, kEventHotKeyPressed};
    OSStatus install = InstallEventHandler(GetApplicationEventTarget(), HotkeyPressed, 1, &event,
                                           (__bridge void *)self, &_hotkeyHandler);
    EventHotKeyID identifier = {0x4B4F4F4B, 1}; // KOOK
    OSStatus registration = install == noErr
        ? RegisterEventHotKey(kVK_ANSI_C, controlKey | optionKey, identifier,
                              GetApplicationEventTarget(), 0, &_hotkey)
        : install;
    _status.button.toolTip = registration == noErr
        ? @"KookieClock · Control–Option–C to bring forward"
        : @"KookieClock · shortcut unavailable. Use this menu to bring forward.";
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
    return WindowContent(@[form, columns, reset, Label(@"Control–Option–C brings the clock forward.")]);
}

- (void)refreshSettingsWindow {
    if (_settingsWindow.isVisible) _settingsWindow.contentView = [self settingsContent];
}

- (void)showSettings:(id)sender {
    if (!_settingsWindow.isVisible) {
        _settingsWindow = ControlsWindow(@"KookieClock Settings", [self settingsContent]);
    }
    [NSApp activateIgnoringOtherApps:YES];
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
    NSLog(@"KookieClock smoke test passed: settings, cast, rendered visit, door timing, sound decode, desktop placement. Playback accepted: %@.",
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
